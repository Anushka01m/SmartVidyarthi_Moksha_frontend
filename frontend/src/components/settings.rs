use leptos::*;
#[component]
pub fn Settings() -> impl IntoView {
    let (dyslexia, set_dyslexia) = create_signal(false);
    view! {
        <div class="kid-card">
            <h2>"Accessibility"</h2>
            <button on:click=move |_| set_dyslexia.set(!dyslexia.get())>
                {if dyslexia.get() {"Standard Font"} else {"Dyslexia Friendly Font"}}
            </button>
        </div>
    }
}
