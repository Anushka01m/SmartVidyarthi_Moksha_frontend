use leptos::*;
#[component]
pub fn Mentor() -> impl IntoView {
    view! {
        <div class="kid-card">
            <h2>"Mentor Help"</h2>
            <form>
                <input type="text" placeholder="Type your message..." />
                <button type="submit">"Send to Mentor"</button>
            </form>
            <button on:click=move |_| {
                let win = web_sys::window().unwrap();
                win.open_with_url_and_target("https://meet.jit.si/RuralLearningHelp", "_blank").unwrap();
            }>"Start Video Call"</button>
        </div>
    }
}
