pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    /// Means that the operating system's graphical environment has behaved in an unexpected way
    PlatformApiFailed(&'static str),
    
    /// The part of a graphics API specific to the platform failed
    GraphicsApiFailed(&'static str),

    /// May be caused by memory allocation failures, mutex creation failures etc.
    InternalFailure,

    /// An argument passed to a function is invalid
    InvalidArgument,

    /// A subsystem is turned off globally
    SubsystemNotAvailable,

    /// The subsystem state cannot be changed
    SubsystemForced,

    /// You should specify a certain Cargo feature during compilation
    FeatureNotCompiled,
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlatformApiFailed(msg) => write!(f, "platform API failed ({msg})"),
            Self::GraphicsApiFailed(msg) => write!(f, "graphics API failed ({msg})"),
            Self::InternalFailure => write!(f, "internal logic failed"),
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::SubsystemNotAvailable => write!(f, "the subsystem is not available"),
            Self::SubsystemForced => write!(f, "the subsystem state is forced"),
            Self::FeatureNotCompiled => write!(f, "feature was not compiled"),
        }
    }
}