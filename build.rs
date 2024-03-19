use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::process::Command;

fn main() {
    set_build_id();
}

fn set_build_id() {
    // Get the current git commit hash for the build id
    // If the command fails, generate a random string
    let build_id = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|output| String::from_utf8(output.stdout).ok())
        .ok()
        .flatten()
        .unwrap_or(format!(
            "dev-{}",
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
        ));

    println!("cargo:rustc-env=BUILD_ID={}", build_id);
}
