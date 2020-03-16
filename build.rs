//! [ISO 20022] Universal Financial Message Scheme builder
//!
//! # Usage
//!
//! Enable build script through `Cargo.toml` by removing the
//! `build = false` from the package section.
//!
//! [iso 20022]: https://www.iso20022.org/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .out_dir("./src/")
        .compile(
            &[
                "proto/head.001.001.02.proto",
                "proto/pain.001.001.10.proto",
                "proto/pain.002.001.11.proto",
                "proto/pain.007.001.10.proto",
                "proto/pain.008.001.09.proto",
                "proto/pain.013.001.08.proto",
                "proto/pain.014.001.08.proto",
            ],
            &["proto"],
        )?)
}
