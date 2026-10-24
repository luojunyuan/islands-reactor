use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process, thread,
    time::{Duration, Instant},
};

#[allow(dead_code)]
mod iuxc_runtime;

const APP_MANIFEST: &str = include_str!("../assets/app.manifest");

pub fn embed_manifest() {
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    let manifest_asset = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("app.manifest");
    println!("cargo:rerun-if-changed={}", manifest_asset.display());

    stage_runtime();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_path = out_dir.join("islands-reactor-app.manifest");
    fs::write(&manifest_path, APP_MANIFEST).unwrap_or_else(|err| {
        panic!(
            "failed to write islands-reactor manifest to {}: {err}",
            manifest_path.display()
        )
    });

    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
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
        _ => panic!("unsupported target environment for islands-reactor manifest embedding"),
    }
}

fn stage_runtime() {
    let Ok(out_dir) = env::var("OUT_DIR").map(PathBuf::from) else {
        return;
    };
    let Some(arch) = target_arch_folder() else {
        println!(
            "cargo:warning=Islands.UI.Xaml runtime staging skipped: unsupported target architecture"
        );
        return;
    };
    let Some(asset_dir) = iuxc_runtime::runtime_asset_dir(arch) else {
        println!(
            "cargo:warning=Islands.UI.Xaml runtime staging skipped: unsupported target architecture"
        );
        return;
    };
    if let Some(target_dir) = target_output_dir(&out_dir) {
        if let Err(err) = copy_runtime_payload(&asset_dir, &target_dir) {
            println!(
                "cargo:warning=Islands.UI.Xaml runtime payload copy failed for {}: {err}",
                target_dir.display()
            );
        }
    } else {
        println!(
            "cargo:warning=Islands.UI.Xaml runtime staging skipped: could not resolve target dir"
        );
    }
}

fn target_arch_folder() -> Option<&'static str> {
    match env::var("CARGO_CFG_TARGET_ARCH").ok()?.as_str() {
        "x86_64" => Some("x64"),
        "aarch64" => Some("arm64"),
        _ => None,
    }
}

fn copy_runtime_payload(asset_dir: &Path, target_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(target_dir)?;
    let _stage_lock = RuntimeStageLock::acquire(target_dir)?;
    for name in [iuxc_runtime::CONTROLS_DLL, iuxc_runtime::CONTROLS_PRI] {
        let source = asset_dir.join(name);
        println!("cargo:rerun-if-changed={}", source.display());
        if source.exists() {
            copy_file_if_missing_or_stale(&source, target_dir.join(name))?;
        } else {
            println!(
                "cargo:warning=Islands.UI.Xaml runtime asset missing: {}",
                source.display()
            );
        }
    }
}

struct RuntimeStageLock {
    path: PathBuf,
}

impl RuntimeStageLock {
    fn acquire(target_dir: &Path) -> io::Result<Self> {
        let path = target_dir.join(".islands-reactor-runtime.lock");
        let started = Instant::now();
        loop {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(mut file) => {
                    use io::Write as _;
                    let _ = writeln!(file, "{}", process::id());
                    return Ok(Self { path });
                }
                Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                    if started.elapsed() > Duration::from_secs(120) {
                        return Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            format!("timed out waiting for {}", path.display()),
                        ));
                    }
                    thread::sleep(Duration::from_millis(50));
                }
                Err(err) => return Err(err),
            }
        }
    }
}

impl Drop for RuntimeStageLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn copy_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, dst)?;
    Ok(())
}

fn copy_file_if_missing_or_stale(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    if let (Ok(src_metadata), Ok(dst_metadata)) = (fs::metadata(src), fs::metadata(dst)) {
        let same_len = src_metadata.len() == dst_metadata.len();
        let dst_is_fresh = match (src_metadata.modified(), dst_metadata.modified()) {
            (Ok(src_modified), Ok(dst_modified)) => dst_modified >= src_modified,
            _ => false,
        };
        if same_len && dst_is_fresh {
            return Ok(());
        }
    }
    copy_file(src, dst)
}

fn target_output_dir(out_dir: &Path) -> Option<PathBuf> {
    let mut path = out_dir;
    while let Some(parent) = path.parent() {
        if path.file_name().and_then(|v| v.to_str()) == Some("build") {
            return Some(parent.to_path_buf());
        }
        path = parent;
    }
    None
}
