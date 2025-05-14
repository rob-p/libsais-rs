fn main() {
    println!("cargo:rerun-if-changed=libsais/include");
    let mut cmake = cmake::Config::new("libsais");
    let mut cmake = cmake
        .cflag("-DCMAKE_BUILD_TYPE=Release")
        .cflag("-DEXTRA_FLAGS=\"-march=native\"");
    /*if cfg!(feature = "openmp") {
        cmake = cmake.cflag("-DLIBSAIS_OPENMP");
    }
    */
    let out_dir = cmake.build_target("all").build();
    println!("cargo:rustc-link-search=native={}/build", out_dir.display());
    println!("cargo:rustc-link-lib=static=libsais");

    let mut bindgen = bindgen::Builder::default()
        .header("libsais/include/libsais.h")
        .header("libsais/include/libsais16.h")
        .header("libsais/include/libsais64.h")
        .header("libsais/include/libsais16x64.h")
        .clang_arg("--include-directory=libsais");
    /*if cfg!(feature = "openmp") {
        bindgen = bindgen.clang_arg("-DLIBSAIS_OPENMP");
        println!("cargo:rustc-link-lib=gomp");
    }*/
    bindgen
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
