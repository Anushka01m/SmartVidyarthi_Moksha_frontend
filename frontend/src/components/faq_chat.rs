use leptos::*;
use leptos::ev::SubmitEvent;

#[component]
pub fn FaqChat() -> impl IntoView {
    let (messages, set_messages) = create_signal::<Vec<String>>(vec![]);
    let (input, set_input) = create_signal("".to_string());
    let (bot_thinking, set_bot_thinking) = create_signal(false);

    let send = move |e: SubmitEvent| {
        e.prevent_default();
        let user_msg = input.get();
        if user_msg.trim().is_empty() { return; }
        set_messages.update(|msgs| msgs.push(format!("You: {}", user_msg)));
        set_bot_thinking.set(true);

        // Simulate backend API call:
        let set_messages = set_messages.clone();
        let set_bot_thinking = set_bot_thinking.clone();
        let question = user_msg.clone();

        spawn_local(async move {
            // Replace this with real API POST call:
            // let answer = api::mascot_faq_answer(&question).await.unwrap_or("Mascot: Sorry, I couldn't answer.".to_string());

            // For demo, we fake a reply!
            let answer = if question.to_lowercase().contains("math") {
                "Mascot: Math is amazing! Let's solve problems together.".to_string()
            } else if question.to_lowercase().contains("hello") {
                "Mascot: Hello! How can I help you learn today?".to_string()
            } else {
                "Mascot: Great question! Let me find out for you...".to_string()
            };
            leptos::sleep(std::time::Duration::from_millis(1100)).await;

            set_messages.update(|msgs| msgs.push(answer));
            set_bot_thinking.set(false);
        });

        set_input.set(String::new());
    };

    view! {
        <div class="kid-card">
            <h2>
                <img src="/static/mascots/pikachu.svg" width="40" style="vertical-align:middle;"/> "Ask the Mascot"
            </h2>
            <div class="chat-box" style="max-height:220px;overflow-y:auto;">
                {messages.get().iter().map(|msg| view! {
                    <div class={if msg.starts_with("You:") {"chat-bubble user"} else {"chat-bubble bot"}}>
                        {msg}
                    </div>
                }).collect_view()}
                { if bot_thinking.get() {
                  view!{
                    <div class="chat-bubble bot">
                        <img src="/static/mascots/pikachu.svg" class="mascot-thinking" width="24" style="vertical-align:middle;"/>
                        <span>Thinking...</span>
                    </div>
                  }
                } else {view!{}}
            </div>

            <form on:submit=send style="display:flex;gap:0.5em;">
                <input
                    type="text"
                    value=input.get()
                    on:input=move |e| set_input.set(event_target_value(e))
                    placeholder="Ask me anything!"
                    style="flex:1;padding:0.5em;"
                />
                <button type="submit">"Ask"</button>
            </form>
        </div>
    }
}
