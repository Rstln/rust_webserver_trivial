use crate::http::{method, request};

use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Result as FmtResult, Formatter};
use std::str;
use std::str::Utf8Error;


// name of lifetime: buf
#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// get member element
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}



impl <'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    
    // GET / HTTP/1.1

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?; 

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protcol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protcol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method, 
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' '  || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
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

impl Error for ParseError {

}
 
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidRequest => "Invalid Request",
        }
    }
}

// a from trait
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}


// every time get Utf8Error, return a InvalidEncoding error
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}


impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        unimplemented!()
    }
}


// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }