use std::f64;

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
