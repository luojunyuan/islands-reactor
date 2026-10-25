# Reactor Example Ports

This package mirrors the example names under `reference/windows-rs/crates/samples/reactor`.
Each migrated example is an independent binary in `src/bin/<name>/main.rs`.

Run an example on Windows with:

```powershell
cargo run -p islands-reactor-examples --bin calculator --target x86_64-pc-windows-msvc
```

Check every migrated binary with:

```powershell
cargo check -p islands-reactor-examples --all-targets --target x86_64-pc-windows-msvc
```

## Migrated (39)

| Upstream example | Port notes |
| --- | --- |
| `app-bar-icon` | Command bar with symbol commands |
| `async-state` | Worker thread updates through `use_async_state` |
| `button-icon` | Stateful icon buttons |
| `calculator` | Two-operand calculator using buttons and render state |
| `card` | Border-based card surface |
| `color-scheme` | Reads the host's effective light/dark scheme |
| `component-input` | Reusable greeting view with explicit input |
| `context` | Context provider and descendant consumer |
| `controlled` | Controlled text box and slider |
| `counter` | Stateful counter |
| `exit-transition` | Conditional element with enter/exit animation |
| `font-icon` | Symbol glyphs on buttons |
| `form` | Text fields, checkbox, and submit state |
| `function-component` | Reusable render functions and local state |
| `icon-elements` | Symbol icons in command bar items |
| `image-reuse` | Reuses a file URI image source |
| `image-sources` | Loads the bundled PNG from a file URI |
| `keyed-list-reorder` | Reorders keyed child rows while preserving identity |
| `minesweeper` | Playable 9x9 board with reveal and right-click flagging |
| `notifyicon-basic` | Native tray icon and event callback |
| `opacity-transition` | Animated visibility |
| `radio-buttons` | RadioButton group with selected state |
| `responsive-navigation` | Responsive navigation built from the supported SplitView |
| `scale-transition` | Implicit scale transition |
| `scroll-viewer` | Scrollable list |
| `shape` | Rectangle, ellipse, and line controls |
| `text-block` | Text size, weight, selection, and wrapping |
| `text-block-alignment` | Control alignment comparison |
| `text-block-typography` | Font size, weight, family, and wrapping |
| `text-box-border` | Text boxes framed by Border controls |
| `text-box-read-only` | Read-only RichEditBox equivalent |
| `theme-brush` | Theme resource brush swatches |
| `theme-transition` | Switches the Island's requested theme |
| `tictactoe` | Interactive 3x3 board |
| `tooltip-placement` | Tooltip placement variants |
| `translation-transition` | Animated layout movement |
| `use-effect` | Runs an effect when counter state changes |
| `window` | Observes client size changes |
| `window-theme` | Theme and backdrop controls |

## Skipped (36)

| Upstream examples | Reason |
| --- | --- |
| `content-dialog`, `content-dialog-theme`, `message-box` | ContentDialog is declared, but open/close presentation is not wired through the Islands host yet. |
| `expander-style`, `navigation`, `navigation-view-icons`, `navigation-view-pane`, `navigation-view-properties`, `tab-view-add-button` | The required Expander, NavigationView, and TabView controls are not available. |
| `number-box`, `text-trimming`, `lightweight-resources` | The corresponding NumberBox, text trimming property, and typed lightweight resource overrides are not available. |
| `icon`, `image-icon-size` | The samples require a window icon or ImageIcon support, neither is exposed by the current app/control API. |
| `encoded-image` | Islands Image currently accepts URI sources, not encoded image buffers or SVG sources. |
| `dotsweeper`, `drag-drop`, `pointer-position`, `pointer-resize`, `pointer-tracking` | They depend on richer pointer coordinates/gestures, drag-and-drop, or native window resizing APIs. |
| `element-ref` | The current component API does not expose a typed reference to a realized control. |
| `notifyicon` | The full sample owns and coordinates multiple independent windows; only the basic tray-icon flow is included. |
| `secondary-window`, `window-placement`, `window-placement-registry`, `window-observers` | Multi-window hosting, persisted placement, and placement observers are not available. `window` and `color-scheme` cover the supported observer subset. |
| `explorer`, `gallery`, `virtual`, `workbench` | These larger applications depend on upstream component, navigation, or virtualization APIs that differ from Islands. |
| `notepad` | The sample's file picker and document integration are not available. |
| `solitaire`, `stacker` | The upstream versions depend on richer game-specific composition/canvas APIs than Islands exposes. |
| `framework_dependent`, `self_contained`, `startup_perf` | These exercise deployment packaging or startup benchmark infrastructure rather than portable Reactor UI behavior. |
