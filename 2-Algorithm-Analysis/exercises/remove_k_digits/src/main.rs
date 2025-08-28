fn remove_kdigits(num: &str, k: i32) -> String {
    let num_len = num.len() as i32;
    if k >= num_len {
        return String::from("0");
    }

    if k < 1 || num_len > 10_i32.pow(5) {
        panic!("k or number length out of bounds.");
    }
    
    let mut result: Vec<char> = Vec::new();
    let mut k = k;

    for digit in num.chars() {
        while !result.is_empty() && k > 0 && *result.last().unwrap() > digit {
            result.pop();
            k -= 1;
        }
        result.push(digit);
    }

    while k > 0 {
        result.pop();
        k -= 1;
    }

    let first_index = result.iter().position(|&c| c != '0').unwrap_or(result.len());
    let result_str = result[first_index..].iter().collect::<String>();

    if result_str.is_empty() {
        String::from("0")
    } else {
        result_str
    }
}

fn main() {
    let nums = ["1432219","10200","10","9","112"];
    let ks = [3,1,2,1,1];
    let results = ["1219","200","0","0","11"];
    
    for ((num, k), expected) in nums.iter().zip(ks.iter()).zip(results.iter()) {
        let result = remove_kdigits(num, *k);
        assert_eq!(result, *expected);
    }
}
