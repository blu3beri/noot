//! The simplest stupidest node version manager that is very opinionated. It is used as a personal
//! project and therefore might not fit your use case.
//!
//! Installs everything at "$HOME/.config/noot/" and this is not configurable

use async_trait::async_trait;
use flate2::bufread::GzDecoder;
use fs_extra::dir;
use std::env::consts::ARCH;
use std::fs::create_dir;
use std::io::Cursor;
use std::path::Path;
use std::{env, fs};
use tar::Archive;

/// Main command runner
struct Coordinator {
    /// Where the node versions are installed
    pub path: String,

    /// Which versions are installed
    pub installed: Vec<String>,

    /// Remote url to fetch the node versions from
    pub remote: String,

    /// OS architecture required for installing the correct version
    /// Supported:
    ///   - darwin-arm64
    pub architecture: String,
}

/// Main command runner functionality
#[async_trait]
pub trait Manager {
    /// Log info about the command runner
    fn info(&self);

    /// Validate a command from the cli
    fn validate(&self, args: Vec<String>);

    /// List all the available node versions (and display which installed)
    async fn list(&self);

    /// Add a node version (DOES NOT SET IT)
    async fn add(&self, version: String);

    /// Set a node version (DOES NOT INSTALL IT)
    async fn set(&self, version: String);

    /// Removes a node version
    /// TODO: How to handle removal of set node version?
    ///       We could panic and make it impossible
    ///       or even better, set the latest version when the
    ///       set version is deleted
    fn remove(&self, version: String);
}

#[async_trait]
impl Manager for Coordinator {
    fn info(&self) {
        println!("Path: {} \nInstalled: {:?}", self.path, self.installed);
    }

    fn validate(&self, args: Vec<String>) {
        match &*args[0] {
            "add" => {
                assert!(
                    args.len() == 2,
                    "Please supply 1 and only 1 version to install"
                );
            }
            "set" => {
                assert!(args.len() == 2, "Please supply 1 and only 1 version to set");

                let node_path = format!("{}node-v{}-{}", &self.path, &args[1], &self.architecture);
                let does_path_exist = Path::exists(Path::new(&node_path));
                assert!(does_path_exist, "Node version does not exist");
            }
            "remove" => {
                assert!(
                    args.len() == 2,
                    "Please supply 1 and only 1 version to remove"
                );
                let node_path = format!("{}node-v{}-{}", &self.path, &args[1], &self.architecture);
                let does_path_exist = Path::exists(Path::new(&node_path));
                assert!(does_path_exist, "Node version does not exist");
            }
            "rc" => {}
            _ => panic!("USAGE: <INFO | ADD | SET | LIST | REMOVE>"),
        }
    }

    async fn list(&self) {
        println!("List");
    }

    // TODO: Add statusbar to downloading and unpacking
    async fn add(&self, version: String) {
        let url = format!(
            "{}v{}/node-v{}-{}.tar.gz",
            &self.remote, &version, &version, &self.architecture
        );

        println!("Downloading node version: {}", version);
        let res = reqwest::get(url).await.unwrap();
        let bytes = res.bytes().await.unwrap();
        let content = Cursor::new(bytes);
        let tar = GzDecoder::new(content);
        let mut archive = Archive::new(tar);

        println!("Unpacking...");
        archive.unpack(&self.path).unwrap();
    }

    async fn set(&self, version: String) {
        let bins = vec!["node", "npm", "npx"];
        let using_path = format!("{}using", &self.path);
        let node_path = format!("{}node-v{}-{}", &self.path, &version, &self.architecture);

        let mut options = dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;

        bins.iter().for_each(|x| {
            let p1 = format!("{}/bin/{}", node_path, x);
            let p2 = format!("{}/{}", using_path, x);
            let _ = std::os::unix::fs::symlink(p1, p2);
        })
    }

    fn remove(&self, version: String) {
        let node_path = format!("{}node-v{}-{}", &self.path, &version, &self.architecture);
        match fs::remove_dir_all(node_path) {
            Ok(_) => println!("removed: {}", version),
            Err(_) => println!("Could not delete: {}", version),
        }
    }
}

/// CLI entrypoint
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    // https://nodejs.org/dist/v16.11.0/node-v16.11.0-darwin-arm64.tar.gz

    // Must match one of these architectures
    let arch = match ARCH {
        "aarch64" => "darwin-arm64",
        _ => panic!("{}: UNSUPPORTED ARCHITECTURE", ARCH),
    };

    let home = env::var("HOME").unwrap();
    let path = format!("{}/.config/noot/", home);
    let _ = create_dir(&path);
    let _ = create_dir(format!("{}{}", &path, "using"));
    let coordinator = Coordinator {
        path,
        installed: vec![],
        remote: "https://nodejs.org/dist/".to_owned(),
        architecture: arch.to_owned(),
    };

    coordinator.validate(args.to_owned());

    match &*args[0] {
        "info" => coordinator.info(),
        "list" => coordinator.list().await,
        "add" => coordinator.add(args[1].to_owned()).await,
        "set" => coordinator.set(args[1].to_owned()).await,
        "remove" => coordinator.remove(args[1].to_owned()),
        _ => panic!("USAGE: <RC | INFO | ADD | SET | LIST | REMOVE>"),
    }
}
