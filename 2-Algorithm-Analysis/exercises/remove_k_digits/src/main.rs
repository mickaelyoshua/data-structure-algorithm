use std::f64;

fn remove_kdigits(num: &str, k: usize) -> String {
    let mut min = f64::INFINITY;

    for i in 0..=num.len()-k {
        let value = format!("{}{}", &num[..i], &num[i+k..]).parse().unwrap();
        if value < min {
            min = value;
        }

    }
    min.to_string()
}

fn main() {
    let num = "1432219";
    let k = 3;

    let result = remove_kdigits(num, k);
    assert_eq!(result, "1219");
}
