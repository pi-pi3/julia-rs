
/// Constructs a Julia function declaration in the form of a stringified Julia expression.
///
/// # Syntax
/// decl_jl! {
///     pub extern "Julia" fn <libname> :: <func> ( <arg: Type> * ) -> OutType;
/// }
///
/// # Example
/// ```
/// let square = decl_jl! {
///     pub extern "Julia" fn libsquare :: square(x: Float64) -> Float64;
/// };
/// "function square(x::Float64, )
///     ccall((:square, \"libsquare\"), Float64, (Float64,), x)
/// end"
/// ```
#[macro_export]
macro_rules! decl_jl {
    {
        pub extern "Julia" fn $lib:ident :: $name:ident ( $( $arg:ident : $type:ty ),* ) -> $out:ty;
    } => {
        concat!(
                "function ", stringify!($name), "(", $( stringify!($arg), "::", stringify!($type), ", ", )* ")\n",
                    "    ccall((:", stringify!($name), ", \"", stringify!($lib), "\"), ", stringify!($out),
                    ", (", $( stringify!($type), ",", )* "), ",  $( stringify!($arg), ",", )* ")\n",
                "end"
            )
    };
    {
        $(
            pub extern "Julia" fn $lib:ident :: $name:ident ( $( $arg:ident : $type:ty ),* ) -> $out:ty;
        )*
    } => {
        vec![
            $(
                decl_jl! { pub extern "Julia" fn $lib:ident :: $name:ident ( $( $arg:ident : $type:ty ),* ) -> $out:ty; }
            ),*
        ]
    }
}

/// Turns Rust functions into Julia extern functions.
///
/// # Syntax
/// ```
/// extern_jl! {
///     extern "Julia" <libname> :: <StructName> {
///         pub fn <func> ( <arg: Type> ) -> OutType {
///             <body>
///         }
///     }
/// }
///
/// # Example
/// ```
/// extern_jl! {
///     extern "Julia" libsquare :: Square {
///         pub fn square(x: Float64) -> Float64 {
///             x * x
///         }
///     }
/// }
///
/// fn main() {
///     let mut jl = Julia::new().unwrap()
///
///     let sqr = Square::new();
///     sqr.decl(&mut jl);
///
///     jl.eval_string("assert(square(5.0) == 25.0)");
/// }
/// ```
#[macro_export]
macro_rules! extern_jl {
    {
        extern "Julia" $lib:ident :: $struct:ident {
            $(
                pub fn $name:ident ( $( $arg:ident : $type:ty ),* ) -> $out:ty $body:block
            )*
        }
    } => {
        $(
            #[no_mangle]
            pub extern "C" fn $name ( $( $arg : $type ),* ) -> $out $body
        )*

        struct $struct {
            $(
                pub $name: &'static str,
            )*
        }

        impl $struct {
            pub fn new() -> $struct {
                $struct {
                    $(
                        $name: decl_jl! {
                            pub extern "Julia" fn $lib :: $name ( $( $arg : $type ),* ) -> $out;
                        },
                    )*
                }
            }

            pub fn decl(self, jl: &mut $crate::api::Julia) -> $crate::error::Result<$crate::api::Ref> {
                let mut decl = String::new();

                $(
                    decl.push_str(self.$name);
                )*
                jl.load(&mut decl.as_bytes(), Some(concat!( "jl-decl-", stringify!($lib), ".jl" )))
            }
        }
    }
}
