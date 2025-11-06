fn main() {
    println!("cargo:rerun-if-changed=puredoom_wrapper.c");

    cc::Build::new()
        .include(".")
        .flag_if_supported("--target=wasm32-unknown-unknown")
        .flag_if_supported("-nostdinc")
        .warnings(false)
        .file("puredoom_wrapper.c")
        .target("wasm32-unknown-unknown")
        .compile("puredoom");
}
