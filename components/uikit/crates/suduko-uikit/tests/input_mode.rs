use suduko_uikit::InputMode;

#[test]
fn input_mode_covers_the_three_placements() {
    assert_eq!(InputMode::default(), InputMode::Below);
    assert_ne!(InputMode::Above, InputMode::Popup);
    assert_eq!(format!("{}", InputMode::Above), "above");
    assert_eq!(format!("{}", InputMode::Below), "below");
    assert_eq!(format!("{}", InputMode::Popup), "popup");
}
