
use std::result;
use std::ffi::FromBytesWithNulError;
use std::sync::PoisonError;
use std::rc::Rc;
use std::char::CharTryFromError;
use std::string::FromUtf8Error;

use exception::Exception;

pub type Result<T> = result::Result<T, Error>;

// TODO: Debug
#[derive(Clone)]
pub enum Error {
    UnhandledException(Exception),
    CStrError,
    InvalidUnbox,
    NotAFunction,
    CallError,
    EvalError,
    NullValue,
    NullPointer,
    PoisonError,
    ResourceInUse,
    UTF8Error,
    InvalidSymbol,
    UndefVar,
}

impl From<FromBytesWithNulError> for Error {
    fn from(_: FromBytesWithNulError) -> Error {
        Error::CStrError
    }
}

impl From<CharTryFromError> for Error {
    fn from(_: CharTryFromError) -> Error {
        Error::UTF8Error
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        Error::UTF8Error
    }
}

impl<G> From<PoisonError<G>> for Error {
    fn from(_: PoisonError<G>) -> Error {
        Error::PoisonError
    }
}

impl<T> From<Rc<T>> for Error {
    fn from(_: Rc<T>) -> Error {
        Error::ResourceInUse
    }
}
