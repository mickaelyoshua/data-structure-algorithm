// https://leetcode.com/problems/counting-bits/

fn count_bits_hard_code(n: i32) -> Vec<i32> {
    let mut ans = vec![0; n as usize + 1];
    let mut count = 0;
    for (i, mut number) in (0..=n).enumerate() {
        while number > 0 {
            if number % 2 == 1 {
                count += 1;
            }
            number /= 2;
        }
        ans[i] = count;
        count = 0;
    }
    ans
}

// Explanation:
// - The operation `i / 2` is equivalent to a right bit shift (`i >> 1`), which removes the last bit
//   from the number's binary representation.
// - Because of this, the number of set bits in `i` is simply the number of set bits
//   in `i / 2`, plus the value of the last bit that was removed.
// - The term `(i & 1)` is a fast and efficient way to get this last bit (it results in 1 if `i`
//   is odd and 0 if `i` is even).
fn count_bits_smart(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n + 1];
        for i in 1..=n {
            ans[i] = ans[i / 2] + (i & 1) as i32;
        }
        ans
}

fn count_bits_rust(n: i32) -> Vec<i32> {
    (0..=n).map(|number| number.count_ones() as i32).collect()
}

fn main() {
    let inputs: [i32;2] = [
        2,
        5
    ];
    let outputs: Vec<Vec<i32>> = vec![
        vec![0, 1, 1],
        vec![0, 1, 1, 2, 1, 2]
    ];

    println!("Hard coded");
    for (input, output) in inputs.iter().zip(&outputs) {
        let result = count_bits_hard_code(*input);
        assert_eq!(result, *output);
    }

    println!("Smart");
    for (input, output) in inputs.iter().zip(&outputs) {
        let result = count_bits_smart(*input);
        assert_eq!(result, *output);
    }

    println!("Rustacean");
    for (input, output) in inputs.iter().zip(&outputs) {
        let result = count_bits_rust(*input);
        assert_eq!(result, *output);
    }
}
