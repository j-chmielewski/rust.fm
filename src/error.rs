use std::fmt;

#[derive(Debug, Clone)]
pub enum RustFmError {
    NoDevicesFound,
    SdrError
}

impl fmt::Display for RustFmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustFmError::NoDevicesFound => write!(f, "No devices found"),
            RustFmError::SdrError => write!(f, "SDR operation error")
        }
    }
}

impl std::error::Error for RustFmError {}

impl From<rtlsdr_mt::Error> for RustFmError {
    fn from(err: rtlsdr_mt::Error) -> Self {
        Self::SdrError
    }
}
