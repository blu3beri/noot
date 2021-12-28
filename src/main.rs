//! The simplest stupidest node version manager that is very opinionated. It is used as a personal
//! project and therefore might not fit your use case.
//!
//! Installs everything at "$HOME/.config/noot/" and this is not changable

use std::env;
use std::env::consts::ARCH;

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
    ///     - darwin-arm64
    pub architecture: String,
}

/// Main command runner functionality
pub trait Manager {
    /// Log info about the command runner
    fn info(&self);

    /// Validate a command from the cli
    fn validate(&self, args: Vec<String>);

    /// List all the available node versions (and display which installed)
    fn list(&self);

    /// Add a node version (DOES NOT SET IT)
    fn add(&self, version: String);

    /// Set a node version (DOES NOT INSTALL IT)
    fn set(&self, version: String);

    /// Removes a node version
    /// TODO: How to handle removal of set node version?
    ///       We could panic and make it impossible
    ///       or even better, set the latest version when the
    ///       set version is deleted
    fn remove(&self, version: String);
}

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
            }
            "remove" => {
                assert!(
                    args.len() == 2,
                    "Please supply 1 and only 1 version to remove"
                );
            }
            _ => panic!("USAGE: <INFO | ADD | SET | LIST | REMOVE>"),
        }
    }

    fn list(&self) {
        println!("List");
    }

    fn add(&self, version: String) {
        println!(
            "Add: {}\nFrom: {}\nArch: {}",
            version, self.remote, self.architecture
        );
    }

    fn set(&self, version: String) {
        println!("Set: {}", version);
    }

    fn remove(&self, version: String) {
        println!("Remove: {}", version);
    }
}

/// CLI entrypoint
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Must match one of these architectures
    let arch = match ARCH {
        "aarch64" => "darwin-arm64",
        _ => panic!("UNSUPPORTED ARCHITECTURE"),
    };

    let coordinator = Coordinator {
        path: "~/.config/noot".to_owned(),
        installed: vec![],
        remote: "https://nodejs.org/dist/".to_owned(),
        architecture: arch.to_owned(),
    };

    coordinator.validate(args.to_owned());

    match &*args[0] {
        "info" => coordinator.info(),
        "add" => coordinator.add(args[1].to_owned()),
        "set" => coordinator.set(args[1].to_owned()),
        "remove" => coordinator.remove(args[1].to_owned()),
        "list" => coordinator.list(),
        _ => panic!("USAGE: <INFO | ADD | SET | LIST | REMOVE>"),
    }
}
