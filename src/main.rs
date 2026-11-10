fn main() {
    println!("2 is prime: {}", is_prime(2));
    println!("1 is prime: {}", is_prime(1));
    println!("4 is prime: {}", is_prime(4));
    println!("13 is prime: {}", is_prime(13));

    println!("Next prime number to 75: {}", next_prime(75));
    println!("Previous prime number to 75: {}", prev_prime(75));

    println!("Amount of prime numbers that are less than or equal to 100: {}", prime_count(100))
}

fn is_prime(n: u32) -> bool { // check if an integer value is a prime number
    if n < 2 {
        return false;
    }
    for d in 2..n {
        if d * d > n {
            break;
        }
        if n % d == 0 {
            return false;
        }
    }
    true
}

fn next_prime(n: u32) -> u32 { // returns the smallest prime number that is greater than n
    let mut num: u32 = n + 1;
    loop {
        if is_prime(num) {
            break num;
        }

        num += 1;
    }
}

fn prev_prime(n: u32) -> u32 { // returns the biggest prime number that is less than n
    let mut num: u32 = n - 1;
    loop {
        if is_prime(num) {
            break num;
        }

        num -= 1;
    }
}

fn prime_count(n: u32) -> u32 { // returns how many primes are ≤ n

    let mut primes: u32 = 0;

    if n < 2 {
        return primes;
    }

    for d in 2..=n {
        if is_prime(d) {
            primes += 1;
        }
    }

    primes
}