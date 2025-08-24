fn fibonacci_recursion(n: u64) -> u64 {
    if n <= 1 { // starting point
        return n
    }
    fibonacci_recursion(n-1) + fibonacci_recursion(n-2) // rule of succession
}

fn main() {
    let n = 5;
    let number = fibonacci_recursion(n);
    println!("The number of position {n} on the Fibonnaci Sequence is {number}");

    let n = [p1,...,pn];
    let m = n.len();
    let mut prod = 1;

    for i in 0..m {
        prod *= n[i];
    }

    println!("Product is {prod}");


}
