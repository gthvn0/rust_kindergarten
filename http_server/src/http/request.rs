use super::method::{Method, MethodError};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // First we get a slice of string from the array of bytes
        let request = str::from_utf8(buf)?;

        println!("request: {}", request);

        // Now we can extract the different part from the slice of string
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        println!("Checking protocol");
        // we only support HTTP 1.1
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // Now we want to transform the method to a Method
        let method: Method = method.parse()?;

        // Now we need to split the path and the query string.
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }

        Ok(Request {
            path: path.to_string(),
            query_string: match query_string {
                Some(q) => Some(q.to_string()),
                None => None
            },
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
