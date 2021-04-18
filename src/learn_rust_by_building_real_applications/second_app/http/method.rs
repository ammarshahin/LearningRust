use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    HEAD,
}

impl FromStr for Method {
    type Err = MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
