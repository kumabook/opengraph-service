use std;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use iron::prelude::*;
use iron::IronError;
use iron::status::Status;
use iron::mime::Mime;
use urlencoded;
use serde_json;
use opengraph;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    BadRequest,
    Unprocessable,
    NotFound,
    Unexpected,
}

impl Error {
    pub fn status(&self) -> Status {
        match *self {
            Error::BadRequest        => Status::BadRequest,
            Error::Unprocessable     => Status::UnprocessableEntity,
            Error::NotFound          => Status::NotFound,
            Error::Unexpected        => Status::InternalServerError,
        }
    }

    pub fn as_response(&self) -> Response {
        Response::with((self.status(),
                        Mime::from_str("application/json").ok().unwrap(),
                        serde_json::to_string(&self).unwrap()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::BadRequest            => write!(f, "BadRequest"),
            Error::Unprocessable         => write!(f, "Unproccesable"),
            Error::NotFound              => write!(f, "NotFound"),
            Error::Unexpected            => write!(f, "UnexpectedError"),
        }
    }
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        match err {
            Error::BadRequest        => IronError::new(err, Status::BadRequest),
            Error::Unprocessable     => IronError::new(err, Status::BadRequest),
            Error::NotFound          => IronError::new(err, Status::NotFound),
            Error::Unexpected        => IronError::new(err, Status::BadRequest),
        }
    }
}

impl From<urlencoded::UrlDecodingError> for Error {
    fn from(_: urlencoded::UrlDecodingError) -> Error {
        Error::BadRequest
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Error {
        Error::Unexpected
    }
}

impl From<opengraph::error::Error> for Error {
    fn from(_: opengraph::error::Error) -> Error {
        Error::Unexpected
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str { "" }
}
