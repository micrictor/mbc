use std::fmt;
use std::str::FromStr;
use http::uri::{InvalidUri, Uri};

#[derive(Clone, Copy)]
pub enum Proto {
    Tcp,
    Rtu,
}

impl fmt::Debug for Proto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Proto::Tcp => write!(f, "tcp"),
            Proto::Rtu => write!(f, "rtu"),
        }
    }
}

impl FromStr for Proto {
    type Err = InvalidScheme;
    #[inline]
    fn from_str(scheme: &str) -> Result<Proto, Self::Err> {
        match scheme {
            "tcp" => Ok(Proto::Tcp),
            "rtu" => Ok(Proto::Rtu),
            _ => Err(InvalidScheme{scheme: scheme.to_string()})
        }

    }
}

/// The provided scheme was not a supported one
#[derive(Debug, Clone)]
pub struct InvalidScheme {
    scheme: String,
}

impl fmt::Display for InvalidScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid scheme {}, expected one of {:?}", self.scheme, [Proto::Rtu, Proto::Tcp])
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

fn default_port_for_proto(proto: Proto) -> u16 {
    match proto {
        Proto::Rtu => 9600,
        Proto::Tcp => 502,
    }
}

#[derive(Clone, Debug)]
pub struct ModbusUri {
    pub proto: Proto,
    pub host: String,
    pub port: u16,
}

impl ModbusUri {
    pub fn try_from<'a>(uri: http::Uri) -> Result<ModbusUri, UriError> {
        let scheme = match uri.scheme_str() {
            Some(scheme) => scheme,
            None => return Err(UriError::Missing(MissingComponent{uri: uri.to_string(), missing: "scheme"}))
        };
        let proto = match Proto::from_str(scheme) {
            Ok(proto) => proto,
            Err(e) => return Err(UriError::Scheme(e)), 
        };

        let host = match uri.host() {
            Some(host) => host,
            None => return Err(UriError::Missing(MissingComponent{uri: uri.to_string(), missing: "host"}))
        };

        let port = match uri.port() {
            Some(port) => port.as_u16(),
            None => default_port_for_proto(proto),
        };
        
        Ok(ModbusUri{proto: proto, host: host.to_string(), port: port})
    }
}


impl FromStr for ModbusUri {
    type Err = UriError;

    #[inline]
    fn from_str<'a>(s: &str) -> Result<ModbusUri, UriError> {
        let uri = Uri::try_from(s.as_bytes())?;

        ModbusUri::try_from(uri)
    }
}

impl fmt::Display for ModbusUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}://{}:{}", self.proto, self.host, self.port)
    }
}