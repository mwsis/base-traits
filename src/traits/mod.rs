// traits/mod.rs

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),*) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(as_f64, AsF64);
declare_and_publish!(as_str, AsStr);
declare_and_publish!(is_default, IsDefault);
declare_and_publish!(is_empty, IsEmpty);
declare_and_publish!(is_zero, IsZero);
declare_and_publish!(len, Len);
declare_and_publish!(to_f64, ToF64);
declare_and_publish!(zero, Zero);

mod sealed;
pub(crate) use sealed::Sealed;


// ///////////////////////////// end of file //////////////////////////// //