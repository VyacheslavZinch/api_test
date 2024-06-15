use futures::stream::TryStreamExt;
use mongodb::options::FindOneOptions;
use mongodb::{bson::doc, Collection, options::FindOptions};
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    env,
    thread::{self, JoinHandle},
};
use dotenvy::Error;
use mongodb::bson::oid::ObjectId;
use rocket::log::private::kv::Source;

#[derive(Debug, Serialize, Deserialize)]
struct Token(String);

pub async fn is_valid_token(token: &str) -> bool{
    dotenvy::dotenv().expect(".env error");

    let connection = match env::var("MONGO_URL") {
        Ok(_connection) => _connection,
        Err(_e) => {
            panic!("{}", _e)
        }
    };

    let client = Client::with_uri_str(&connection).await.expect("err");
    let db = client.database("api_access_tokens");
    let tokens_collections = db.collection("tokens");

    if doc_exists(&tokens_collections, token).await.expect("err") {
        return true;
    }else {
        return false;
    }

}

async fn doc_exists(collection: &Collection<Document>, id: &str) -> Result<bool, Error> {
    let object_id = ObjectId::parse_str(id.clone());
    let filter = doc! {"_id": id};
    let count = collection.count_documents(filter, None).await;

    Ok(count.unwrap() > 0)


}

struct Document(String);