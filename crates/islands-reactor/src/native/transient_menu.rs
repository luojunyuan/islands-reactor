use std::collections::HashSet;
use std::ffi::c_void;

use windows_core::{Error, HRESULT, PCWSTR, Result};

use super::notifyicon::ScreenPoint;

type Hmenu = *mut c_void;
type Hwnd = *mut c_void;

const MF_GRAYED: u32 = 0x0000_0001;
const MF_POPUP: u32 = 0x0000_0010;
const MF_SEPARATOR: u32 = 0x0000_0800;
const MF_STRING: u32 = 0x0000_0000;
const TPM_NONOTIFY: u32 = 0x0000_0080;
const TPM_RETURNCMD: u32 = 0x0000_0100;
const TPM_RIGHTBUTTON: u32 = 0x0000_0002;
const WM_NULL: u32 = 0;

windows_core::link!("user32.dll" "system" fn AppendMenuW(menu : Hmenu, flags : u32, item : usize, text : PCWSTR) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn CreatePopupMenu() -> Hmenu);
windows_core::link!("user32.dll" "system" fn DestroyMenu(menu : Hmenu) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn PostMessageW(hwnd : Hwnd, message : u32, wparam : usize, lparam : isize) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetForegroundWindow(hwnd : Hwnd) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn TrackPopupMenuEx(menu : Hmenu, flags : u32, x : i32, y : i32, hwnd : Hwnd, params : *const c_void) -> u32);

/// An item accepted by a [`TransientMenu`].
pub enum TransientMenuItem {
    /// A command with an application-defined identifier.
    Item {
        id: u32,
        label: String,
        enabled: bool,
    },
    /// A visual separator.
    Separator,
    /// A submenu containing more items.
    Submenu {
        label: String,
        items: Vec<TransientMenuItem>,
    },
}

impl TransientMenuItem {
    /// Creates an enabled command item.
    pub fn item(id: u32, label: impl Into<String>) -> Self {
        Self::Item {
            id,
            label: label.into(),
            enabled: true,
        }
    }

    /// Creates a disabled command item.
    pub fn disabled(id: u32, label: impl Into<String>) -> Self {
        Self::Item {
            id,
            label: label.into(),
            enabled: false,
        }
    }

    /// Creates a separator.
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Creates a submenu.
    pub fn submenu(
        label: impl Into<String>,
        items: impl IntoIterator<Item = TransientMenuItem>,
    ) -> Self {
        Self::Submenu {
            label: label.into(),
            items: items.into_iter().collect(),
        }
    }
}

/// A native Windows popup menu for a notification-area icon.
pub struct TransientMenu {
    items: Vec<TransientMenuItem>,
    on_command: Box<dyn Fn(u32)>,
}

impl TransientMenu {
    /// Creates a menu and a callback for the selected command identifier.
    ///
    /// Command identifiers must be nonzero and unique across the full menu.
    pub fn new(
        items: impl IntoIterator<Item = TransientMenuItem>,
        on_command: impl Fn(u32) + 'static,
    ) -> Self {
        Self {
            items: items.into_iter().collect(),
            on_command: Box::new(on_command),
        }
    }

    pub(crate) fn show(&self, owner: *mut c_void, position: ScreenPoint) -> Result<()> {
        validate_items(&self.items, &mut HashSet::new())?;
        let menu = create_menu(&self.items)?;
        let _menu = OwnedMenu(menu);

        unsafe {
            _ = SetForegroundWindow(owner);
        }
        let command = unsafe {
            TrackPopupMenuEx(
                menu,
                TPM_RETURNCMD | TPM_NONOTIFY | TPM_RIGHTBUTTON,
                position.x,
                position.y,
                owner,
                std::ptr::null(),
            )
        };
        unsafe {
            _ = PostMessageW(owner, WM_NULL, 0, 0);
        }
        if command != 0 {
            (self.on_command)(command);
        }
        Ok(())
    }
}

struct OwnedMenu(Hmenu);

impl Drop for OwnedMenu {
    fn drop(&mut self) {
        unsafe {
            _ = DestroyMenu(self.0);
        }
    }
}

fn validate_items(items: &[TransientMenuItem], ids: &mut HashSet<u32>) -> Result<()> {
    for item in items {
        match item {
            TransientMenuItem::Item { id, .. } => {
                if *id == 0 || !ids.insert(*id) {
                    return Err(Error::new(
                        HRESULT(0x8007_0057_u32 as i32),
                        "transient menu command identifiers must be nonzero and unique",
                    ));
                }
            }
            TransientMenuItem::Separator => {}
            TransientMenuItem::Submenu { items, .. } => validate_items(items, ids)?,
        }
    }
    Ok(())
}

fn create_menu(items: &[TransientMenuItem]) -> Result<Hmenu> {
    let menu = unsafe { CreatePopupMenu() };
    if menu.is_null() {
        return Err(Error::from_thread());
    }

    for item in items {
        let result = match item {
            TransientMenuItem::Item { id, label, enabled } => append_menu(
                menu,
                MF_STRING | if *enabled { 0 } else { MF_GRAYED },
                *id as usize,
                label,
            ),
            TransientMenuItem::Separator => {
                if unsafe { AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null()) }.as_bool() {
                    Ok(())
                } else {
                    Err(Error::from_thread())
                }
            }
            TransientMenuItem::Submenu { label, items } => match create_menu(items) {
                Ok(submenu) => {
                    if let Err(error) = append_menu(menu, MF_POPUP, submenu as usize, label) {
                        unsafe {
                            _ = DestroyMenu(submenu);
                        }
                        Err(error)
                    } else {
                        Ok(())
                    }
                }
                Err(error) => Err(error),
            },
        };
        if let Err(error) = result {
            unsafe {
                _ = DestroyMenu(menu);
            }
            return Err(error);
        }
    }

    Ok(menu)
}

fn append_menu(menu: Hmenu, flags: u32, item: usize, label: &str) -> Result<()> {
    let label = label.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
    if unsafe { AppendMenuW(menu, flags, item, PCWSTR(label.as_ptr())) }.as_bool() {
        Ok(())
    } else {
        Err(Error::from_thread())
    }
}
