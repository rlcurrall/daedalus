use std::{fs, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=resources");

    fs::remove_dir_all("public/build").expect("Could not remove public/build directory");

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

    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("git rev-parse HEAD failed");

    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_ID={}", git_hash);
}
