pub fn generate_random_primes(number: usize) -> impl Iterator<Item = usize> {
    let mut cache = vec![true; number + 1];
    (2..=number).filter(move |&x| {
        if !cache[x] {
            return false;
        }
        for y in (x * 2..=number).step_by(x) {
            cache[y] = false;
        }
        true
    })
}
