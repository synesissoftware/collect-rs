/// Simpler means to declare a module (`$mod_name`) and publish 1+ of its
/// elements (`$type_name`s), as in:
///
/// ```rust,ignore
/// // file src/my_module/mod.rs
/// use crate::macros::declare_and_publish;
///
/// declare_and_publish!(frequency_map, FrequencyMap, FrequencySet); // declares mod `frequency_map` and uses publicly types `FrequencyMap` and `FrequencySet` from it
/// ```
macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),*) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

pub(crate) use declare_and_publish;


// ///////////////////////////// end of file //////////////////////////// //
