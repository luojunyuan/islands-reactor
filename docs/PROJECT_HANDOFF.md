# Islands Reactor Project Handoff

Use this file as the starting context for a fresh agent conversation.

## Project Goal

`islands-reactor` tracks the public API and internal structure of the current
upstream Reactor crate at `reference/windows-rs/crates/libs/reactor`. Its
platform kernel hosts `Windows.UI.Xaml` in XAML Islands.

The workspace is arranged as follows:

- `reference/windows-rs`: upstream `windows-rs` submodule
- `crates/islands-reactor`: Reactor-compatible library and native backend
- `crates/islands-reactor-setup`: app manifest and runtime setup helper
- `crates/tools/islands-reactor-codegen`: checked-in binding/code generator
- `samples`: gallery and small applications

## Runtime

Applications depend on `islands-reactor` normally and use
`islands-reactor-setup` as a build dependency. App build scripts call:

```rust
fn main() {
    islands_reactor_setup::embed_manifest();
    println!("cargo:rerun-if-changed=build.rs");
}
```

The setup crate stages the `Islands.UI.Xaml.Controls` DLL and PRI, embeds the
app manifest, and creates or merges the app `resources.pri`. It is a library
crate and does not need its own build script.

## Bindings

Normal builds use checked-in bindings and do not download packages. Binding
generation is explicit:

```powershell
cargo run -p islands-reactor-codegen -- generate-bindings
```

Bindings live under `crates/islands-reactor/src/native`:

- `bindings.rs`: Windows XAML, Win32, and WinRT support bindings
- `bindings_iuxc.rs`: private bindings for the Islands controls package

The code generator reads the workspace declarations in
`crates/tools/islands-reactor-codegen/src/reactor_widgets.toml` and regenerates
widget bindings and backend dispatch tables.

## Porting Contract

For platform-neutral behavior, follow the matching implementation in
`reference/windows-rs/crates/libs/reactor`. Preserve the public API, state
shape, prop/event names, and generated binding model where possible. Use the
system XAML controls first; isolate platform-specific behavior in the native
backend or Islands runtime adapter.

Document remaining behavior gaps in `docs/UNSUPPORTED_REACTOR.md`. Do not add
runtime dependencies for controls that are absent from the Windows XAML
baseline.

## Implementation Notes

- Button variants map to XAML style resources: `AccentButtonStyle`,
  `SubtleButtonStyle`, and `TextBlockButtonStyle`.
- Theme brush bindings use XAML style setters with `{ThemeResource ...}`.
- Backdrop support uses `WS_EX_NOREDIRECTIONBITMAP` and DWM system backdrop
  attributes.
- `ScrollView` and `TitleBar` use the private Islands controls package.

## Migration Target

```toml
[dependencies]
islands-reactor = { path = "..." }
```

```rust
use islands_reactor::*;
```

Application code should stay close to upstream Reactor except where the
platform compatibility document records a gap.
