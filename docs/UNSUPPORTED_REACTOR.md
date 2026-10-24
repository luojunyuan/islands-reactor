# Compatibility Status

The crate follows the current Reactor layout in `reference/windows-rs`, with a
`Windows.UI.Xaml` XAML Islands backend. It uses system XAML controls and the
project's `Islands.UI.Xaml.Controls` runtime for the additional `ScrollView` and
`TitleBar` controls. Runtime deployment is managed outside this workspace.

## Adapted Surface

| API | Local implementation |
| --- | --- |
| `ScrollView` | `Islands.UI.Xaml.Controls.ScrollView` |
| `SelectorBar` | Horizontal `ToggleButton` composition with single selection |
| `TitleBar` | Islands control with Win32 non-client integration |
| `Backdrop` | Win32 DWM system backdrop attributes |

## Omitted Surface

The following controls are not part of the crate or gallery because the
Windows XAML Islands baseline does not provide them:

- `BreadcrumbBar`
- `Expander`
- `InfoBadge`
- `InfoBar`
- `NavigationView`
- `NumberBox`
- `RadioButtons`
- `TabView`
- `TeachingTip`

Other differences from Reactor include the WinAppSDK-specific window and
composition APIs. The host uses Win32 and DWM integration where a comparable
behavior is available. `SurfaceImageSource` and `SwapChainPanel` interop helpers
remain unsupported because their native interop interfaces are not included in
the checked-in bindings.

## Bindings

Normal Cargo builds use checked-in bindings and do not run `windows-bindgen` or
download packages. Regeneration is explicit:

```powershell
cargo run -p islands-reactor-codegen -- generate-bindings
```

The generator uses the Windows XAML metadata and the local Islands controls
metadata. It generates checked-in Rust bindings and dispatch modules, but does
not vendor runtime DLL or PRI files.
