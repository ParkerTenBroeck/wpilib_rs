use std::{env, path::PathBuf, str::FromStr};

use build_utils::{BuildConfig, Profile, Target};

extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let config = BuildConfig {
        profile: Profile::from_str(
            &std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_owned()),
        )
        .unwrap(),
        target: match env::var("WPILIB_TARGET").as_deref() {
            Ok("NATIVE") => Target::Native,
            Ok("ROBORIO") => Target::Roborio,
            Ok(val) => panic!("INVALID ENV $WPILIB_TARGET={val}"),
            Err(_) => {
                println!("cargo:warning=WPILIB_TARGET not set, defaulting to native");
                Target::Native
            }
        },
    };

    let file_name = format!("bindings_{}.rs", config.full_target_name());

    let out_file = out_path.join(file_name);

    if !out_file.exists() {
        let mut bindings = bindgen::Builder::default()
            .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
            .header("wrapper.hpp")
            .clang_arg("-xc++")
            .clang_arg("-std=c++20");

        // C/C++ libs

        match config.target {
            Target::Native => {
                let paths = if cfg!(target_os = "linux") {
                    "LINUX_CPP_LIB_PATH"
                } else if cfg!(target_os = "windows") {
                    "WINDOWS_CPP_LIB_PATH"
                } else if cfg!(target_os = "macos") {
                    "MACOS_CPP_LIB_PATH"
                } else {
                    panic!("Unknown target OS");
                };

                for path in env::var(paths)
                    .unwrap_or_else(|_| panic!("{paths} env not defined"))
                    .split(',')
                {
                    bindings = bindings.clang_arg(format!("-isystem{path}"))
                }
            }
            Target::Roborio => {
                // env::home_dir()
                let mut wpilib = build_utils::wpilib_path();
                wpilib.push(PathBuf::from_str("2023/roborio/arm-nilrt-linux-gnueabi/").unwrap());
                let toolchaindir = wpilib;

                let cppver = "12";

                let mut include1 = toolchaindir.clone();
                include1.push(
                    PathBuf::from_str(
                        format!("sysroot/usr/include/c++/{cppver}/arm-nilrt-linux-gnueabi")
                            .as_str(),
                    )
                    .unwrap(),
                );

                let mut include2 = toolchaindir.clone();
                include2.push(
                    PathBuf::from_str(
                        format!("sysroot/usr/include/c++/{cppver}/backward").as_str(),
                    )
                    .unwrap(),
                );

                let mut include3 = toolchaindir.clone();
                include3.push(
                    PathBuf::from_str(format!("sysroot/usr/include/c++/{cppver}").as_str())
                        .unwrap(),
                );

                let mut include4 = toolchaindir.clone();
                include4.push(PathBuf::from_str("sysroot/usr/include").unwrap());

                bindings = bindings
                    .clang_arg("--sysroot=".to_owned() + toolchaindir.to_str().unwrap())
                    .clang_arg("-isystem".to_owned() + include1.to_str().unwrap())
                    .clang_arg("-isystem".to_owned() + include2.to_str().unwrap())
                    .clang_arg("-isystem".to_owned() + include3.to_str().unwrap())
                    .clang_arg("-isystem".to_owned() + include4.to_str().unwrap())
            }
        }

        let path = build_utils::headers_path().to_str().unwrap().to_owned();
        // Library header include paths
        let bindings = bindings
            .clang_arg(format!("-I{path}/hal-cpp"))
            .clang_arg(format!("-I{path}/ntcore-cpp"))
            .clang_arg(format!("-I{path}/wpilibc-cpp"))
            .clang_arg(format!("-I{path}/wpimath-cpp"))
            .clang_arg(format!("-I{path}/wpiutil-cpp/fmt"))
            .clang_arg(format!("-I{path}/wpiutil-cpp"));

        let bindings = bindings
            .opaque_type("std::thread.*")
            .opaque_type("fmt.*")
            .opaque_type("std.*")
            .allowlist_type("frc.*")
            .allowlist_type("HAL.*")
            .allowlist_type("hal.*")
            .allowlist_type("nt.*")
            .allowlist_type("NT.*")
            // fixes strange mystical errors beyond me
            // .opaque_type(".*basic_ostream_sentry.*")
            // .opaque_type(".*basic_istream_sentry_traits_type.*")
            // .opaque_type(".*basic_istream_sentry___istream_type.*")
            // .opaque_type(".*basic_istream_sentry___streambuf_type.*")
            // .opaque_type(".*vector__Temporary_value__Storage.*")
            // .opaque_type(".*rep.*")
            .derive_debug(true)
            // .generate_inline_functions(true)
            .allowlist_function("HAL.*")
            .allowlist_function("frc.*")
            .allowlist_function("nt.*")
            .allowlist_function("NT.*")
            .allowlist_function("std.*")
            .allowlist_function("GetWPILibVersion")
            .enable_cxx_namespaces()
            .vtable_generation(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings!");

        bindings
            .write_to_file(out_file)
            .expect("Couldn't write bindings!");
    }
}
