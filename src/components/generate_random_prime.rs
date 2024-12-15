use dioxus::prelude::*;

use crate::{router::Route, services::generate_random_prime};

#[component]
pub fn GenerateRandomPrime() -> Element {
    let mut draft = use_signal(|| "".to_string());
    let mut random_prime = use_signal(|| 0);
    let mut error = use_signal(|| "".to_string());

    let onkeydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter && !draft.read().is_empty() {
            let number = draft.to_string();
            let draft_number: u32 = match number.parse() {
                Ok(num) => num,
                Err(_e) => {
                    error.set("変換に失敗しました".to_string());
                    return;
                }
            };
            match generate_random_prime::generate_random_prime(draft_number) {
                Ok(prime) => random_prime.set(prime),
                Err(_e) => error.set("変換に失敗しました".to_string()),
            }
            error.set("".to_string());
        }
    };
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        br {}
        input {
                class: "new-todo",
                placeholder: "number",
                value: "{draft}",
                autofocus: "true",
                oninput: move |evt| draft.set(evt.value()),
                onkeydown
            }
        div {
            h1 { "Random prime: {random_prime}" }
        }
        div {
            h1 { "{error}" }
        }
    }
}
