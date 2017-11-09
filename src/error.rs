
use std::fmt;
use std::result;
use std::error;
use std::io;
use std::char::CharTryFromError;
use std::string::FromUtf8Error;
use std::ffi::{FromBytesWithNulError, IntoStringError};
use std::sync::PoisonError;
use std::rc::Rc;

use api::Exception;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnhandledException(Exception),
    InvalidUnbox,
    NotAFunction,
    CallError,
    EvalError,
    NullPointer,
    InvalidSymbol,
    JuliaInitialized,
    CStrError(FromBytesWithNulError),
    PoisonError,
    ResourceInUse,
    UTF8Error(CharTryFromError),
    FromUTF8Error(FromUtf8Error),
    IntoStringError(IntoStringError),
    IOError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnhandledException(ref ex) => write!(f, "UnhandledException({})", ex),
            Error::CStrError(ref err) => write!(f, "CStrError({})", err),
            Error::UTF8Error(ref err) => write!(f, "UTF8Error({})", err),
            Error::FromUTF8Error(ref err) => write!(f, "FromUTF8Error({})", err),
            Error::IntoStringError(ref err) => write!(f, "IntoStringError({})", err),
            Error::IOError(ref err) => write!(f, "IOError({})", err),
            Error::InvalidUnbox | Error::NotAFunction | Error::CallError | Error::EvalError |
            Error::NullPointer | Error::InvalidSymbol | Error::JuliaInitialized |
            Error::PoisonError | Error::ResourceInUse => fmt::Debug::fmt(self, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnhandledException(ref ex) => ex.description(),
            Error::InvalidUnbox => "that Value cannot be unboxed into that Rust type",
            Error::NotAFunction => "this Value is not a Function",
            Error::CallError => "an error occurred while calling a Julia Function",
            Error::EvalError => "an error occurred while evaluating a Julia expression",
            Error::NullPointer => "the supplied raw pointer is a null pointer",
            Error::InvalidSymbol => "the symbol contains invalid characters",
            Error::JuliaInitialized => "Julia was already initialized",
            Error::CStrError(ref err) => err.description(),
            Error::PoisonError => "attempt to use a poisoned mutex",
            Error::ResourceInUse => "attempt to take ownership of a resource in use",
            Error::UTF8Error(ref err) => err.description(),
            Error::FromUTF8Error(ref err) => err.description(),
            Error::IntoStringError(ref err) => err.description(),
            Error::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::UnhandledException(ref ex) => Some(ex),
            Error::CStrError(ref err) => Some(err),
            Error::UTF8Error(ref err) => Some(err),
            Error::FromUTF8Error(ref err) => Some(err),
            Error::IntoStringError(ref err) => Some(err),
            Error::IOError(ref err) => Some(err),
            Error::InvalidUnbox | Error::NotAFunction | Error::CallError | Error::EvalError |
            Error::NullPointer | Error::InvalidSymbol | Error::JuliaInitialized |
            Error::PoisonError | Error::ResourceInUse => None,
        }
    }
}

impl From<FromBytesWithNulError> for Error {
    fn from(err: FromBytesWithNulError) -> Error {
        Error::CStrError(err)
    }
}

impl From<CharTryFromError> for Error {
    fn from(err: CharTryFromError) -> Error {
        Error::UTF8Error(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::FromUTF8Error(err)
    }
}

impl<G> From<PoisonError<G>> for Error {
    fn from(_err: PoisonError<G>) -> Error {
        Error::PoisonError
    }
}

impl<T> From<Rc<T>> for Error {
    fn from(_err: Rc<T>) -> Error {
        Error::ResourceInUse
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<IntoStringError> for Error {
    fn from(err: IntoStringError) -> Error {
        Error::IntoStringError(err)
    }
}
