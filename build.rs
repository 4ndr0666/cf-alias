use cargo_toml::Manifest;
use std::path::Path;

fn main() {
    let manifest = Manifest::from_path(Path::new("Cargo.toml")).expect("failed to read Cargo.toml");
    let package = manifest.package.expect("missing [package]");
    let version = match package.version {
        cargo_toml::Inheritable::Inherit(_) => "unknown".to_string(),
        cargo_toml::Inheritable::Set(v) => v,
    };
    println!("cargo:rustc-env=CFA_VERSION=v{}", version);
}
