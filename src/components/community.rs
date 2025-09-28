use leptos::*;
#[component]
pub fn CommunitySuggestions() -> impl IntoView {
    view! {
        <div class="kid-card">
            <h2>"Community Wall"</h2>
            <form>
                <textarea placeholder="Share your achievement!"></textarea>
                <button type="submit">"Post"</button>
            </form>
        </div>
    }
}
