use leptos::*;
#[component]
pub fn Achievements() -> impl IntoView {
    let badges = vec![
        ("/static/badges/first_quiz.svg", "First Quiz"),
        ("/static/badges/perfect_score.svg", "Perfect Score")
    ];
    view! {
        <div class="kid-card">
            <h2>"My Badges"</h2>
            <div>
                {badges.into_iter().map(|(icon, name)| view! {
                    <img src={icon} title={name} class="badge-img"/>
                }).collect_view()}
            </div>
        </div>
    }
}
