
use std::result;
use std::ffi::FromBytesWithNulError;
use std::sync::PoisonError;
use std::rc::Rc;
use std::char::CharTryFromError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Hash)]
pub enum Error {
    CStrError,
    InvalidUnbox,
    NotAFunction,
    CallError,
    EvalError,
    NullValue,
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
