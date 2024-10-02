fn main() {
    println!("cargo:rerun-if-changed=libsais/include");
    let out_dir = cmake::Config::new("libsais")
        .cflag("-DCMAKE_BUILD_TYPE=Release")
        .cflag("-DEXTRA_FLAGS=\"-march=native\"")
        .build_target("all")
        .build();
    println!("cargo:rustc-link-search=native={}/build", out_dir.display());
    println!("cargo:rustc-link-lib=static=libsais");

    bindgen::Builder::default()
        .header("libsais/include/libsais.h")
        .header("libsais/include/libsais16.h")
        .header("libsais/include/libsais64.h")
        .header("libsais/include/libsais16x64.h")
        .clang_arg("--include-directory=libsais")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
