
//! This module provides types necessary for error checking and debugging.

use std::fmt;
use std::result;
use std::error;
use std::io;
use std::char::CharTryFromError;
use std::string::FromUtf8Error;
use std::ffi::{FromBytesWithNulError, IntoStringError, NulError};
use std::sync::PoisonError;
use std::rc::Rc;

use api::Exception;

/// Generic julia-rs Result type, used pretty much everywhere a failure might occur
pub type Result<T> = result::Result<T, Error>;

/// A union of all possible errors that might occur in Julia runtime and
/// julia-rs, including Julia exceptions, Rust's io errors and alike, errors
/// arising from trying to use poisonend resources or trying to consume
/// resources in use.
#[derive(Debug)]
pub enum Error {
    /// An exception has occurred.
    UnhandledException(Exception),
    /// Cannot unbox into a certain type.
    InvalidUnbox,
    /// Tried to call a non-function object.
    NotAFunction,
    /// An error occurred while trying to call a function.
    CallError,
    /// An error occurred while evaluating a string or expression.
    EvalError,
    /// Attempt to construct a string or Julia object with a null pointer.
    NullPointer,
    /// Invalid characters used in symbol. See
    /// [docs.julialang.org](https://docs.julialang.org/en/stable/manual/variables/)
    /// for details on symbols and allowed characters.
    InvalidSymbol,
    /// Attempt to initialize Julia in a thread where it's already initialized.
    JuliaInitialized,
    /// Wrapper for ffi::FromBytesWithNulError.
    CStrError(FromBytesWithNulError),
    /// Wrapper for ffi::NulError.
    CStringError(NulError),
    /// Wrapper for sync::PoisonError.
    PoisonError,
    /// Wrapper for errors arising from trying to consume an Rc which is
    /// currently borrowed.
    ResourceInUse,
    /// Wrapper for char::CharTryFromError.
    UTF8Error(CharTryFromError),
    /// Wrapper for string::FromUtf8Error.
    FromUTF8Error(FromUtf8Error),
    /// Wrapper for ffi::IntoStringError.
    IntoStringError(IntoStringError),
    /// Wrapper for io::Error.
    IOError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnhandledException(ref ex) => write!(f, "UnhandledException({})", ex),
            Error::CStrError(ref err) => write!(f, "CStrError({})", err),
            Error::CStringError(ref err) => write!(f, "CStringError({})", err),
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
            Error::CStringError(ref err) => err.description(),
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
            Error::CStringError(ref err) => Some(err),
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

impl From<NulError> for Error {
    fn from(err: NulError) -> Error {
        Error::CStringError(err)
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
