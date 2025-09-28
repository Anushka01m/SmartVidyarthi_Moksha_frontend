use leptos::*;
#[component]
pub fn Quiz() -> impl IntoView {
    let (show_cheer, set_show_cheer) = create_signal(false);

    let on_submit = move |e: web_sys::Event| {
        e.prevent_default();
        set_show_cheer.set(true);
    };

    view! {
        <div class="kid-card">
            <h2>"Quiz Time!"</h2>
            <form on:submit=on_submit>
                <p>"Who is the mascot of Pokémon?"</p>
                <label><input type="radio" name="q1" value="pikachu" />"Pikachu"</label>
                <label><input type="radio" name="q1" value="eevee" />"Eevee"</label>
                <button type="submit">"Submit"</button>
            </form>
            {move || if show_cheer.get() {
                view! {
                  <div class="mascot-cheer">
                    <img src="/static/mascots/pikachu.svg" width="80"/>
                    <p style="color:gold;"><b>Great job!</b> 🎉</p>
                  </div>
                }
            } else { view!{} }}
        </div>
    }
}
