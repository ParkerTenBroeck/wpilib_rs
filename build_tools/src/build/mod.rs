use std::process::Command;

use build_utils::{libraries_path, BuildConfig, Profile, Target};

pub fn build(config: BuildConfig) {
    let BuildConfig { profile, target } = config;

    build_utils::maven::unpack_headers();
    build_utils::maven::unpack_libraries(config);

    let mut run_cmd = Command::new("cargo");
    run_cmd.current_dir(std::env::current_dir().unwrap());

    #[cfg(nightly)]
    {
        run_cmd.arg("+nightly");
    }

    run_cmd.arg("build");

    run_cmd.arg("--bin");
    run_cmd.arg(std::env::var("TARGET_BINARY").unwrap_or_else(|_| "robot_code".to_owned()));

    if profile == Profile::Release {
        run_cmd.arg("--release");
    }

    if target == Target::Roborio {
        run_cmd.arg("--target").arg(get_target_tripple(target));
    }

    run_cmd.env(
        "WPILIB_TARGET",
        match target {
            Target::Native => "NATIVE",
            Target::Roborio => "ROBORIO",
        },
    );
    let mut gcc = build_utils::wpilib_path();
    gcc.push("2023/roborio/bin/arm-frc2023-linux-gnueabi-gcc");
    let gcc = gcc.as_os_str().to_str().unwrap();

    let mut ar = build_utils::wpilib_path();
    ar.push("2023/roborio/arm-nilrt-linux-gnueabi/bin/ar");
    let ar = ar.as_os_str().to_str().unwrap();

    run_cmd.arg(format!(
        "--config=target.arm-unknown-linux-gnueabi.linker=\"{gcc}\""
    ));
    run_cmd.arg(format!(
        "--config=target.arm-unknown-linux-gnueabi.ar=\"{ar}\""
    ));

    let mut libs_path = libraries_path();

    libs_path.push(config.profile.as_str());

    let deps = build_utils::maven::load_deps();

    let mut libraries = "--config=target.'cfg(all())'.rustflags=[".to_owned();

    for lib in deps.local {
        if lib.no_native_libs && config.target == Target::Native {
            continue;
        }
        if lib.no_roborio_libs && config.target == Target::Roborio {
            continue;
        }

        let mut lib_path = libs_path.clone();
        lib_path.push(lib.artifact_id);
        lib_path.push(config.get_target_os());
        lib_path.push(config.get_target_arch());
        lib_path.push("shared");

        for file in std::fs::read_dir(lib_path).unwrap().flatten() {
            let path = file.path(); //file.file_name()
            let full_name = path.file_name().unwrap().to_str().unwrap();
            let prefix = full_name.split('.').next().unwrap();

            if file.file_type().unwrap().is_file() && prefix.starts_with("lib") {
                // let lib = prefix.trim_start_matches("lib");
                // lib.
                if full_name.contains(".so") && !full_name.ends_with("debug") {
                    // libraries.push_str(&format!("\"-C\", \"link-args=-Wl,--as-needed\","));
                    libraries.push_str(&format!(
                        "\"-C\", \"link-args=-Wl,--as-needed,{}\",",
                        path.as_os_str().to_str().unwrap()
                    ));
                }
            }
        }
    }

    // run_cmd.env("LIBRARIES", libraries);
    libraries.push(']');
    run_cmd.arg(libraries);

    assert!(run_cmd.status().unwrap().success());
}

fn get_target_tripple(target: Target) -> &'static str {
    match target {
        Target::Native => {
            //TODO
            // TARGET
            todo!()
            // "fdsfdsfdslj"
        }
        Target::Roborio => "arm-unknown-linux-gnueabi",
    }
}
