use std::cell::Cell;
use std::rc::Rc;

use super::super::*;

#[test]
fn pointer_and_keyboard_handlers_are_attached_to_declarations() {
    let tapped = Rc::new(Cell::new(false));
    let tapped_callback = Rc::clone(&tapped);
    let element = Button::new("Save")
        .on_tapped(move || tapped_callback.set(true))
        .on_pointer_pressed(|info| assert!(info.is_left_button_pressed))
        .keyboard_accelerator(KeyboardAccelerator::new(
            KeyboardKey::S,
            KeyModifiers::CONTROL,
            || {},
        ));

    let handlers = element.modifiers.pointer_handlers.as_ref().unwrap();
    handlers.on_tapped.as_ref().unwrap().invoke(());
    handlers
        .on_pointer_pressed
        .as_ref()
        .unwrap()
        .invoke(PointerEventInfo {
            is_left_button_pressed: true,
            is_right_button_pressed: false,
            is_middle_button_pressed: false,
        });
    let accelerator = &element.modifiers.keyboard_accelerators.as_ref().unwrap()[0];
    accelerator.on_invoked.invoke(());

    assert!(tapped.get());
    assert_eq!(accelerator.key, KeyboardKey::S);
    assert!(accelerator.modifiers.contains(KeyModifiers::CONTROL));
}
