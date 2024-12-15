use dioxus::prelude::*;

use crate::components::{
    blog::Blog, generate_random_prime::GenerateRandomPrime,
    generate_random_primes::GenerateRandomPrimes, home::Home, is_prime::IsPrime,
};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
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
