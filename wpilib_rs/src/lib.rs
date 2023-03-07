#[allow(clippy::all)]
#[allow(warnings)]
#[cfg(feature = "regen_bindings")]
#[cfg(not(feature = "export_bindings"))]
mod bindings_t {
    #[cfg(not(feature = "simulation"))]
    include!(concat!(env!("OUT_DIR"), "/bindings_roborio.rs"));

    #[cfg(feature = "simulation")]
    include!(concat!(env!("OUT_DIR"), "/bindings_native.rs"));
}

#[allow(clippy::all)]
#[allow(warnings)]
#[cfg(feature = "regen_bindings")]
#[cfg(feature = "export_bindings")]
pub mod bindings_t {
    #[cfg(not(feature = "simulation"))]
    include!(concat!(env!("OUT_DIR"), "/bindings_roborio.rs"));

    #[cfg(feature = "simulation")]
    include!(concat!(env!("OUT_DIR"), "/bindings_native.rs"));
}

#[cfg(not(feature = "regen_bindings"))]
#[cfg(not(feature = "simulation"))]
#[allow(clippy::all)]
#[allow(warnings)]
mod bindings_roborio;

#[cfg(not(feature = "regen_bindings"))]
#[cfg(feature = "simulation")]
#[allow(clippy::all)]
#[allow(warnings)]
mod bindings_native;

#[allow(clippy::all)]
#[allow(warnings)]
#[cfg(not(feature = "regen_bindings"))]
#[cfg(not(feature = "export_bindings"))]
mod bindings {
    #[cfg(feature = "simulation")]
    pub use super::bindings_native::*;
    #[cfg(not(feature = "simulation"))]
    pub use super::bindings_roborio::*;
}
#[allow(clippy::all)]
#[allow(warnings)]
#[cfg(not(feature = "regen_bindings"))]
#[cfg(feature = "export_bindings")]
pub mod bindings {
    #[cfg(feature = "simulation")]
    pub use super::bindings_native::*;
    #[cfg(not(feature = "simulation"))]
    pub use super::bindings_roborio::*;
}

#[cfg(feature = "export_bindings")]
pub mod cxx_ffi;
#[cfg(not(feature = "export_bindings"))]
mod cxx_ffi;

pub mod frc;
pub mod hal;
pub mod robot_base;
pub mod timed_robot;
