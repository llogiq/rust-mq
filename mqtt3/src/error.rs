use std::result;
use std::io;
use std::string::FromUtf8Error;
use byteorder;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IncorrectPacketFormat,
    InvalidTopicPath,
    UnsupportedProtocolName,
    UnsupportedProtocolVersion,
    UnsupportedQualityOfService,
    UnsupportedPacketType,
    UnsupportedConnectReturnCode,
    PayloadSizeIncorrect,
    PayloadTooLong,
    PayloadRequired,
    TopicNameMustNotContainNonUtf8,
    TopicNameMustNotContainWildcard,
    MalformedRemainingLength,
    UnexpectedEof,
    Io(io::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        Error::TopicNameMustNotContainNonUtf8
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::UnexpectedEOF => Error::UnexpectedEof,
            byteorder::Error::Io(err) => Error::Io(err)
        }
    }
}
