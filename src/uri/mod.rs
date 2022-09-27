use std::fmt;
use std::str::{self, FromStr};
use std::num::ParseIntError;
use http::uri::{InvalidUri, Uri};

const VALID_SCHEMES: &'static [&str] = &["rtu", "tcp"];

/// An error resulting from a failed attempt to construct a URI.
#[derive(Debug, Clone)]
pub struct InvalidScheme {
    scheme: String,
}

impl fmt::Display for InvalidScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid scheme {}, expected one of {:?}", self.scheme, VALID_SCHEMES)
    }
}

#[derive(Debug)]
pub enum UriError {
    Scheme(InvalidScheme),
    Uri(InvalidUri),
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UriError::Scheme(scheme_error) => 
                write!(f, "{}", scheme_error),
            UriError::Uri(uri_error) => 
                write!(f, "{}", uri_error),
        }
    }
}

// Make it an error!
impl std::error::Error for UriError {}

impl From<InvalidScheme> for UriError {
    fn from(e: InvalidScheme) -> Self {
        UriError::Scheme(e)
    }
}

impl From<InvalidUri> for UriError {
    fn from(e: InvalidUri) -> Self {
        UriError::Uri(e)
    }
}

#[derive(Clone, Debug)]
pub struct ModbusUri {
    scheme: String,
    host: String,
    port: String,
}

impl ModbusUri {
    pub fn port_u32(&self) -> Result<u32, ParseIntError> {
        self.port.parse()
    }

    pub fn try_from(uri: http::Uri) -> Result<ModbusUri, InvalidScheme> {
        let scheme = uri.scheme_str().unwrap();
        if !VALID_SCHEMES.contains(&scheme) {
            return Err(InvalidScheme{scheme: scheme.to_owned()})
        }
        Ok(ModbusUri{scheme: uri.scheme_str().unwrap().to_string(), host: uri.host().unwrap().to_string(), port: uri.port().unwrap().to_string()})
    }
}


impl FromStr for ModbusUri {
    type Err = UriError;

    #[inline]
    fn from_str(s: &str) -> Result<ModbusUri, UriError> {
        let uri = Uri::try_from(s.as_bytes())?;

        let mb_uri = ModbusUri::try_from(uri)?;
        
        Ok(mb_uri)
    }
}