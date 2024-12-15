use dioxus::prelude::*;

use crate::{router::Route, services::generate_random_prime};

#[component]
pub fn IsPrime() -> Element {
    let mut draft = use_signal(|| "".to_string());
    let mut is_prime = use_signal(|| false);
    let mut error = use_signal(|| "".to_string());

    let onkeydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter && !draft.read().is_empty() {
            let number = draft.to_string();
            let draft_number: u64 = match number.parse() {
                Ok(num) => num,
                Err(_e) => {
                    error.set("変換に失敗しました".to_string());
                    return;
                }
            };
            is_prime.set(generate_random_prime::is_prime(draft_number));
        }
    };
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        br{}
        input {
                class: "new-todo",
                placeholder: "number",
                value: "{draft}",
                autofocus: "true",
                oninput: move |evt| draft.set(evt.value()),
                onkeydown
            }
        div {
            h1 { "Is prime: {is_prime}" }
        }
    }
}
