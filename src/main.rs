#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod generate_random_prime;
mod generate_random_primes;
mod is_prime;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/generate-random-prime")]
    GenerateRandomPrime {},
    #[route("/generate-random-primes")]
    GenerateRandomPrimes {},
    #[route("/is-prime")]
    IsPrime {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        br{}
        Link {
            to: Route::GenerateRandomPrime {},
            "Go to Generate random prime"
        }
        br{}
        Link {
            to: Route::GenerateRandomPrimes {},
            "Go to Generate random primes"
        }
        br{}
        Link {
            to: Route::IsPrime {},
            "Go to Is Prime"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

#[component]
fn GenerateRandomPrime() -> Element {
    let mut draft = use_signal(|| "".to_string());
    let mut random_prime = use_signal(|| 0);
    let mut error = use_signal(|| "".to_string());

    let onkeydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter && !draft.read().is_empty() {

            let number = draft.to_string();
            let draft_number: usize = match number.parse() {
                Ok(num) => num,
                Err(e) => {
                    error.set("変換に失敗しました".to_string());
                    return;
                }
            };
            random_prime.set(generate_random_prime::generate_random_prime(draft_number).unwrap());
            draft.set("".to_string());
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
    }
}

#[component]
fn GenerateRandomPrimes() -> Element {
    let random_primes = generate_random_primes::generate_random_primes(1000).collect::<Vec<_>>();
    let random_primes_format = format!("{:?}", random_primes);
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        div {
            h1 { "Random prime: {random_primes_format}" }
        }
    }
}


#[component]
fn IsPrime() -> Element {
    let is_prime = is_prime::is_prime(5);
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        div {
            h1 { "Is prime: {is_prime}" }
        }
    }
}
