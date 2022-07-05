use std::{env, path::PathBuf, process::Command};
// use wayland_scanner::{generate_client_code};

fn main() {
    if let Some(output) = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
    {
        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    }
    gio::compile_resources(
        "data/resources",
        "data/resources/resources.gresource.xml",
        "compiled.gresource",
    );
    // let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
    // // Location of the xml file, relative to the `Cargo.toml`
    // let ext_workspace_protocol_file = "data/resources/ext-workspace-unstable-v1.xml";
    // // Target directory for the generate files
    // generate_client_code!(
    //     ext_workspace_protocol_file,
    //     &dest.join("ext_workspace.rs"),
    // );
}