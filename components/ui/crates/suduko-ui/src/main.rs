use suduko_ui::App;
use yew::Renderer;

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

fn main() {
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("build-info"))
    {
        let label = format!(
            "v{} | {} | {} | {}",
            env!("CARGO_PKG_VERSION"),
            build_info::BUILD_HOST,
            build_info::GIT_SHA,
            build_info::BUILD_TIMESTAMP
        );
        el.set_text_content(Some(&label));
    }
    if let Some(root) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("app"))
    {
        Renderer::<App>::with_root(root).render();
    } else {
        Renderer::<App>::new().render();
    }
}
