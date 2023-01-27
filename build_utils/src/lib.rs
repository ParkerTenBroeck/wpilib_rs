use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

pub mod maven;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Target {
    Native,
    Roborio,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Profile {
    Release,
    #[default]
    Debug,
}

impl Profile {
    pub fn as_str(&self) -> &'static str {
        match self {
            Profile::Release => "release",
            Profile::Debug => "debug",
        }
    }
}

impl FromStr for Profile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "release" => Ok(Self::Release),
            "debug" => Ok(Self::Debug),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BuildConfig {
    pub profile: Profile,
    pub target: Target,
}

impl BuildConfig {
    pub fn get_target_arch(&self) -> &'static str {
        match self.target {
            Target::Roborio => "athena",
            Target::Native => {
                if cfg!(target_arch = "x86_64") {
                    "x86-64"
                } else if cfg!(target_arch = "x86") {
                    "x86"
                } else {
                    "UNKNOWN"
                }
            }
        }
    }

    pub fn full_target_name(&self) -> String {
        format!("{}{}", self.get_target_os(), self.get_target_arch())
    }

    pub fn get_target_os(&self) -> &'static str {
        match self.target {
            Target::Roborio => "linux",
            Target::Native => {
                if cfg!(target_os = "linux") {
                    "linux"
                } else if cfg!(target_os = "windows") {
                    "windows"
                } else if cfg!(target_os = "macos") {
                    "macos"
                } else {
                    "UNKNOWN"
                }
            }
        }
    }

    pub fn get_target_profile(&self) -> &'static str {
        self.profile.as_str()
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            profile: Profile::default(),
            target: Target::Roborio,
        }
    }
}

pub fn wpilib_path() -> PathBuf {
    let env = match env::var("WPILIB_PATH") {
        Ok(ok) => ok,
        Err(_) => "~/wpilib".to_owned(),
    };

    if env.starts_with('~') {
        let mut home = home::home_dir()
            .expect("Home dir not found, try setting WPILIB_PATH to a absolute path");
        let env = env.strip_prefix('~').unwrap();
        let env = env.trim_start_matches('\\');
        let env = env.trim_start_matches('/');
        let path_from_home = PathBuf::from_str(env).unwrap_or_else(|_| {
            panic!(
                "Invalid WPILIB_PATH: {}",
                env::var("WPILIB_PATH").unwrap_or_else(|_| "UNDEFINED".to_owned())
            )
        });
        home.push(path_from_home);
        home
    } else {
        PathBuf::from_str(&env).unwrap_or_else(|_| {
            panic!(
                "Invalid WPILIB_PATH: {}",
                env::var("WPILIB_PATH").unwrap_or_else(|_| "UNDEFINED".to_owned())
            )
        })
    }
}

pub fn get_local_maven_path() -> PathBuf {
    let env = match env::var("LOCAL_MAVEN_PATH") {
        Ok(ok) => ok,
        Err(_) => "~/wpilib/2023/maven".to_owned(),
    };

    if env.starts_with('~') {
        let mut home = home::home_dir()
            .expect("Home dir not found, try setting LOCAL_MAVEN_PATH to a absolute path");
        let env = env.strip_prefix('~').unwrap();
        let env = env.trim_start_matches('\\');
        let env = env.trim_start_matches('/');
        let path_from_home = PathBuf::from_str(env).unwrap_or_else(|_| {
            panic!(
                "Invalid LOCAL_MAVEN_PATH: {}",
                env::var("LOCAL_MAVEN_PATH").unwrap_or_else(|_| "UNDEFINED".to_owned())
            )
        });
        home.push(path_from_home);
        home
    } else {
        PathBuf::from_str(&env).unwrap_or_else(|_| {
            panic!(
                "Invalid LOCAL_MAVEN_PATH: {}",
                env::var("LOCAL_MAVEN_PATH").unwrap_or_else(|_| "UNDEFINED".to_owned())
            )
        })
    }
}

pub fn workspace_path() -> PathBuf {
    let mut run_cmd = Command::new("cargo");
    run_cmd.arg("locate-project");
    run_cmd.arg("--message-format");
    run_cmd.arg("plain");
    run_cmd.arg("--workspace");

    let out = run_cmd.output().unwrap();
    assert!(out.status.success());
    let path = std::str::from_utf8(&out.stdout).unwrap();
    let path = path.trim();
    let path = path.split('\n').last().unwrap();
    let mut path = PathBuf::from_str(path).unwrap();
    path.pop();
    path
}

pub fn headers_path() -> PathBuf {
    let target = format!("{}/headers", std::env::var("TARGET_DIR").unwrap());
    PathBuf::from_str(&target).unwrap()
}

pub fn libraries_path() -> PathBuf {
    let target = format!("{}/libraries", std::env::var("TARGET_DIR").unwrap());
    PathBuf::from_str(&target).unwrap()
}

pub fn extract_archive(archive: impl AsRef<Path>, dest: impl AsRef<Path>) {
    let archive_file =
        fs::File::open(&archive).unwrap_or_else(|_| panic!("File: {}", archive.as_ref().display()));
    let archive_meta = archive_file.metadata();

    let mut archive = zip::ZipArchive::new(archive_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, dest.as_ref().display());
            fs::create_dir_all(&dest).unwrap();
        } else {
            let mut dest = dest.as_ref().to_owned();
            dest.push(file.name());

            let mut old = true;
            if let Ok(file) = fs::File::open(&dest) {
                if let (Ok(file_meta), Ok(archive_meta)) = (file.metadata(), &archive_meta) {
                    if let (Ok(file_mod), Ok(archive_mod)) =
                        (file_meta.modified(), archive_meta.modified())
                    {
                        if archive_mod <= file_mod {
                            old = false;
                        }
                    }
                }
            }

            if old {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    file.name(),
                    dest.display(),
                    file.size()
                );
            } else {
                println!("File {} upto date", file.name());
            }
            if let Some(p) = dest.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }

            if old {
                let mut outfile = fs::File::create(dest).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }
}
