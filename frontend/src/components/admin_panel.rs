use leptos::*;
use serde::Deserialize;

// Example admin user row data structure.
// In real use, add/remove fields as needed.
#[derive(Clone, Deserialize)]
pub struct UserRow {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub xp_points: i32,
}

#[component]
pub fn AdminPanel() -> impl IntoView {
    // Here you would normally fetch the users from your backend API.
    // For this example, we use static/demo data:
    let (users, set_users) = create_signal(vec![
        UserRow { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string(), role: "student".to_string(), xp_points: 100 },
        UserRow { id: 2, name: "Ravi".to_string(), email: "ravi@example.com".to_string(), role: "teacher".to_string(), xp_points: 200 },
        UserRow { id: 3, name: "Nina".to_string(), email: "nina@example.com".to_string(), role: "admin".to_string(), xp_points: 300 },
    ]);
    // To integrate real backend, fetch API and set_users.

    view! {
        <div class="kid-card">
            <h2>"Admin Panel – Manage Users"</h2>
            <table>
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"Name"</th>
                        <th>"Email"</th>
                        <th>"Role"</th>
                        <th>"XP"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {users.get().iter().map(|u| view!{
                        <tr>
                            <td>{u.id}</td>
                            <td>{&u.name}</td>
                            <td>{&u.email}</td>
                            <td>{&u.role}</td>
                            <td>{u.xp_points}</td>
                            <td>
                                <button on:click=move |_| { /* promote logic here */ }>"Promote"</button>
                                <button on:click=move |_| { /* delete logic here */ }>"Delete"</button>
                            </td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}
