pub enum Theme {
    Light,
    Dark,
    Space
}
pub fn apply_theme(theme: Theme) {
    let document = web_sys::window().unwrap().document().unwrap();
    let class = match theme {
        Theme::Light => "light-theme",
        Theme::Dark => "dark-theme",
        Theme::Space => "space-theme"
    };
    document.body().unwrap().set_class_name(class);
}
