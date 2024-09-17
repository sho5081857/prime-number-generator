use rand::Rng;

pub fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let sqrt_num = (num as f64).sqrt().floor() as usize;
    for i in (5..=sqrt_num).step_by(6) {
        if num % (i as u64) == 0 || num % ((i + 2) as u64) == 0 {
            return false;
        }
    }

    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
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
