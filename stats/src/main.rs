use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();

    let mut v = vec!();
    for _ in 0..100 {
        v.push(rng.gen_range(1..=10));
    }
    v.sort();

    println!("List is {:?}", &v);
    println!("Mean is: {}", mean(&v));
    println!("Median is: {}", median(&v));
    println!("Mode is: {}", mode(&v));
}

fn mean(v: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum as f32 / v.len() as f32
}

fn median(v: &Vec<i32>) -> f32 {
    let len = v.len();
    if len % 2 == 1 {
        return v[len/2] as f32;
    }
    (v[len/2] + v[len/2 + 1]) as f32 / 2 as f32
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for i in v {
        let count = hm.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (k, count) in hm {
        if count > max_count {
            mode = *k;
            max_count = count;
        }
    }
    mode
}
