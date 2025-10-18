use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Capture(CaptureError),
    Save(SaveError),
    KeySend(KeySendError),
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Configuration error: {}", e),
            AppError::Capture(e) => write!(f, "Capture error: {}", e),
            AppError::Save(e) => write!(f, "Save error: {}", e),
            AppError::KeySend(e) => write!(f, "Key send error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError::Config(err)
    }
}

impl From<CaptureError> for AppError {
    fn from(err: CaptureError) -> Self {
        AppError::Capture(err)
    }
}

impl From<SaveError> for AppError {
    fn from(err: SaveError) -> Self {
        AppError::Save(err)
    }
}

impl From<KeySendError> for AppError {
    fn from(err: KeySendError) -> Self {
        AppError::KeySend(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidKey,
    InvalidIterationCount,
    DirectoryCreationFailed(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidKey => write!(f, "Invalid arrow key specified"),
            ConfigError::InvalidIterationCount => write!(f, "Invalid iteration count"),
            ConfigError::DirectoryCreationFailed(e) => {
                write!(f, "Failed to create directory: {}", e)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug)]
pub enum CaptureError {
    GetDCFailed,
    CreateCompatibleDCFailed,
    CreateBitmapFailed,
    BitBltFailed,
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureError::GetDCFailed => write!(f, "Failed to get device context"),
            CaptureError::CreateCompatibleDCFailed => {
                write!(f, "Failed to create compatible device context")
            }
            CaptureError::CreateBitmapFailed => write!(f, "Failed to create bitmap"),
            CaptureError::BitBltFailed => write!(f, "Failed to copy screen content"),
        }
    }
}

impl std::error::Error for CaptureError {}

#[derive(Debug)]
pub enum SaveError {
    GetDIBitsFailed,
    FileWriteFailed(std::io::Error),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::GetDIBitsFailed => write!(f, "Failed to get bitmap data"),
            SaveError::FileWriteFailed(e) => write!(f, "Failed to write file: {}", e),
        }
    }
}

impl std::error::Error for SaveError {}

#[derive(Debug)]
pub enum KeySendError {
    SendInputFailed,
}

impl fmt::Display for KeySendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeySendError::SendInputFailed => write!(f, "Failed to send keyboard input"),
        }
    }
}

impl std::error::Error for KeySendError {}
