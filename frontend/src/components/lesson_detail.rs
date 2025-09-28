use leptos::*;
#[component]
pub fn LessonDetail() -> impl IntoView {
    let read_text = "Gravity keeps us on Earth!";
    view! {
        <div class="kid-card">
            <h2>"Today's Lesson: Gravity"</h2>
            <button on:click=move |_| {
              let win = web_sys::window().unwrap();
              let synth = win.speech_synthesis().unwrap();
              let utt = web_sys::SpeechSynthesisUtterance::new_with_text(read_text).unwrap();
              synth.speak(&utt);
            }>"🔈 Listen"</button>
            <p>{read_text}</p>
        </div>
    }
}
