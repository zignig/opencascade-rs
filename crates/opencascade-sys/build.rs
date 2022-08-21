fn main() {
    let dst = cmake::Config::new("OCCT").define("BUILD_LIBRARY_TYPE", "Static").build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=TKMath");
    println!("cargo:rustc-link-lib=static=TKernel");
    println!("cargo:rustc-link-lib=static=TKGeomBase");
    println!("cargo:rustc-link-lib=static=TKG3d");
    println!("cargo:rustc-link-lib=static=TKTopAlgo");
    println!("cargo:rustc-link-lib=static=TKGeomAlgo");
    println!("cargo:rustc-link-lib=static=TKGeomBase");
    println!("cargo:rustc-link-lib=static=TKBRep");

    let _build = cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .include(format!("{}", dst.join("include").join("opencascade").display()))
        .include("include")
        .file("cpp/wrapper.cpp")
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static=wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/wrapper.hxx");
    println!("cargo:rerun-if-changed=cpp/wrapper.cpp");
}