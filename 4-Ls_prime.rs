//4. Implement a function that checks whether a given number is prime or not.

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num = 4;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}
