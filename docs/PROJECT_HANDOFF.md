# Islands Reactor Project Handoff

Use this file as the starting context for a fresh agent conversation.

## Project Goal

`islands-reactor` tracks the public API and internal structure of the current
upstream Reactor crate at `reference/windows-rs/crates/libs/reactor`. Its
platform kernel hosts `Windows.UI.Xaml` in XAML Islands.

The workspace is arranged as follows:

- `reference/windows-rs`: upstream `windows-rs` submodule
- `crates/islands-reactor`: Reactor-compatible library and native backend
- `crates/tools/reactor`: Islands binding/code generator, following the upstream tool layout
- `samples`: gallery and small applications

## Runtime

Applications depend on `islands-reactor` normally. The setup helper has been
removed, so sample builds do not add a build dependency or build script.

The host can load an `Islands.UI.Xaml.Controls.pri` file placed beside the
executable. Runtime deployment and the application manifest are managed by the
application outside this workspace.

## Bindings

Normal builds use checked-in bindings and do not download packages. Binding
generation is explicit:

```powershell
cargo run -p tool-reactor -- generate-bindings
```

Bindings live under `crates/islands-reactor/src/native`:

- `bindings.rs`: Windows XAML, Win32, and WinRT support bindings
- `bindings_iuxc.rs`: private bindings for the Islands controls package
- `winui/backend/generated`: generated property and event dispatch modules

The code generator reads `crates/tools/reactor/src/schema.toml` and its binding
filters, then regenerates every checked-in generated Rust file under
`crates/islands-reactor/src`. Public widget declarations remain handwritten
because Islands uses its own declaration API and control set; they have no
one-to-one generated declaration file in this port. Generation does not stage
runtime DLL or PRI assets.

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
