fn fib(max: i32) {
    let mut n = 0;
    let mut a = 0;
    let mut b = 1;
    while n < max {
        println!("{}", b);
        let x = a;
        a = b;
        b = x + b;
        n += 1;
    }
}

fn main() {
    fib(15)
}
