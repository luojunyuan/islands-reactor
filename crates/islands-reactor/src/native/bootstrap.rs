/// Compatibility handle returned by [`initialize`].
///
/// XAML Islands uses the OS `Windows.UI.Xaml` runtime, so no XAML Islands
/// bootstrap is required.
#[derive(Debug, Default)]
pub struct BootstrapHandle;

/// Initialize the reactor platform layer.
///
/// This is a no-op for islands-reactor and is kept only for source
/// compatibility with examples that call `bootstrap::initialize()`.
pub fn initialize() -> windows_core::Result<BootstrapHandle> {
    Ok(BootstrapHandle)
}
