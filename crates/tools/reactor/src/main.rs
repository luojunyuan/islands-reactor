use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

mod gen_bindings;
mod helpers;
mod metadata;
mod native;
mod schema;
mod toml_parser;

const SCHEMA: &str = include_str!("schema.toml");

const XAML_MINIMAL_FILTERS: &str = include_str!("bindings.txt");

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("generate-bindings") if args.next().is_none() => {
            if let Err(err) = generate_bindings() {
                eprintln!("tool-reactor: {err}");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: cargo run -p tool-reactor -- generate-bindings");
            std::process::exit(2);
        }
    }
}

fn generate_bindings() -> Result<(), String> {
    let root = workspace_root()?;
    generate_xaml_bindings(&root)?;
    generate_reactor_code(&root)?;
    Ok(())
}

fn generate_xaml_bindings(root: &Path) -> Result<(), String> {
    let out = root.join("crates/islands-reactor/src/native/bindings.rs");
    let temp = root.join("target").join("tool-reactor").join("bindings.rs");
    if let Some(parent) = temp.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    windows_bindgen::builder()
        .input_default()
        .output(&temp)
        .minimal()
        .flat()
        .filters(XAML_MINIMAL_FILTERS.lines())
        .write();
    patch_xaml_bindings(&temp)?;
    mark_generated(&temp)?;
    copy_if_changed(&temp, &out)
}

fn generate_reactor_code(root: &Path) -> Result<(), String> {
    let metadata_dir = stage_reactor_metadata(root)?;
    let resolver = metadata::MetadataResolver::load(&metadata_dir);
    let mut controls = toml_parser::parse(SCHEMA, &resolver);

    for ctrl in &mut controls {
        let handle = ctrl.handle().to_string();
        for prop in &mut ctrl.prop {
            prop.resolve_defaults(Some(&resolver), &handle);
        }
    }

    for warning in schema::validate(&controls, &resolver) {
        eprintln!("{warning}");
    }

    write_generated_rs(
        &root.join("crates/islands-reactor/src/generated.rs"),
        &gen_bindings::generate(&controls),
    )?;
    write_generated_rs(
        &root.join("crates/islands-reactor/src/native/winui/backend/generated/set_prop.rs"),
        &native::set_prop::generate(&controls, &resolver),
    )?;
    write_generated_rs(
        &root.join("crates/islands-reactor/src/native/winui/backend/generated/attach_event.rs"),
        &native::attach_event::generate(&controls, &resolver),
    )?;
    write_generated_rs(
        &root.join("crates/islands-reactor/src/native/winui/backend/generated.rs"),
        native::BACKEND_GENERATED_MODULE,
    )
}

fn stage_reactor_metadata(root: &Path) -> Result<PathBuf, String> {
    let work = root
        .join("target")
        .join("tool-reactor")
        .join("reactor-metadata");
    recreate_dir(&work)?;

    let windows_winmd = locate_windows_bindgen_default_winmd(root)?;
    copy_file(&windows_winmd, &work.join("Windows.winmd"))?;
    Ok(work)
}

fn locate_windows_bindgen_default_winmd(root: &Path) -> Result<PathBuf, String> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(root.join("Cargo.toml"))
        .exec()
        .map_err(|err| format!("cargo metadata failed: {err}"))?;

    let package = metadata
        .packages
        .iter()
        .find(|package| package.name.as_str() == "windows-bindgen")
        .ok_or_else(|| "cargo metadata did not include windows-bindgen".to_string())?;
    let manifest = package.manifest_path.as_std_path();
    let winmd = manifest
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| {
            format!(
                "windows-bindgen manifest has no libraries directory: {}",
                manifest.display()
            )
        })?
        .join("default")
        .join("Windows.winmd");
    require_file(&winmd, "windows-bindgen default Windows.winmd")?;
    Ok(winmd)
}

fn write_generated_rs(path: &Path, code: &str) -> Result<(), String> {
    write_if_changed(path, &rustfmt(code))
}

fn write_if_changed(path: &Path, content: &str) -> Result<(), String> {
    if let Ok(existing) = fs::read_to_string(path)
        && existing == content
    {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(path, content).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn rustfmt(code: &str) -> String {
    let Ok(mut child) = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return code.to_string();
    };
    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(code.as_bytes());
    }
    match child.wait_with_output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).unwrap_or_else(|_| code.to_string())
        }
        _ => code.to_string(),
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("could not resolve workspace root from {}", dir.display()))
}

fn recreate_dir(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|err| format!("failed to remove {}: {err}", path.display()))?;
    }
    fs::create_dir_all(path).map_err(|err| format!("failed to create {}: {err}", path.display()))
}

fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "source file")?;
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::copy(src, dst).map(|_| ()).map_err(|err| {
        format!(
            "failed to copy {} to {}: {err}",
            src.display(),
            dst.display()
        )
    })
}

fn copy_if_changed(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "generated bindings")?;
    if let (Ok(existing), Ok(generated)) = (fs::read(dst), fs::read(src))
        && existing == generated
    {
        return Ok(());
    }
    copy_file(src, dst)
}

fn mark_generated(path: &Path) -> Result<(), String> {
    const HEADER: &str = "// This file is auto-generated by tool-reactor. Do not edit.\n";
    let mut code = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    if !code.starts_with(HEADER) {
        code.insert_str(0, HEADER);
        fs::write(path, code)
            .map_err(|err| format!("failed to write {}: {err}", path.display()))?;
    }
    Ok(())
}

fn patch_xaml_bindings(path: &Path) -> Result<(), String> {
    let mut code = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    patch_windows_core_api(&mut code);
    insert_selection_changed_event_handler_ctor(&mut code)?;
    fs::write(path, code).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn patch_windows_core_api(code: &mut String) {
    for name in ["Type", "CopyType", "CloneType"] {
        *code = code.replace(
            &format!("windows_core::{name}"),
            &format!("windows_core::imp::{name}"),
        );
    }
    *code = code
        .replace("windows_core::imp::Box::new", "std::boxed::Box::new")
        .replace("windows_core::Box::new", "std::boxed::Box::new");
}

fn insert_selection_changed_event_handler_ctor(code: &mut String) -> Result<(), String> {
    if code.contains("impl SelectionChangedEventHandler {\n    pub fn new<") {
        return Ok(());
    }

    let ctor = r#"struct SelectionChangedEventHandlerResultBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> SelectionChangedEventHandlerResultBox<F>
{
    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SelectionChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            )
            .into()
        }
    }
}
impl SelectionChangedEventHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    {
        let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, _>::new(
            &SelectionChangedEventHandlerResultBox::<F>::VTABLE,
            handler,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
"#;

    for needle in [
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct ListViewSelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct ListViewSelectionMode(pub i32);",
    ] {
        if let Some(index) = code.find(needle) {
            code.insert_str(index, ctor);
            return Ok(());
        }
    }

    Err("failed to patch SelectionChangedEventHandler constructor".to_string())
}

fn require_file(path: &Path, label: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("{label} not found: {}", path.display()))
    }
}
