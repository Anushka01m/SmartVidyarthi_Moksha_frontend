use leptos::*;
#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let (theme, set_theme) = create_signal("light".to_string());
    view! {
        <div class="kid-card">
            <h2>"Theme Switcher"</h2>
            <button on:click=move |_| set_theme.set("light".to_string())>"Light"</button>
            <button on:click=move |_| set_theme.set("dark".to_string())>"Dark"</button>
            <button on:click=move |_| set_theme.set("space".to_string())>"Space"</button>
        </div>
    }
}
