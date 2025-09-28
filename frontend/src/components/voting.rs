use leptos::*;
#[component]
pub fn Voting() -> impl IntoView {
    let (ideas, set_ideas) = create_signal(vec![
        ("Science in Marathi", 12),
        ("Math regional stories", 9)
    ]);
    view! {
        <div class="kid-card">
            <h2>"Community Suggestions"</h2>
            <ul>
                {ideas.get().iter().map(|(s, v)| view! {
                    <li>{format!("{} (Votes: {})", s, v)} <button>"👍"</button></li>
                }).collect_view()}
            </ul>
        </div>
    }
}
