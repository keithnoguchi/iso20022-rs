//! [ISO 20022] Universal Financial Message Scheme builder
//!
//! [iso 20022]: https://www.iso20022.org/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Re-run only when this file is changed.
    println!("cargo:rerun-if-changed=build.rs");
    // Install rustfmt which is used by tonic_build.
    std::process::Command::new("rustup").args(&["component", "add", "rustfmt"])
        .status().unwrap();
    Ok(tonic_build::configure()
        .build_client(false)
        .build_server(false)
        // enable below only when we need to update files.
        //.out_dir("./src/")
        .compile(
            &[
                "proto/head.001.001.02.proto",
                "proto/pain.001.001.10.proto",
                "proto/pain.002.001.11.proto",
            ],
            &["proto"],
        )?)
}
