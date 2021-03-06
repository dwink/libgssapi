use std::{env, path::PathBuf, process::Command};

fn search_pat(base: &str, pat: &str) -> bool {
    let res = Command::new("find")
        .arg(base)
        .arg("-name")
        .arg(pat)
        .output();
    match dbg!(res) {
        Err(_) => false,
        Ok(output) => output.stdout.len() > 0,
    }
}

enum Gssapi {
    Mit,
    Heimdal,
}

fn which() -> Gssapi {
    let (ldpath, mit_pat, heimdal_pat) = {
        if cfg!(target_os = "macos") {
            (
                env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap(),
                "libgssapi_krb5.dylib*",
                "libgssapi.dylib*",
            )
        } else if cfg!(target_family = "unix") {
            (
                env::var("LD_LIBRARY_PATH").unwrap(),
                "libgssapi_krb5.so*",
                "libgssapi.so*",
            )
        } else {
            panic!("use SSPI on windows")
        }
    };
    let paths = vec![
        "/lib",
        "/lib64",
        "/usr/lib",
        "/usr/lib64",
        "/usr/local/opt/krb5",
    ];
    for path in ldpath.split(':').chain(paths) {
        if search_pat(path, mit_pat) {
            return Gssapi::Mit;
        }
        if search_pat(path, heimdal_pat) {
            return Gssapi::Heimdal;
        }
    }
    panic!("no gssapi implementation found, install mit kerberos or heimdal");
}

fn main() {
    let imp = which();
    match imp {
        Gssapi::Mit => println!("cargo:rustc-link-lib=gssapi_krb5"),
        Gssapi::Heimdal => println!("cargo:rustc-link-lib=gssapi"),
    }
    let bindings = bindgen::Builder::default()
        .whitelist_type("(OM_.+|gss_.+)")
        .whitelist_var("_?GSS_.+|gss_.+")
        .whitelist_function("gss_.*")
        .header(match imp {
            Gssapi::Mit => "src/wrapper_mit.h",
            Gssapi::Heimdal => "src/wrapper_heimdal.h",
        })
        .generate()
        .expect("failed to generate gssapi bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings")
}
