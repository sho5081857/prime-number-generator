#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod generate_random_prime;
mod generate_random_primes;

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
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

#[component]
fn GenerateRandomPrime() -> Element {
    let random_prime = generate_random_prime::generate_random_prime(5).unwrap();
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
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
