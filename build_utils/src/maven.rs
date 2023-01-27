use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{get_local_maven_path, BuildConfig, Profile, Target};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalArtifacts {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    #[serde(default)]
    pub no_headers: bool,
    #[serde(default)]
    pub no_native_libs: bool,
    #[serde(default)]
    pub no_roborio_libs: bool,
}

impl LocalArtifacts {
    pub fn get_artifact(&self, name: impl AsRef<str>) -> PathBuf {
        let mut repo_path = get_local_maven_path();
        for p in self.group_id.split('.') {
            repo_path.push(p);
        }
        repo_path.push(&self.artifact_id);
        repo_path.push(&self.version);
        repo_path.push(&format!(
            "{}-{}-{}",
            self.artifact_id,
            self.version,
            name.as_ref()
        ));
        repo_path
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MavenRepository {
    pub local: Vec<LocalArtifacts>,
}

pub fn load_deps() -> MavenRepository {
    let path = PathBuf::from_str(std::env::var("MAVEN_DEPS").unwrap().as_str()).unwrap();
    let file = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&file).unwrap()
}

pub fn unpack_headers() {
    let out_path = crate::headers_path();

    let maven = load_deps();

    for lib in maven.local {
        if lib.no_headers {
            continue;
        }
        let mut out_path = out_path.clone();
        out_path.push(&lib.artifact_id);

        let archive_path = lib.get_artifact("headers.zip");
        crate::extract_archive(archive_path, out_path);
    }
}

pub fn get_libs() -> Vec<PathBuf> {
    panic!();
}

pub fn get_header_paths() -> Vec<PathBuf> {
    panic!();
}

pub fn unpack_libraries(config: BuildConfig) {
    let maven = load_deps();

    let target = config.full_target_name();

    let mut out_path = crate::libraries_path();

    out_path.push(config.profile.as_str());

    for lib in maven.local {
        if lib.no_native_libs && config.target == Target::Native {
            continue;
        }
        if lib.no_roborio_libs && config.target == Target::Roborio {
            continue;
        }
        let mut out_path = out_path.clone();
        out_path.push(&lib.artifact_id);
        let archive_path = if config.profile == Profile::Release {
            lib.get_artifact(format!("{target}.zip"))
        } else {
            lib.get_artifact(format!("{target}debug.zip"))
        };
        crate::extract_archive(archive_path, out_path);
    }
}
