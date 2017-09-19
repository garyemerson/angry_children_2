use std::io;

fn main() {
    let n = get_line().trim().parse::<usize>().unwrap();
    let k = get_line().trim().parse::<usize>().unwrap();
    // println!("n: {}; k: {}", n, k);

    let mut packets = Vec::with_capacity(n);
    for _ in 0..n {
        packets.push(get_line().trim().parse::<i64>().unwrap());
    }
    packets.sort();
    // println!("packets is {:?}", packets);

    // A the element at index i is the sum of all elements before xi and xi
    // (where xi is element at index i in packets).
    let mut sums = Vec::with_capacity(n);
    sums.push(packets[0]);
    for i in 1..(packets.len()) {
        let new = sums[i - 1] + packets[i];
        sums.push(new);
    }
    // println!("sums is {:?}", sums);

    let mut min_unfair = get_inital_unfairness(&packets, k);
    let mut min_unfair_slow = 0;//get_unfair_slow(&packets, 0, k - 1);
    // println!("min_unfair: {}; min_unfair_slow: {}", min_unfair, min_unfair_slow);

    let mut prev_unfair = min_unfair;
    for start in 1..(n - k + 1) {
        let new_unfair =
              log("new part    ", (((k - 1) as i64) * packets[start + k - 1]) - sum(&sums, start, start + k - 2))
            + log("prev_unfair ", prev_unfair)
            - log("old part    ", (sum(&sums, start, start + k - 2) - ((k - 1) as i64) * packets[start - 1]));
        let new_unfair_slow = 0;//get_unfair_slow(&packets, start, start + k - 1);
        // println!("unfair      {} to {} is {}", start, start + k - 1, new_unfair);
        // println!("unfair_slow {} to {} is {}", start, start + k - 1, new_unfair_slow);
        if new_unfair_slow < min_unfair_slow {
            min_unfair_slow = new_unfair_slow;
        }
        prev_unfair = new_unfair;
        // println!("comparing new_unfair({}) with min_unfair({})", new_unfair, min_unfair);
        if new_unfair < min_unfair {
            min_unfair = new_unfair;
        }
    }
    // println!("min_unfair      {}", min_unfair);
    // println!("min_unfair_slow {}", min_unfair_slow);
    println!("{}", min_unfair);
}

fn log<T: std::fmt::Debug>(msg: &str, val: T) -> T {
    // println!("{}: {:?}", msg, val);
    val
}

fn get_unfair_slow(packets: &Vec<i64>, start: usize, end: usize) -> i64 {
    let mut unfair = 0;
    for i in start..end {
        for j in (i + 1)..(end + 1) {
            unfair += (packets[i] - packets[j]).abs();
        }
    }
    unfair
}

fn sum(sums: &Vec<i64>, start: usize, end: usize) -> i64 {
    // println!("getting sums from {} to {}", start, end);
    if start == 0 {
        return sums[end];
    } else {
        return sums[end] - sums[start - 1];
    }
}

fn get_inital_unfairness(packets: &Vec<i64>, k: usize) -> i64 {
    let mut unfairness = 0;
    for dist in 1..k {
        // println!("using dist {}", dist);
        for start in 0..(k - dist) {
            // println!("diffing indices {} and {}", start + dist, start);
            unfairness += packets[start + dist]  - packets[start];
        }
    }
    unfairness
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
