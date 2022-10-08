fn main() {
    let dst = cmake::Config::new("vendor")
        .pic(true)
        .define("NAMIGATOR_BUILD_PYTHON", "FALSE")
        .define("STORM_USE_BUNDLED_LIBRARIES", "TRUE")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("NAMIGATOR_INSTALL_TESTS", "TRUE")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=libpathfind");
    println!("cargo:rustc-link-lib=static=libmapbuild");
    println!("cargo:rustc-link-lib=static=utility");
    println!("cargo:rustc-link-lib=static=parser");
    println!("cargo:rustc-link-lib=static=storm");
    println!("cargo:rustc-link-lib=static=Detour");
    println!("cargo:rustc-link-lib=static=Recast");

    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rerun-if-changed=vendor")
}
