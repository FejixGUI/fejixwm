pub type Result<T> = std::result::Result<T, Error>;


pub enum Error {
    /// Means that a certain feature is not provided or not implemented on the platform
    Unsupported,

    /// Means that the operating system's graphical environment has behaved in an unexpected way
    PlatformApiFailed,
    
    /// May be caused when a certain graphics API is not supported
    GraphicsApiFailed,

    /// May be caused by memory allocation failures, mutex creation failures etc.
    InternalLogicFailed,
}