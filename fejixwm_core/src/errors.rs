pub type Result<T> = std::result::Result<T, Error>;


pub enum Error {
    /// Means that a certain feature is not provided or not implemented on the platform
    Unsupported,

    /// Means that the operating system's graphical environment has behaved in an unexpected way
    PlatformApiFailed(&'static str),
    
    /// May be caused when a certain graphics API is not supported
    GraphicsApiFailed(&'static str),

    /// May be caused by memory allocation failures, mutex creation failures etc.
    InternalLogicFailed,
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported => write!(f, "feature is unsupported"),
            Self::PlatformApiFailed(msg) => write!(f, "platform API failed ({msg})"),
            Self::GraphicsApiFailed(msg) => write!(f, "graphics API failed ({msg})"),
            Self::InternalLogicFailed => write!(f, "internal logic failed"),
        }
    }
}