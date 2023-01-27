#[allow(clippy::all)]
#[allow(warnings)]
mod bindings_t {
    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    include!(concat!(env!("OUT_DIR"), "/bindings_linuxathena.rs"));

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    include!(concat!(env!("OUT_DIR"), "/bindings_linuxx86-64.rs"));

    #[cfg(target_os = "windows")]
    include!(concat!(env!("OUT_DIR"), "/bindings_windowsx86-64.rs"));

    #[cfg(target_os = "macos")]
    include!(concat!(env!("OUT_DIR"), "/bindings_macosx86-64.rs"));
}

#[cfg(feature = "export_bindings")]
pub use bindings_t::root as bindings;
#[cfg(not(feature = "export_bindings"))]
use bindings_t::root as bindings;

#[cfg(feature = "export_bindings")]
pub mod cxx_ffi;
#[cfg(not(feature = "export_bindings"))]
mod cxx_ffi;

pub mod frc;
pub mod hal;
pub mod robot_base;
pub mod timed_robot;
