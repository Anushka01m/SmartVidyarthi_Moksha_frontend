use leptos::*;
#[component]
pub fn Assignments() -> impl IntoView {
    let items = vec![("Lesson 1","Due 2025-11-30", false), ("Quiz A", "Due 2025-12-01", true)];
    view! {
        <div class="kid-card">
            <h2>"Your Assignments"</h2>
            <ul>
            {items.into_iter().map(|(n, d, done)| view! {
                <li>
                  <b>{n}</b> {" "} <i>{d}</i> {" "} {if done{"✅"}else{"❌"}}
                </li>
            }).collect_view()}
            </ul>
        </div>
    }
}
