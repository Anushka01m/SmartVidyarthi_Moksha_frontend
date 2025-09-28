use leptos::*;
pub struct AppState {
    pub user: Option<String>,
    pub theme: String
}
pub fn default_state() -> AppState {
    AppState { user: None, theme: "light".to_string() }
}
