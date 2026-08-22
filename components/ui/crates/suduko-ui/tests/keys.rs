use suduko_ui::keys::{Key, parse_key};

#[test]
fn digits_and_space_parse_as_before() {
    assert!(matches!(parse_key("Digit5", "5"), Some(Key::Digit(5))));
    assert!(matches!(parse_key("Numpad7", "7"), Some(Key::Digit(7))));
    assert!(matches!(parse_key("Space", " "), Some(Key::Space)));
    assert!(matches!(parse_key("Escape", "Escape"), Some(Key::Escape)));
    assert!(parse_key("KeyA", "a").is_none());
}

#[test]
fn backspace_and_delete_both_erase() {
    // PC/Windows/Linux and Mac Backspace.
    assert!(matches!(
        parse_key("Backspace", "Backspace"),
        Some(Key::Space)
    ));
    // Mac fn+Backspace (forward delete) and PC Delete.
    assert!(matches!(parse_key("Delete", "Delete"), Some(Key::Space)));
    // Some browsers label the physical Delete key Backspace on Mac.
    assert!(matches!(parse_key("Delete", "Backspace"), Some(Key::Space)));
    // Unknown codes still fall through.
    assert!(parse_key("KeyX", "x").is_none());
}
