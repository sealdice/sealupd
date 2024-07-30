fn main() {
    if !cfg!(windows) {
        return;
    }

    embed_manifest::embed_manifest(embed_manifest::new_manifest("Sealupd"))
        .expect("failed to create manifest");
    println!("cargo:rerun-if-changed=build.rs");

    static_vcruntime::metabuild();
}
