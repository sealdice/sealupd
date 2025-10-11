#[cfg(windows)]
fn main() {
    embed_manifest::embed_manifest(embed_manifest::new_manifest("Sealupd")).expect("create manifest");
    println!("cargo:rerun-if-changed=build.rs");

    static_vcruntime::metabuild();
}

#[cfg(not(windows))]
fn main() {}
