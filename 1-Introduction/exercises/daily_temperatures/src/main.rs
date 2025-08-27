// https://leetcode.com/problems/daily-temperatures/

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let temperatures_len = temperatures.len();
    if temperatures_len < 1 || temperatures_len > 10_usize.pow(5) {
        panic!("Temperatures length is out of bounds.");
    }

    for t in temperatures.iter() {
        if *t < 30 || *t > 100 {
            panic!("Temperature out of bounds: {t}");
        }
    }

    let mut answer = vec![0; temperatures_len];
    let mut stack: Vec<usize> = Vec::new();

    for (i, &temperature) in temperatures.iter().enumerate() {
        while let Some(&last_index) = stack.last() {
            if temperature > temperatures[last_index] {
                answer[last_index] = (i - last_index) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    answer
}

fn main() {
    let temperatures = vec![73,74,75,71,69,72,76,73];
    let answer = daily_temperatures(temperatures);
    assert_eq!(answer, [1,1,4,2,1,1,0,0]);

    let temperatures = vec![30,40,50,60];
    let answer = daily_temperatures(temperatures);
    assert_eq!(answer, [1,1,1,0]);

    let temperatures = vec![30,60,90];
    let answer = daily_temperatures(temperatures);
    assert_eq!(answer, [1,1,0]);

    let temperatures = vec![90,60,30];
    let answer = daily_temperatures(temperatures);
    assert_eq!(answer, [0,0,0]);
}
