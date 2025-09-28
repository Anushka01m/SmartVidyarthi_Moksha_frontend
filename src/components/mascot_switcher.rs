use leptos::*;
#[component]
pub fn MascotSwitcher() -> impl IntoView {
    let mascots = vec![
        ("/static/mascots/pikachu.svg", "Pikachu"),
        ("/static/mascots/doraemon.svg", "Doraemon")
    ];
    let (current, set_current) = create_signal(0);

    view! {
        <div class="kid-card">
            <h2>"Choose Your Mascot"</h2>
            <div>
                {mascots.iter().enumerate().map(|(i, (url, name))| view! {
                    <img src={(*url).to_string()} title={(*name).to_string()}
                        class={if i == current.get() { "selected-mascot" } else { "" }}
                        width="64"
                        on:click=move |_| set_current.set(i)
                    />
                }).collect_view()}
            </div>
        </div>
    }
}
