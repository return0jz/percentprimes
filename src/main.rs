fn is_prime(n: u64) -> bool {
    let sqrt_n = ((n as f64).sqrt() as u64) + 1;
    for i in 2..sqrt_n {
        if (n%i) > 0 {
            return false;
        }
    }
    true
}

fn main() {
    let (mut n_prime, mut n_not) = (0 as u64, 0 as u64);

    let mut i : u64 = 0;
    loop {
        if is_prime(i) {
            n_prime += 1;
        } else {
            n_not += 1
        }
        let percent = n_prime as f64 / (n_prime + n_not) as f64;
        println!("at {}: {}%", i, percent);
        std::thread::sleep(std::time::Duration::from_millis(200));
        i += 1;
    }
}
