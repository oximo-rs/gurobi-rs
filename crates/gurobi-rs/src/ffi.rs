cfg_if::cfg_if! {
    if #[cfg(feature = "gurobi12")] {
        pub use guro_sys::*;

        pub mod shims {
            use super::*;
            pub unsafe fn empty_env(env: *mut *mut GRBenv) -> c_int{
                let (major, minor, technical) = crate::version();
                unsafe { GRBemptyenvinternal(env, major, minor, technical) }
            }


            pub unsafe fn load_env(env: *mut *mut GRBenv, logfilename: c_str) -> c_int{
                let (major, minor, technical) = crate::version();
                unsafe { GRBloadenvinternal(env, logfilename, major, minor, technical) }
            }
        }
    } else if #[cfg(any(feature = "gurobi11", feature = "gurobi10"))] {
        pub use guro_sys::*;

        pub mod shims {
            use super::*;

            pub unsafe fn empty_env(env: *mut *mut GRBenv) -> c_int{
                unsafe { GRBemptyenv(env) }
            }


            pub unsafe fn load_env(env: *mut *mut GRBenv, logfilename: c_str) -> c_int{
                unsafe { GRBloadenv(env, logfilename) }
            }
        }
    } else {
        compile_error!("bug: one of the above feature flags should have been hit.");
    }
}
