use std::{
    env,
    path::PathBuf
};

fn main() {
    println!("cargo:rustc-link-lib=gssapi_krb5");
    let bindings = bindgen::Builder::default()
        .whitelist_type("(OM_.+|gss_.+)")
        .whitelist_var("_?GSS_.+|gss_.+")
        .whitelist_function("gss_.*")
        .header("wrapper.h")
        .generate()
        .expect("failed to generate gssapi bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings")
}