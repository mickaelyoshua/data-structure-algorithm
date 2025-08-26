fn power(a: i64, n: i32) -> i64 {
    if n == 0 {
        return 1
    }

    let x = power(a, n/2);

    if n%2 == 0 {
        x*x
    } else {
        a*x*x
    }
}

fn main() {
    let a = 3;
    let n = 4;

    let result = power(a, n);
    println!("{a} to the power of {n} is equal to {result}");
}
