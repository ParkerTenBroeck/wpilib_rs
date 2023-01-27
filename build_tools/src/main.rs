use std::borrow::Cow;

use build_utils::{BuildConfig, Profile, Target};

pub mod build;
pub mod deploy;
pub mod monitor;
pub mod util;

enum Config {
    None,
    Help,
    Build(BuildConfig),
    Deploy(BuildConfig, bool),
}

impl Config {
    pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Self, Cow<'static, str>> {
        let first = args.next();
        let first = if let Some(first) = first {
            first
        } else {
            return Ok(Config::None);
        };

        let mut conf_type = match first.as_str().trim() {
            "help" => Config::Help,
            "build" => Config::Build(BuildConfig::default()),
            "deploy" => Config::Deploy(BuildConfig::default(), true),
            stinky => return Err(format!("Unrecognized input: {stinky}").into()),
        };

        for arg in args {
            match &mut conf_type {
                Config::None => return Err(format!("Invalid argument: {arg}").into()),
                Config::Help => return Err(format!("Invalid argument for help: {arg}").into()),
                Config::Build(BuildConfig { profile, target }) => match arg.as_str().trim() {
                    "--release" => *profile = Profile::Release,
                    "--debug" => *profile = Profile::Debug,
                    "--native" => *target = Target::Native,
                    "--roborio" => *target = Target::Roborio,
                    stinky => return Err(format!("Invalid argument for build: {stinky}").into()),
                },
                Config::Deploy(build_kind, build) => match arg.as_str().trim() {
                    "--release" => build_kind.profile = Profile::Release,
                    "--debug" => build_kind.profile = Profile::Debug,
                    "--native" => build_kind.target = Target::Native,
                    "--roborio" => build_kind.target = Target::Roborio,
                    "--nobuild" => *build = false,
                    stinky => return Err(format!("Invalid argument for build: {stinky}").into()),
                },
            }
        }

        Ok(conf_type)
    }
}

fn main() {
    let args = std::env::args().skip(1).peekable(); // skip executable name

    let conf = Config::parse_args(args);
    let conf = match conf {
        Ok(ok) => ok,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    match conf {
        Config::None | Config::Help => help(),
        Config::Build(config) => build::build(config),
        Config::Deploy(config, rebuild) => {
            if rebuild {
                build::build(config);
            }
            deploy::deploy(config);
        }
    }
}

fn help() {
    println!("This isnt very helpful is it :/")
}
