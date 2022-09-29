use std::fmt;
use std::str::FromStr;
use http::uri::{InvalidUri, Uri};

const VALID_SCHEMES: &'static [&str] = &["rtu", "tcp"];

/// The provided scheme was not a supported one
#[derive(Debug, Clone)]
pub struct InvalidScheme {
    scheme: String,
}

impl fmt::Display for InvalidScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid scheme {}, expected one of {:?}", self.scheme, VALID_SCHEMES)
    }
}

#[derive(Debug, Clone)]
pub struct MissingComponent {
    uri: String,
    missing: &'static str,
}

impl fmt::Display for MissingComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid uri {}, missing required component {}", self.uri, self.missing)
    }
}

#[derive(Debug)]
pub enum UriError {
    Scheme(InvalidScheme),
    Missing(MissingComponent),
    Uri(InvalidUri),
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UriError::Scheme(scheme_error) => 
                write!(f, "{}", scheme_error),
            UriError::Uri(uri_error) => 
                write!(f, "{}", uri_error),
            UriError::Missing(missing_error) =>
                write!(f, "{}", missing_error),
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

fn default_port_for_scheme<'a>(scheme: &'a str) -> u16 {
    match scheme {
        "rtu" => 9600,
        "tcp" => 502,
        _ => 0
    }
}

#[derive(Clone, Debug)]
pub struct ModbusUri {
    scheme: String,
    host: String,
    port: u16,
}

impl ModbusUri {
    pub fn try_from(uri: http::Uri) -> Result<ModbusUri, UriError> {
        let scheme = match uri.scheme_str() {
            Some(scheme) => scheme,
            None => return Err(UriError::Missing(MissingComponent{uri: uri.to_string(), missing: "scheme"}))
        };
        if !VALID_SCHEMES.contains(&scheme.to_ascii_lowercase().as_str()) {
            return Err(UriError::Scheme(InvalidScheme{scheme: scheme.to_owned()}))
        }

        let host = match uri.host() {
            Some(host) => host,
            None => return Err(UriError::Missing(MissingComponent{uri: uri.to_string(), missing: "host"}))
        };

        let port = match uri.port() {
            Some(port) => port.as_u16(),
            None => default_port_for_scheme(&scheme),
        };
        
        Ok(ModbusUri{scheme: scheme.to_string(), host: host.to_string(), port: port})
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