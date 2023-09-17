fn main() {
    for i in 0..20 {
        println!("fib {}: {}", i, fib(i));
    }
}

fn fib(mut n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut n0: u64 = 0;
    let mut n1: u64 = 1;
    // here we are going to compute the n-th fib, n > 1
    while n > 1 {
        let temp = n1;
        n1 = n1 + n0;
        n0 = temp;
        n -= 1;
    }

    n1
}
