fn main() {
    println!("cargo::rustc-check-cfg=cfg(nightly)");

    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .expect("failed to run rustc");

    let version = String::from_utf8_lossy(&output.stdout);

    if version.contains("nightly") {
        println!("cargo:rustc-cfg=nightly");
    }
}
