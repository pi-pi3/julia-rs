
use std::fmt;

pub trait Number {}

pub trait Real: Number {}

pub trait AbstractFloat: Number + Real {}
pub trait Integer: Number + Real {}

pub trait Signed: Number + Real + Integer {}
pub trait Unsigned: Number + Real + Integer {}

pub type Bool = bool;
impl Number for Bool {}
impl Real for Bool {}
impl Integer for Bool {}

pub type Char = char;

pub type Int8 = i8;
impl Number for Int8 {}
impl Real for Int8 {}
impl Integer for Int8 {}
impl Signed for Int8 {}

pub type Int16 = i16;
impl Number for Int16 {}
impl Real for Int16 {}
impl Integer for Int16 {}
impl Signed for Int16 {}

pub type Int32 = i32;
impl Number for Int32 {}
impl Real for Int32 {}
impl Integer for Int32 {}
impl Signed for Int32 {}

pub type Int64 = i64;
impl Number for Int64 {}
impl Real for Int64 {}
impl Integer for Int64 {}
impl Signed for Int64 {}

pub type Int = isize;
impl Number for Int {}
impl Real for Int {}
impl Integer for Int {}
impl Signed for Int {}

pub type UInt8 = u8;
impl Number for UInt8 {}
impl Real for UInt8 {}
impl Integer for UInt8 {}
impl Unsigned for UInt8 {}

pub type UInt16 = u16;
impl Number for UInt16 {}
impl Real for UInt16 {}
impl Integer for UInt16 {}
impl Unsigned for UInt16 {}

pub type UInt32 = u32;
impl Number for UInt32 {}
impl Real for UInt32 {}
impl Integer for UInt32 {}
impl Unsigned for UInt32 {}

pub type UInt64 = u64;
impl Number for UInt64 {}
impl Real for UInt64 {}
impl Integer for UInt64 {}
impl Unsigned for UInt64 {}

pub type UInt = usize;
impl Number for UInt {}
impl Real for UInt {}
impl Integer for UInt {}
impl Unsigned for UInt {}

pub type Float32 = f32;
impl Number for Float32 {}
impl Real for Float32 {}
impl AbstractFloat for Float32 {}

pub type Float64 = f64;
impl Number for Float64 {}
impl Real for Float64 {}
impl AbstractFloat for Float64 {}

#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Complex<T: Number> {
    pub a: T,
    pub b: T,
}

impl<T: Number + fmt::Debug> fmt::Debug for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} + {:?}im", self.a, self.b)
    }
}

impl<T: Number + fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}im", self.a, self.b)
    }
}

impl<T: Number> Number for Complex<T> {}

#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Rational<T: Number + Real + Integer> {
    pub num: T,
    pub den: T,
}

impl<T: Number + fmt::Debug + Real + Integer> fmt::Debug for Rational<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}//{:?}", self.num, self.den)
    }
}

impl<T: Number + fmt::Display + Real + Integer> fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}//{}", self.num, self.den)
    }
}

impl<T: Number + Real + Integer> Number for Rational<T> {}
impl<T: Number + Real + Integer> Real for Rational<T> {}
