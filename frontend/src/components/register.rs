use leptos::*;
#[component]
pub fn Register() -> impl IntoView {
    view! {
        <div class="kid-card">
            <h2>"Register"</h2>
            <form>
                <label>"Name" <input type="text" /></label>
                <label>"Email" <input type="email" /></label>
                <label>"Password" <input type="password" /></label>
                <button type="submit">"Register"</button>
            </form>
        </div>
    }
}
