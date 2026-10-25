use std::env;
use std::fs;
use std::path::PathBuf;

const APP_MANIFEST: &str = include_str!("../assets/app.manifest");

/// Embeds the Windows compatibility manifest needed by XAML Islands apps.
///
/// Call this from the consuming package's `build.rs`. It does nothing when
/// the package is built for a non-Windows target.
pub fn embed_manifest() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_path = out_dir.join("islands-reactor.manifest");
    fs::write(&manifest_path, APP_MANIFEST).unwrap_or_else(|error| {
        panic!(
            "failed to write manifest {}: {error}",
            manifest_path.display()
        )
    });

    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV not set");
    let target_abi = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();
    match (target_env.as_str(), target_abi.as_str()) {
        ("msvc", _) => {
            println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        ("gnu", "llvm") => {
            println!("cargo:rustc-link-arg-bins=-Wl,/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-bins=-Wl,/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        _ => panic!("unsupported target environment: {target_env}{target_abi}"),
    }
}
