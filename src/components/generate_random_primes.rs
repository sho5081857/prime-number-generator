use dioxus::prelude::*;

use crate::{router::Route, services::generate_random_primes};

#[component]
pub fn GenerateRandomPrimes() -> Element {
    let random_primes = generate_random_primes::generate_random_primes(1000).collect::<Vec<_>>();
    let random_primes_format = format!("{:?}", random_primes);
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        div {
            h1 { "Random prime: {random_primes_format}" }
        }
    }
}
