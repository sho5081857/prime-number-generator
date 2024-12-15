use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Home() -> Element {
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
