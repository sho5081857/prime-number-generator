use rand::Rng;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn generate_random_prime(digits: u32) -> Result<u64, &'static str> {
    if digits < 1 {
        return Err("桁数は1以上でなければなりません");
    }

    let lower_bound = 10_u64.pow(digits - 1);
    let upper_bound = 10_u64.pow(digits) - 1;

    let mut rng = rand::thread_rng();
    let max_attempts = 1_000_000; // 最大試行回数を設定

    for _ in 0..max_attempts {
        let candidate = rng.gen_range(lower_bound..=upper_bound);
        if candidate % 2 != 0 && is_prime(candidate) {
            return Ok(candidate);
        }
    }

    Err("指定された桁数の素数を見つけることができませんでした")
}
