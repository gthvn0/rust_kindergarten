use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    PATCH,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => return Ok(Method::GET),
            "POST" => return Ok(Method::POST),
            "PUT" => return Ok(Method::PUT),
            "HEAD" => return Ok(Method::HEAD),
            "DELETE" => return Ok(Method::DELETE),
            "PATCH" => return Ok(Method::PATCH),
            "OPTIONS" => return Ok(Method::OPTIONS),
            "CONNECT" => return Ok(Method::CONNECT),
            "TRACE" => return Ok(Method::TRACE),
            _ => return Err(MethodError),
        }
    }
}

pub struct MethodError;
