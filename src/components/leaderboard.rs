use leptos::*;
#[component]
pub fn Leaderboard() -> impl IntoView {
    let leaderboard = vec![
        ("Alice", 500), ("Ravi", 470), ("Nina", 450)
    ];
    view! {
        <div class="kid-card">
            <h2>"Top Learners"</h2>
            <ol>
                {leaderboard.into_iter().enumerate().map(|(i, (name, xp))| view! {
                    <li>{format!("{}. {} — {xp} XP", i+1, name)}</li>
                }).collect_view()}
            </ol>
        </div>
    }
}
