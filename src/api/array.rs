
use sys::*;

jlvalues! {
    pub struct Array(jl_array_t);
    pub struct Svec(jl_svec_t);
}

#[macro_export]
macro_rules! jlvec {
    [] => {
        {
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let raw = unsafe { $crate::sys::jl_svec(0) };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    };
    [$elem:expr] => {
        {
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let elem = $crate::api::Value::from($elem).into_inner()?;
                let raw = unsafe {
                    $crate::sys::jl_svec1(elem as *mut _)
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    };
    [$elem1:expr, $elem2:expr] => {
        {
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let elem1 = $crate::api::Value::from($elem1).into_inner()?;
                let elem2 = $crate::api::Value::from($elem2).into_inner()?;
                let raw = unsafe {
                    $crate::sys::jl_svec2(elem1 as *mut _, elem2 as *mut _)
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    };
    [$( $elem:expr ),+] => {
        {
            use $crate::api::JlValue;
            let mut count = 0;
            #[allow(unknown_lints)]
            #[allow(no_effect)]
            {
                $(
                    || $elem;
                    count += 1;
                )+
            }

            fn svec(count: usize) -> $crate::error::Result<$crate::api::Svec> {
                let raw = unsafe {
                    $crate::sys::jl_svec(count,
                                         $(
                                             $crate::api::Value::from($elem).into_inner()?
                                         ),+
                                         )
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec(count)
        }
    };
    [$elem:expr; $n:expr] => {
        {
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let elem = $crate::api::Value::from($elem).into_inner()?;
                let raw = unsafe {
                    $crate::sys::jl_svec_fill($n, elem)
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    }
}
