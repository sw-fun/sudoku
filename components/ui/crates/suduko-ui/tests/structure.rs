#[test]
fn ui_exposes_app_component() {
    fn assert_component<C: yew::BaseComponent>() {}
    assert_component::<suduko_ui::App>();
}
