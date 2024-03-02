use std::{fs, process::Command};

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("git rev-parse HEAD failed");

    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_ID={}", git_hash);

    // Only run npm install and npm build in release mode
    if "release" == std::env::var("PROFILE").expect("Could not get PROFILE") {
        println!("cargo:rerun-if-changed=resources");
        println!("cargo:rerun-if-changed=package.json");
        println!("cargo:rerun-if-changed=package-lock.json");
        println!("cargo:rerun-if-changed=vite.config.js");

        if fs::metadata("public/build").is_ok() {
            fs::remove_dir_all("public/build").expect("Could not remove public/build directory");
        }

        let npm_install = Command::new("npm")
            .args(&["install"])
            .output()
            .expect("npm install failed");
        if !npm_install.status.success() {
            panic!(
                "npm install failed: {}",
                String::from_utf8(npm_install.stderr)
                    .expect("Could not get error message from npm install")
            );
        }

        let npm_build = Command::new("npm")
            .args(&["run", "build"])
            .output()
            .expect("npm build failed");
        if !npm_build.status.success() {
            panic!(
                "npm build failed: {}",
                String::from_utf8(npm_build.stderr)
                    .expect("Could not get error message from npm build")
            );
        }
    }
}
