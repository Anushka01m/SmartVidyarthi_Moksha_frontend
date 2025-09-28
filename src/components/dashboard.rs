use leptos::*;
#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="kid-card">
            <h2>"My Dashboard"</h2>
            <a href="/quiz">"Take a Quiz"</a>
            <a href="/assignments">"Assignments"</a>
            <a href="/achievements">"Achievements"</a>
            <a href="/leaderboard">"Leaderboard"</a>
        </div>
    }
}
