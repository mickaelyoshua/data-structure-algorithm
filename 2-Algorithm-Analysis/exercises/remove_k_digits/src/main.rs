use std::f32;

fn remove_kdigits(num: &str, k: usize) -> String {
    if num.len() <= k {
        return String::from("0");
    }

    let mut min = f32::INFINITY;

    for i in 0..=num.len()-k {
        let value = format!("{}{}", &num[..i], &num[i+k..])
            .parse()
            .unwrap();
        if value < min {
            min = value;
        }
    }
    min.to_string()
}

fn main() {
    let nums = ["1432219","10200","10"];
    let ks = [3,1,2];
    let results = ["1219","200","0"];
    
    for (i, (num, k)) in nums.iter().zip(ks).enumerate() {
        let result = remove_kdigits(num, k);
        assert_eq!(result, results[i]);
    }
}
