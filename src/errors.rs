use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::error;
use std::fmt;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ApiError {
    reason: &'static str,
    status: i16,
}

impl ApiError {
    pub fn new(reason: &'static str, status: i16) -> ApiError {
        ApiError { reason, status }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        self.reason
    }
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .status(Status::raw(500))
            .sized_body(Cursor::new(format!("{}", self)))
            .ok()
    }
}
