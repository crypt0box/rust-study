use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{}", is_prime(100000007));
    let end = start.elapsed();
    println!("{}.{:03}s", end.as_secs(), end.subsec_nanos() / 1_000_000);
}

fn is_prime(num: u32) -> bool {
    if num == 1 {
        return false;
    } else if num == 2 {
        return true;
    } else {
        for i in 2..num {
            if num % i == 0 {
                return false;
            }
        }
        true
    }
}
