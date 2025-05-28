fn main() {
    println!("cargo:rerun-if-changed=proto/feroxdb.proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/feroxdb.proto"], &["proto"])
        .expect("Failed to compile proto files");
}
