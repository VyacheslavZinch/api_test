use std::env;

use mongodb::{bson::doc, sync::Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Token(String);

pub async fn is_valid_token(token: String) -> bool {
    dotenvy::dotenv().expect(".env error");

    let connection = match env::var("MONGO_URL") {
        Ok(_connection) => _connection,
        Err(_e) => {
            panic!("{}", _e)
        }
    };

    let client = Client::with_uri_str(&connection);
    let database = client.unwrap().database("api_access_tokens");
    let collection = database.collection::<Token>("_id");


    let result = match collection.find(doc! {"_id": token}, None) {
        Ok(value) => true,
        Err(_e) => false
    };

    result
}
