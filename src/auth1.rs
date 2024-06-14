use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::tokio::task;
use crate::mongo::*;

pub struct TokenAuth(String);

impl TokenAuth {

    fn from_authorization_header(header: &str) -> Option<TokenAuth> {
        let token = header.to_lowercase();
        Some(TokenAuth(token))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenAuth {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        let auth_request_token = Self::from_authorization_header(auth_header.unwrap());
        let auth_request_token = auth_request_token.unwrap();
        let token = auth_request_token.0;

        if is_valid_token(token.clone().to_string()).await {
            Outcome::Success(TokenAuth(token))
        }else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}