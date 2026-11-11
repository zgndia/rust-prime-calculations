fn main() {
    let numbers: [u32; 15] = [7, 12, 23, 36, 41, 49, 53, 64, 71, 81, 89, 95, 97, 102, 113];

    for num in numbers {
        // loop through every number inside numbers and check if they are a prime number or not
        println!(
            "Number {num} is {}",
            if is_prime(num) {
                "a prime number!"
            } else {
                "not a prime number!"
            }
        );
    }

    let x: u32 = 75;
    let y: u32 = 36;

    println!("Next prime number after {x}: {}", next_prime(x));
    println!("Previous prime number before {y}: {}", prev_prime(y));

    let target: u32 = 100;

    let (numbers, count) = prime_count(target);
    println!("There are {count} prime numbers that are less than or equal to {target}, which are:");
    for n in &numbers {
        if n == &numbers[&numbers.len() - 1] {
            // if it's the last number in the vec
            println!("{n}");
            break;
        }
        print!("{n}, ");
    }

    let numbers: [u32; 5] = [18, 29, 45, 77, 100];

    for n in numbers {
        println!(
            "{n} is {}",
            if is_even_prime(n) {
                "an even prime number!"
            } else if is_prime(n) {
                "a prime number!"
            } else {
                "a regular number!"
            }
        );
    }
}

fn is_prime(n: u32) -> bool {
    // check if an integer value is a prime number
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

fn next_prime(n: u32) -> u32 {
    // returns the smallest prime number that is > n
    let mut num: u32 = n + 1;
    loop {
        if is_prime(num) {
            break num;
        }

        num += 1;
    }
}

fn prev_prime(n: u32) -> u32 {
    // returns the biggest prime number that is < n
    let mut num: u32 = n - 1;
    loop {
        if is_prime(num) {
            break num;
        }

        num -= 1;
    }
}

fn prime_count(n: u32) -> (Vec<u32>, u32) {
    // returns the primes that are ≤ n inside a vector and the count as a tuple

    let mut primes: Vec<u32> = Vec::new();
    let mut count: u32 = 0;

    if n < 2 {
        return (primes, count);
    }

    for d in 2..=n {
        if is_prime(d) {
            primes.push(d);
            count += 1;
        }
    }

    (primes, count) // return the prime numbers and the number count as a tuple
}

fn is_even_prime(n: u32) -> bool {
    // An even number that can only be divided by 1 and other even numbers will be called "even primes"

    if n < 2 || n % 2 != 0 {
        return false;
    }

    for d in (3..n).step_by(2) {
        // it will skip every even number
        if n % d != 0 {
            // division is not a whole number
            continue;
        }
        if n / d % 2 != 0 {
            return false;
        }
    }
    true
}
