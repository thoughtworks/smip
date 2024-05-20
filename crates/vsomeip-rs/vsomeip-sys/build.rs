pub fn main() -> miette::Result<()> {
    // cxx_build::bridge("src/lib.rs")
    // .include("src")
    // .std("c++17")
    // .compile("vsomeip-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/shim.hpp");

    let mut b = autocxx_build::Builder::new("src/lib.rs", ["vsomeip/vsomeip.hpp", "src"])
    .build()?;

    b.std("c++17")
    .cargo_warnings(false)
    .compile("vsomeip-sys");

    println!("cargo:rustc-link-lib=vsomeip3");

    Ok(())
}