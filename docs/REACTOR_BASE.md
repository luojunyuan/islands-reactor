# Reactor Base

`islands-reactor` follows the current `windows-rs` Reactor crate structure and
preserves its public API where the platform allows. The rendering kernel uses
`Windows.UI.Xaml` hosted in XAML Islands rather than WinAppSDK windows.

Controls supplied by the Islands runtime live behind private bindings and
backend adapters. Platform differences that cannot be represented by the
Windows XAML controls are documented in `UNSUPPORTED_REACTOR.md`.
