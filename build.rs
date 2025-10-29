use cargo_toml::Manifest;
use std::path::Path;

fn main() {
    let manifest_path = Path::new("Cargo.toml");
    let manifest = Manifest::from_path(manifest_path).expect("failed to read Cargo.toml");
    let version = manifest.package.expect("no [package]").version;
    // Extract the inner string explicitly
    println!("cargo:rustc-env=CFA_VERSION=v{}", version.as_str());
}
