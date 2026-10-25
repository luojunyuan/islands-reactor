use windows_core::{IInspectable, Interface, Result};

use crate::bindings as Xaml;

pub(super) fn string_as_textblock(text: &str) -> Result<IInspectable> {
    let tb = Xaml::TextBlock::new()?;
    tb.SetText(text)?;
    tb.cast()
}
