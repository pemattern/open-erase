fn main() {
    println!("cargo:rerun-if-changed=web/src/");
    println!("cargo:rerun-if-changed=web/index.html");
    println!("cargo:rerun-if-changed=web/styles.css");

    let _ = std::process::Command::new("trunk")
        .args(["build", "--release", "--config", "web/"])
        .output()
        .expect("Failed to run trunk build");
}
