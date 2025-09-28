use leptos::*;
#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="kid-card">
            <h2>"Login"</h2>
            <form>
                <label>"Email" <input type="email" /></label>
                <label>"Password" <input type="password" /></label>
                <button type="submit">"Login"</button>
            </form>
        </div>
    }
}
