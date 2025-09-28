use leptos::*;
#[component]
pub fn TeacherDashboard() -> impl IntoView {
    let students = vec![
        ("Arjun", 80, "Done", "2025-12-01"),
        ("Rina", 45, "Pending", "2025-12-03")
    ];
    view! {
        <div class="kid-card">
            <h2>"Classroom Progress"</h2>
            <table>
                <thead>
                    <tr><th>"Name"</th><th>"XP"</th><th>"Assignment"</th><th>"Deadline"</th></tr>
                </thead>
                <tbody>
                {students.into_iter().map(|(name, xp, asn, deadline)| view! {
                    <tr>
                        <td>{name}</td>
                        <td>{xp}</td>
                        <td>{asn}</td>
                        <td>{deadline}</td>
                    </tr>
                }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}
