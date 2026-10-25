# Compatibility Status

The crate follows the current Reactor layout in `reference/windows-rs`, with a
`Windows.UI.Xaml` XAML Islands backend. It uses Windows system XAML controls;
the workspace has no additional XAML control package to deploy.

## Adapted Surface

| API | Local implementation |
| --- | --- |
| `ScrollView` | `Windows.UI.Xaml.Controls.ScrollViewer` |
| `SelectorBar` | Horizontal `ToggleButton` composition with single selection |
| `TitleBar` | System XAML composition with Win32 non-client integration |
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
cargo run -p tool-reactor -- generate-bindings
```

The generator uses the Windows XAML metadata. It generates checked-in Rust
bindings and dispatch modules without NuGet packages or extra runtime assets.
