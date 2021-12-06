use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day_01() {
    let file = File::open("01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut first = true;
    let mut counter = 0;
    let mut old = 0u64;
    let mut nums = Vec::<u64>::new();
    for line in reader.lines() {
        if let Ok(ip) = line {
            if let Ok(num) = ip.parse::<u64>() {
                if first {
                    first = false;
                } else {
                    if num > old {
                        counter += 1;
                    }
                }
                nums.push(num);
                old = num;
            }
        }
    }
    let mut counter2 = 0u64;
    for i in 0..nums.len() - 3 {
        let sum_a: u64 = (&nums[i..i + 3]).iter().sum();
        let sum_b: u64 = (&nums[i + 1..i + 4]).iter().sum();
        if sum_b > sum_a {
            counter2 += 1;
        }
    }
    println!("{} {}", counter, counter2);
}

fn day_02() {
    let file = File::open("02/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut pos = (0u64, 0u64);
    let mut pos2 = (0i64, 0i64);
    let mut aim = 0i64;
    for r in reader.lines() {
        if let Ok(line) = r {
            let words: Vec<&str> = line.split(" ").collect();
            match words[0].as_ref() {
                "forward" => {
                    pos.0 += words[1].parse::<u64>().unwrap();
                    pos2.0 += words[1].parse::<i64>().unwrap();
                    pos2.1 += aim * words[1].parse::<i64>().unwrap();
                }
                "up" => {
                    pos.1 -= words[1].parse::<u64>().unwrap();
                    aim -= words[1].parse::<i64>().unwrap();
                }
                "down" => {
                    pos.1 += words[1].parse::<u64>().unwrap();
                    aim += words[1].parse::<i64>().unwrap()
                }
                _ => (),
            }
        }
    }
    println!("{} {}", pos.0 * pos.1, pos2.0 * pos2.1);
}

fn day_03() {
    let file = File::open("03/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut counters0 = Vec::<u64>::new();
    let mut counters1 = Vec::<u64>::new();
    let mut nums = Vec::<u64>::new();
    for r in reader.lines() {
        if let Ok(line) = r {
            if counters0.len() == 0 {
                counters0.resize(line.len(), 0);
                counters1.resize(line.len(), 0);
            }
            for (i, c) in line.chars().enumerate() {
                match c {
                    '1' => counters1[i] += 1,
                    '0' => counters0[i] += 1,
                    _ => (),
                }
            }
            nums.push(u64::from_str_radix(&line, 2).unwrap());
        }
    }
    let mut gamma = 0u64;
    let mut epsilon = 0u64;
    for i in 0..counters0.len() {
        if counters1[i] > counters0[i] {
            gamma |= 1 << counters0.len() - i - 1;
        } else {
            epsilon |= 1 << counters0.len() - i - 1;
        }
    }
    let mut set_oxygen = HashSet::<u64>::new();
    let mut set_co2 = HashSet::<u64>::new();
    let mut oxygen = 0u64;
    let mut co2 = 0u64;
    for num in nums.iter() {
        set_oxygen.insert(*num);
        set_co2.insert(*num);
    }
    let mut ox_counters1 = counters1.clone();
    let mut ox_counters0 = counters0.clone();
    let mut out = false;
    for bit in 0..ox_counters0.len() {
        let mut temp= Vec::<u64>::new();
        if out {
            break;
        }
        for num in nums.iter() {
            if !set_oxygen.contains(num) {
                continue;
            }
            if ox_counters1[bit] > ox_counters0[bit] {
                if num & (1 << ox_counters0.len() - bit - 1) == 0 {
                    set_oxygen.remove(num);
                    temp.push(*num);
                   
                }
            } else if ox_counters0[bit] > ox_counters1[bit] {
                if num & (1 << ox_counters0.len() - bit - 1) != 0 {
                    set_oxygen.remove(num);
                    temp.push(*num);
                }
            } else if ox_counters1[bit] == ox_counters0[bit] {
                if num & 1 << ox_counters0.len() - bit - 1 == 0 {
                    set_oxygen.remove(num);
                    temp.push(*num);
                }
            }
            if set_oxygen.len() == 1 {
                for e in set_oxygen.drain() {
                    oxygen = e;
                }
                out = true;
                break;
            }
        }
        for n in temp {
            for i in 0..ox_counters0.len() {
                if n & (1 << ox_counters0.len() - i - 1) == 0 {
                    ox_counters0[i] -= 1;
                } else {
                    ox_counters1[i] -= 1;
                }
            }
        }
    }
    let mut co_counters1 = counters1.clone();
    let mut co_counters0 = counters0.clone();
    out = false;
    for bit in 0..co_counters0.len() {
        let mut temp= Vec::<u64>::new();
        for num in nums.iter() {
            if out {
                break;
            }
            if !set_co2.contains(num) {
                continue;
            }
            if co_counters0[bit] < co_counters1[bit] {
                if num & (1 << co_counters0.len() - bit - 1) != 0 {
                    set_co2.remove(num);
                    temp.push(*num);
                    
                }
            } else if co_counters1[bit] < co_counters0[bit] {
                if num & (1 << counters0.len() - bit - 1) == 0 {
                    set_co2.remove(num);
                    temp.push(*num);
                }
            } else if co_counters1[bit] == co_counters0[bit] {
                if num & 1 << co_counters0.len() - bit - 1 != 0 {
                    set_co2.remove(num);
                    temp.push(*num);
                }
            }
            if set_co2.len() == 1 {
                for e in set_co2.drain() {
                    co2 = e;
                }
                out = false;
                break;
            }
        }
        for n in temp {
            for i in 0..co_counters0.len() {
                if n & (1 << co_counters0.len() - i - 1) == 0 {
                    co_counters0[i] -= 1;
                } else {
                    co_counters1[i] -= 1;
                }
            }
        }
    }
    println!("{} {}", gamma * epsilon, oxygen * co2);
}

fn main() {
    day_01();
    day_02();
    day_03();
}
