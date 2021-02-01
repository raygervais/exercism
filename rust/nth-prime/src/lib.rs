pub fn is_prime(n: u32) -> bool {
    return !(2..n - 1).any(|i| n % i == 0);
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];

    return (2..)
        .filter(|candidate: &u32| {
            if !primes.iter().any(|i| candidate % i == 0) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap();
}
