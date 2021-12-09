use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{read_to_string, File};
use std::io::{prelude::*, BufReader};

fn day_01() {
    let file = File::open("01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut first = true;
    let mut counter = 0;
    let mut old = 0u64;
    let mut nums = Vec::<u64>::new();
    for line in reader.lines().flatten() {
        if let Ok(num) = line.parse::<u64>() {
            if first {
                first = false;
            } else if num > old {
                counter += 1;
            }
            nums.push(num);
            old = num;
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
    for line in reader.lines().flatten() {
        let words: Vec<&str> = line.split(' ').collect();
        match words[0] {
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
    println!("{} {}", pos.0 * pos.1, pos2.0 * pos2.1);
}

fn day_03() {
    let file = File::open("03/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut counters0 = Vec::<u64>::new();
    let mut counters1 = Vec::<u64>::new();
    let mut nums = Vec::<u64>::new();
    for line in reader.lines().flatten() {
        if counters0.is_empty() {
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
    let mut gamma = 0u64;
    let mut epsilon = 0u64;
    for i in 0..counters0.len() {
        if counters1[i] > counters0[i] {
            gamma |= 1 << (counters0.len() - i - 1);
        } else {
            epsilon |= 1 << (counters0.len() - i - 1);
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
    'outer1: for bit in 0..ox_counters0.len() {
        let mut temp = Vec::<u64>::new();
        for num in nums.iter() {
            if !set_oxygen.contains(num) {
                continue;
            }
            match ox_counters1[bit].cmp(&ox_counters0[bit]) {
                Ordering::Greater => {
                    if num & 1 << (ox_counters0.len() - bit - 1) == 0 {
                        set_oxygen.remove(num);
                        temp.push(*num);
                    }
                }
                Ordering::Less => {
                    if num & 1 << (ox_counters0.len() - bit - 1) != 0 {
                        set_oxygen.remove(num);
                        temp.push(*num);
                    }
                }
                Ordering::Equal => {
                    if num & 1 << (ox_counters0.len() - bit - 1) == 0 {
                        set_oxygen.remove(num);
                        temp.push(*num);
                    }
                }
            }
            if set_oxygen.len() == 1 {
                for e in set_oxygen.drain() {
                    oxygen = e;
                }
                break 'outer1;
            }
        }
        for n in temp {
            for i in 0..ox_counters0.len() {
                if n & (1 << (ox_counters0.len() - i - 1)) == 0 {
                    ox_counters0[i] -= 1;
                } else {
                    ox_counters1[i] -= 1;
                }
            }
        }
    }
    let mut co_counters1 = counters1.clone();
    let mut co_counters0 = counters0.clone();
    'outer2: for bit in 0..co_counters0.len() {
        let mut temp = Vec::<u64>::new();
        for num in nums.iter() {
            if !set_co2.contains(num) {
                continue;
            }
            match co_counters1[bit].cmp(&co_counters0[bit]) {
                Ordering::Greater => {
                    if num & (1 << (co_counters0.len() - bit - 1)) != 0 {
                        set_co2.remove(num);
                        temp.push(*num);
                    }
                }
                Ordering::Less => {
                    if num & (1 << (counters0.len() - bit - 1)) == 0 {
                        set_co2.remove(num);
                        temp.push(*num);
                    }
                }
                Ordering::Equal => {
                    if num & 1 << (co_counters0.len() - bit - 1) != 0 {
                        set_co2.remove(num);
                        temp.push(*num);
                    }
                }
            }
            if set_co2.len() == 1 {
                for e in set_co2.drain() {
                    co2 = e;
                }
                break 'outer2;
            }
        }
        for n in temp {
            for i in 0..co_counters0.len() {
                if n & (1 << (co_counters0.len() - i - 1)) == 0 {
                    co_counters0[i] -= 1;
                } else {
                    co_counters1[i] -= 1;
                }
            }
        }
    }
    println!("{} {}", gamma * epsilon, oxygen * co2);
}

fn day_04() {
    let file = File::open("04/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut nums = Vec::<u64>::new();
    let mut boards_n = Vec::<Vec<u64>>::new();
    let mut boards_c = Vec::<Vec<u64>>::new();
    let mut counter = 0u64;
    for (i, line) in reader.lines().flatten().enumerate() {
        if i == 0 {
            nums = line
                .split(',')
                .map(|s| s.parse::<u64>())
                .flatten()
                .collect::<Vec<u64>>();
        } else {
            if line.is_empty() {
                continue;
            }
            let row: Vec<u64> = line
                .split(' ')
                .map(|s| s.parse::<u64>())
                .flatten()
                .collect();
            if counter % 25 == 0 {
                boards_n.push(vec![0; 25]);
                boards_c.push(vec![0; 25]);
            }
            for val in row {
                let l = boards_n.len();
                boards_n[l - 1][(counter % 25) as usize] = val;
                boards_c[l - 1][(counter % 25) as usize] = 0;
                counter += 1;
            }
        }
    }
    let mut win1 = 0u64;
    let mut win2 = 0u64;
    let mut win3 = 0u64;
    let mut win4 = 0u64;
    let mut first = true;
    let mut wins = vec![0; boards_n.len()];
    'outer: for n in nums {
        for i in 0..boards_n.len() {
            for j in 0..25 {
                if boards_n[i][j] == n {
                    boards_c[i][j] = 1;
                }
            }
        }
        for i in 0..boards_n.len() {
            for j in 0..5 {
                let x: Vec<u64> = boards_c[i].iter().skip(j).step_by(5).copied().collect();
                let y: Vec<u64> = boards_c[i].iter().skip(j * 5).take(5).copied().collect();
                if x.iter().all(|a| *a == 1) || y.iter().all(|a| *a == 1) {
                    if first {
                        first = false;
                        win1 = n;
                        for k in 0..25 {
                            if boards_c[i][k] == 0 {
                                win2 += boards_n[i][k];
                            }
                        }
                    }
                    wins[i] = 1;
                    let s: u64 = wins.iter().sum();
                    if wins.len() - s as usize == 0 {
                        win3 = n;
                        for k in 0..25 {
                            if boards_c[i][k] == 0 {
                                win4 += boards_n[i][k];
                            }
                        }
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("{} {}", win1 * win2, win3 * win4);
}

fn day_05() {
    let file = File::open("05/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut paths = HashMap::<(u64, u64), u64>::new();
    let mut paths2 = HashMap::<(u64, u64), u64>::new();
    for line in reader.lines().flatten() {
        let dots: Vec<&str> = line.split("->").collect();
        let dot1: Vec<u64> = dots[0]
            .split(',')
            .map(|s| s.trim().parse::<u64>())
            .flatten()
            .collect();
        let dot2: Vec<u64> = dots[1]
            .split(',')
            .map(|s| s.trim().parse::<u64>())
            .flatten()
            .collect();
        if dot1[0] == dot2[0] {
            let mut v = [dot1[1], dot2[1]];
            v.sort_unstable();
            let r = v[0]..=v[1];
            for y in r {
                *paths.entry((dot1[0], y)).or_insert(0) += 1;
                *paths2.entry((dot1[0], y)).or_insert(0) += 1;
            }
        } else if dot1[1] == dot2[1] {
            let mut v = [dot1[0], dot2[0]];
            v.sort_unstable();
            let r = v[0]..=v[1];
            for x in r {
                *paths.entry((x, dot1[1])).or_insert(0) += 1;
                *paths2.entry((x, dot1[1])).or_insert(0) += 1;
            }
        } else {
            let dx = if dot1[0] < dot2[0] { 1 } else { -1 };
            let dy = if dot1[1] < dot2[1] { 1 } else { -1 };
            let mut x = dot1[0];
            let mut y = dot1[1];
            loop {
                *paths2.entry((x, y)).or_insert(0) += 1;
                if x == dot2[0] && y == dot2[1] {
                    break;
                }
                x = (x as i32 + dx) as u64;
                y = (y as i32 + dy) as u64;
            }
        }
    }
    let s = paths
        .values()
        .map(|x| if x > &1 { 1 } else { 0 })
        .sum::<u64>();
    let s2 = paths2
        .values()
        .map(|x| if x > &1 { 1 } else { 0 })
        .sum::<u64>();
    println!("{} {}", s, s2);
}

fn day_06() {
    let line = read_to_string("06/input.txt").unwrap();
    let mut fishes = VecDeque::<u64>::from(vec![0; 9]);
    line.trim()
        .split(',')
        .map(|s| s.parse::<u64>())
        .flatten()
        .for_each(|n| fishes[n as usize] += 1);
    for _ in 0..80u64 {
        let n = fishes.pop_front().unwrap();
        fishes[6] += n;
        fishes.push_back(n);
    }
    print!("{} ", fishes.iter().sum::<u64>());
    for _ in 80..256u64 {
        let n = fishes.pop_front().unwrap();
        fishes[6] += n;
        fishes.push_back(n);
    }
    println!("{}", fishes.iter().sum::<u64>());
}

fn day_07() {
    let line = read_to_string("07/input.txt").unwrap();
    let mut poss = VecDeque::<u64>::new();
    line.trim()
        .split(',')
        .map(|s| s.parse::<u64>())
        .flatten()
        .for_each(|n| poss.push_back(n));
    let max = *poss.iter().max().unwrap();
    let min = *poss.iter().min().unwrap();
    let mut min1 = u64::MAX;
    let mut min2 = u64::MAX;
    for i in min..=max {
        let mut sum = 0u64;
        for pos in poss.iter() {
            let n = i64::abs(*pos as i64 - i as i64) as u64;
            sum += n;
            if sum >= min1 {
                break;
            }
        }
        if sum < min1 {
            min1 = sum;
        }
    }
    for i in min..=max {
        let mut sum = 0u64;
        for pos in poss.iter() {
            let n = i64::abs(*pos as i64 - i as i64) as u64;
            sum += n * (n + 1) / 2;
            if sum >= min2 {
                break;
            }
        }
        if sum < min2 {
            min2 = sum;
        }
    }
    println!("{} {}", min1, min2);
}

fn day_08() {
    let includes = |a: &str, b: &str| -> bool {
        for c in a.chars() {
            if !b.contains(c) {
                return false;
            }
        }
        true
    };
    let file = File::open("08/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut counter = 0u64;
    let mut big_sum = 0u64;
    for line in reader.lines().flatten() {
        let mut d = HashMap::<u64, String>::new();
        let mut d2 = HashMap::<String, u64>::new();
        let chunks: Vec<&str> = line.split('|').collect();
        let digits: Vec<&str> = chunks[0].trim().split(' ').collect();
        for digit in digits.iter() {
            match digit.len() {
                2 => {
                    d.insert(1, digit.to_string());
                }
                3 => {
                    d.insert(7, digit.to_string());
                }
                4 => {
                    d.insert(4, digit.to_string());
                }
                7 => {
                    d.insert(8, digit.to_string());
                }
                _ => (),
            }
        }
        let mut ft_candidates = Vec::<String>::new();
        for digit in digits.iter() {
            match digit.len() {
                2..=4 | 7 => (),
                5 => {
                    if includes(&d[&1], digit) {
                        d.insert(3, digit.to_string());
                    } else if d.contains_key(&6) {
                        if includes(digit, &d[&6]) {
                            d.insert(5, digit.to_string());
                        } else {
                            d.insert(2, digit.to_string());
                        }
                    } else {
                        ft_candidates.push(digit.to_string());
                    }
                }
                6 => {
                    if includes(&d[&4], digit) {
                        d.insert(9, digit.to_string());
                    } else if includes(&d[&7], digit) {
                        d.insert(0, digit.to_string());
                    } else {
                        d.insert(6, digit.to_string());
                    }
                }
                _ => (),
            }
        }
        if !ft_candidates.is_empty() {
            for ftc in ft_candidates {
                if includes(&ftc, &d[&6]) {
                    d.insert(5, ftc);
                } else {
                    d.insert(2, ftc);
                }
            }
        }
        for (k, v) in d {
            let mut l: Vec<char> = v.chars().collect();
            l.sort_unstable();
            d2.insert(l.into_iter().collect(), k);
        }
        let nums: Vec<&str> = chunks[1].trim().split(' ').collect();
        let mut sum = 0u64;
        for num in nums {
            let mut l: Vec<char> = num.chars().collect();
            l.sort_unstable();
            let s: String = l.into_iter().collect();
            sum *= 10;
            sum += d2[&s];
            match num.len() {
                2..=4 | 7 => counter += 1,
                _ => (),
            }
        }
        big_sum += sum;
    }
    println!("{} {}", counter, big_sum);
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
    day_08();
}
