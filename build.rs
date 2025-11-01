use cargo_toml::{Inheritable, Manifest};
use std::path::Path;

fn main() {
    let manifest = Manifest::from_path(Path::new("Cargo.toml")).expect("failed to read Cargo.toml");
    let package = manifest.package.expect("missing [package]");

    let version = match package.version {
        Inheritable::Inherited { workspace: _ } => "unknown".to_string(),
        Inheritable::Set(v) => v,
    };

    println!("cargo:rustc-env=CFA_VERSION=v{}", version);
}
