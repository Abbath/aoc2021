use std::cmp::{max, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs::{read_to_string, File};
use std::io::{prelude::*, BufReader};
use std::iter::from_fn;
use std::ops::Range;
use std::u64;

fn day_01() {
    let file = File::open("01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let nums: Vec<u64> = reader
        .lines()
        .flatten()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let count = |ns: &Vec<u64>, skip| {
        ns.iter()
            .zip(ns.iter().skip(skip))
            .fold(0, |c, (&x, &y)| if x < y { c + 1 } else { c })
    };
    println!("{} {}", count(&nums, 1), count(&nums, 3));
}

fn day_02() {
    let file = File::open("02/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut pos = (0i64, 0i64);
    let mut pos2 = (0i64, 0i64);
    let mut aim = 0i64;
    for line in reader.lines().flatten() {
        let words: Vec<&str> = line.split(' ').collect();
        let val = words[1].parse::<i64>().unwrap();
        match words[0] {
            "forward" => {
                pos.0 += val;
                pos2.0 += val;
                pos2.1 += aim * val;
            }
            "up" => {
                pos.1 -= val;
                aim -= val;
            }
            "down" => {
                pos.1 += val;
                aim += val;
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
    let nums = reader
        .lines()
        .flatten()
        .map(|line| {
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
            u64::from_str_radix(&line, 2).unwrap()
        })
        .collect();
    let len = counters0.len();
    let mut ge = [0u64, 0u64];
    counters0
        .iter()
        .zip(counters1.iter())
        .enumerate()
        .for_each(|(i, (x, y))| {
            ge[if y > x { 0 } else { 1 }] |= 1 << (len - i - 1);
        });
    let compute = |nums: &Vec<u64>, c0: &Vec<u64>, c1: &Vec<u64>, f: fn(u64) -> bool| {
        let mut res = 0u64;
        let mut set = HashSet::<u64>::new();
        let mut counters1 = c1.clone();
        let mut counters0 = c0.clone();
        let len = counters0.len();
        for &num in nums.iter() {
            set.insert(num);
        }
        'outer: for bit in 0..counters0.len() {
            let mut temp = Vec::<u64>::new();
            for num in nums.iter() {
                if !set.contains(num) {
                    continue;
                }
                match counters1[bit].cmp(&counters0[bit]) {
                    Ordering::Greater => {
                        if f(num & 1 << (len - bit - 1)) {
                            set.remove(num);
                            temp.push(*num);
                        }
                    }
                    Ordering::Less => {
                        if !f(num & 1 << (len - bit - 1)) {
                            set.remove(num);
                            temp.push(*num);
                        }
                    }
                    Ordering::Equal => {
                        if f(num & 1 << (len - bit - 1)) {
                            set.remove(num);
                            temp.push(*num);
                        }
                    }
                }
                if set.len() == 1 {
                    for e in set.drain() {
                        res = e;
                    }
                    break 'outer;
                }
            }
            for n in temp {
                for i in 0..len {
                    if n & (1 << (len - i - 1)) == 0 {
                        counters0[i] -= 1;
                    } else {
                        counters1[i] -= 1;
                    }
                }
            }
        }
        res
    };
    println!(
        "{} {}",
        ge[0] * ge[1],
        compute(&nums, &counters0, &counters1, |x| x == 0)
            * compute(&nums, &counters0, &counters1, |x| x != 0)
    );
}

fn day_04() {
    let file = File::open("04/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut nums = Vec::<u64>::new();
    let mut boards_n = Vec::<Vec<u64>>::new();
    let mut boards_c = Vec::<Vec<u64>>::new();
    let mut counter = 0u64;
    let parse = |line: &str, delim: char| -> Vec<u64> {
        line.split(delim)
            .map(|s| s.parse::<u64>())
            .flatten()
            .collect()
    };
    for (i, line) in reader.lines().flatten().enumerate() {
        if i == 0 {
            nums = parse(&line, ',');
        } else {
            if line.is_empty() {
                continue;
            }
            let row = parse(&line, ' ');
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
        let f = |i: usize| {
            boards_c[i]
                .iter()
                .enumerate()
                .filter(|(_, &v)| v == 0)
                .map(|(k, _)| boards_n[i][k])
                .sum::<u64>()
        };
        for i in 0..boards_n.len() {
            for j in 0..5 {
                let x: bool = boards_c[i].iter().skip(j).step_by(5).all(|a| *a == 1);
                let y: bool = boards_c[i].iter().skip(j * 5).take(5).all(|a| *a == 1);
                if x || y {
                    if first {
                        first = false;
                        print!("{} ", n * f(i));
                    }
                    wins[i] = 1;
                    let s: u64 = wins.iter().sum();
                    if wins.len() - s as usize == 0 {
                        println!("{}", n * f(i));
                        break 'outer;
                    }
                }
            }
        }
    }
}

fn day_05() {
    let file = File::open("05/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut paths = HashMap::<(u64, u64), u64>::new();
    let mut paths2 = HashMap::<(u64, u64), u64>::new();
    for line in reader.lines().flatten() {
        let dots: Vec<Vec<u64>> = line
            .split("->")
            .map(|dot| {
                dot.split(',')
                    .map(|s| s.trim().parse::<u64>())
                    .flatten()
                    .collect()
            })
            .collect();
        let dot1 = &dots[0];
        let dot2 = &dots[1];
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
    let f =
        |p: &HashMap<(u64, u64), u64>| p.values().map(|x| if x > &1 { 1 } else { 0 }).sum::<u64>();
    println!("{} {}", f(&paths), f(&paths2));
}

fn day_06() {
    let line = read_to_string("06/input.txt").unwrap();
    let mut fishes = VecDeque::<u64>::from(vec![0; 9]);
    line.trim()
        .split(',')
        .map(|s| s.parse::<u64>())
        .flatten()
        .for_each(|n| fishes[n as usize] += 1);
    for n in 0..256u64 {
        if n == 80 {
            print!("{} ", fishes.iter().sum::<u64>());
        }
        if let Some(n) = fishes.pop_front() {
            fishes[6] += n;
            fishes.push_back(n);
        }
    }
    println!("{}", fishes.iter().sum::<u64>());
}

fn day_07() {
    let line = read_to_string("07/input.txt").unwrap();
    let poss: VecDeque<u64> = line
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>())
        .flatten()
        .collect();
    let max = *poss.iter().max().unwrap();
    let min = *poss.iter().min().unwrap();
    let compute = |f: fn(u64) -> u64| {
        let mut min0 = u64::MAX;
        for i in min..=max {
            let mut sum = 0u64;
            for pos in poss.iter() {
                let n = i64::abs(*pos as i64 - i as i64) as u64;
                sum += f(n);
                if sum >= min0 {
                    break;
                }
            }
            if sum < min0 {
                min0 = sum;
            }
        }
        min0
    };
    println!("{} {}", compute(|n| n), compute(|n| n * (n + 1) / 2));
}

fn day_08() {
    let intersect = |a: &str, b: &str| -> i32 {
        let mut res = 0;
        for c in a.chars() {
            if b.contains(c) {
                res += 1;
            }
        }
        res
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
        for digit in digits.iter() {
            match digit.len() {
                2..=4 | 7 => (),
                5 => {
                    d.insert(
                        if intersect(&d[&1], digit) == 2 {
                            3
                        } else if intersect(&d[&4], digit) == 3 {
                            5
                        } else {
                            2
                        },
                        digit.to_string(),
                    );
                }
                6 => {
                    d.insert(
                        if intersect(&d[&4], digit) == 4 {
                            9
                        } else if intersect(&d[&7], digit) == 3 {
                            0
                        } else {
                            6
                        },
                        digit.to_string(),
                    );
                }
                _ => (),
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

fn day_09() {
    let file = File::open("09/input.txt").unwrap();
    let reader = BufReader::new(file);
    let field: Vec<Vec<u64>> = reader
        .lines()
        .flatten()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).map(|n| n as u64))
                .flatten()
                .collect::<Vec<u64>>()
        })
        .collect();
    let mut sum = 0u64;
    let low_points: Vec<((usize, usize), u64)> = (0..field.len())
        .map(|i| {
            (0..field[0].len())
                .map(|j| {
                    let mut down = true;
                    if i > 0 {
                        down &= field[i][j] < field[i - 1][j];
                    }
                    if j > 0 {
                        down &= field[i][j] < field[i][j - 1];
                    }
                    if i < field.len() - 1 {
                        down &= field[i][j] < field[i + 1][j];
                    }
                    if j < field[0].len() - 1 {
                        down &= field[i][j] < field[i][j + 1];
                    }
                    if down {
                        sum += field[i][j] + 1;
                        Some(((i, j), field[i][j]))
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<((usize, usize), u64)>>()
        })
        .flatten()
        .collect();
    let mut basins = Vec::<u64>::new();
    for low_point in low_points {
        let mut size = 1u64;
        let mut stack = VecDeque::<((usize, usize), u64)>::new();
        let mut basin = HashSet::<(usize, usize)>::new();
        basin.insert(low_point.0);
        stack.push_back(low_point);
        while let Some(((i, j), v)) = stack.pop_back() {
            let mut neighbours = Vec::<(usize, usize)>::new();
            if i > 0 {
                neighbours.push((i - 1, j));
            }
            if j > 0 {
                neighbours.push((i, j - 1));
            }
            if i < field.len() - 1 {
                neighbours.push((i + 1, j));
            }
            if j < field[0].len() - 1 {
                neighbours.push((i, j + 1));
            }
            for (m, n) in neighbours {
                if i > 0 && v < field[m][n] && !basin.contains(&(m, n)) && field[m][n] < 9 {
                    size += 1;
                    stack.push_back(((m, n), field[m][n]));
                    basin.insert((m, n));
                }
            }
        }
        basins.push(size);
    }
    basins.sort_unstable();
    println!("{} {}", sum, basins.iter().rev().take(3).product::<u64>());
}

fn day_10() {
    let file = File::open("10/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score = 0u64;
    let mut scores = Vec::<u64>::new();
    let w = HashMap::from([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]);
    let w2 = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    for line in reader.lines().flatten() {
        let mut stack = Vec::<char>::new();
        for char in line.trim().chars() {
            match char {
                '{' | '[' | '(' | '<' => stack.push(char),
                '}' | ']' | ')' | '>' => {
                    if stack.is_empty() {
                        break;
                    } else if stack.last().unwrap() == &w[&char].0 {
                        stack.pop();
                    } else {
                        stack.clear();
                        score += w[&char].1;
                        break;
                    }
                }
                _ => (),
            }
        }
        if !stack.is_empty() {
            let mut score2 = 0u64;
            while let Some(c) = stack.pop() {
                score2 *= 5;
                score2 += w2[&c];
            }
            scores.push(score2);
        }
    }
    scores.sort_unstable();
    println!("{} {}", score, scores[scores.len() / 2]);
}

fn day_11() {
    let find_homies = |(i, j), rows: usize, cols: usize| {
        let mut neigbours = VecDeque::<(usize, usize)>::new();
        if i > 0 {
            neigbours.push_back((i - 1, j));
            if j > 0 {
                neigbours.push_back((i - 1, j - 1));
            }
        }
        if j > 0 {
            neigbours.push_back((i, j - 1));
            if i < rows - 1 {
                neigbours.push_back((i + 1, j - 1));
            }
        }
        if i < rows - 1 {
            neigbours.push_back((i + 1, j));
            if j < cols - 1 {
                neigbours.push_back((i + 1, j + 1));
            }
        }
        if j < cols - 1 {
            neigbours.push_back((i, j + 1));
            if i > 0 {
                neigbours.push_back((i - 1, j + 1));
            }
        }
        neigbours
    };
    let file = File::open("11/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut field: Vec<Vec<u64>> = reader
        .lines()
        .flatten()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).map(|n| n as u64))
                .flatten()
                .collect::<Vec<u64>>()
        })
        .collect();
    let mut flashed = Vec::<bool>::new();
    flashed.resize(field.len() * field[0].len(), false);
    let mut flashes = 0u64;
    let rows = field.len();
    let cols = field[0].len();
    for step in 0..1000 {
        let sum: u64 = field.iter().map(|v| v.iter().sum::<u64>()).sum();
        if sum == 0 {
            println!("{}", step);
            break;
        }
        for i in 0..rows {
            for j in 0..cols {
                field[i][j] += 1;
                if field[i][j] > 9 && !flashed[i * cols + j] {
                    flashed[i * cols + j] = true;
                    flashes += 1;
                    let mut stack = VecDeque::<(usize, usize)>::new();
                    stack.append(&mut find_homies((i, j), rows, cols));
                    while let Some((m, n)) = stack.pop_front() {
                        field[m][n] += 1;
                        if !flashed[m * cols + n] && field[m][n] > 9 {
                            flashed[m * cols + n] = true;
                            flashes += 1;
                            stack.append(&mut find_homies((m, n), rows, cols));
                        }
                    }
                }
            }
        }
        for i in 0..rows {
            for j in 0..cols {
                if flashed[i * cols + j] {
                    flashed[i * cols + j] = false;
                    field[i][j] = 0;
                }
            }
        }
        if step == 99 {
            print!("{} ", flashes);
        }
    }
}

fn day_12() {
    let file = File::open("12/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vert_map = HashMap::<String, u64>::new();
    let mut counter = 0u64;
    let mut map_vert = HashMap::<u64, bool>::new();
    let graph: HashSet<(u64, u64)> = reader
        .lines()
        .flatten()
        .map(|line| {
            let v: Vec<&str> = line.trim().split('-').collect();
            let v1 = v[0];
            let v2 = v[1];
            let idx1 = if !vert_map.contains_key(v1) {
                vert_map.insert(v1.to_string(), counter);
                map_vert.insert(counter, v1.to_uppercase().eq(v1));
                counter += 1;
                counter - 1
            } else {
                vert_map[v1]
            };
            let idx2 = if !vert_map.contains_key(v2) {
                vert_map.insert(v2.to_string(), counter);
                map_vert.insert(counter, v2.to_uppercase().eq(v2));
                counter += 1;
                counter - 1
            } else {
                vert_map[v2]
            };
            (idx1, idx2)
        })
        .collect();
    let start = vert_map["start"];
    let end = vert_map["end"];
    let mut matrix = Vec::<u64>::new();
    matrix.resize((counter * counter) as usize, 0);
    for (i, j) in graph {
        matrix[(i * counter + j) as usize] = 1;
        matrix[(j * counter + i) as usize] = 1;
    }
    fn traverse2(
        v: u64,
        p: HashSet<u64>,
        counter: u64,
        (m, s, e): (&Vec<u64>, u64, u64),
        mv: &HashMap<u64, bool>,
        dup: Option<u64>,
        is_dup: bool,
    ) -> u64 {
        if v == e {
            return 1;
        }
        let mut res = 0u64;
        for i in 0..counter {
            if m[(v * counter + i) as usize] == 1 {
                if i == s {
                    continue;
                }
                let mut d = dup;
                if !mv[&v] && p.contains(&v) {
                    if is_dup {
                        match dup {
                            None => d = Some(v),
                            Some(_) => continue,
                        }
                    } else {
                        continue;
                    }
                }
                let mut path = p.clone();
                path.insert(v);
                res += traverse2(i, path, counter, (m, s, e), mv, d, is_dup);
            }
        }
        res
    }
    let t = |is_dup: bool| {
        traverse2(
            start,
            HashSet::<u64>::new(),
            counter,
            (&matrix, start, end),
            &map_vert,
            None,
            is_dup,
        )
    };
    println!("{} {}", t(false), t(true));
}

fn day_13() {
    let file = File::open("13/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points = Vec::<(u64, u64)>::new();
    let mut folds = Vec::<(u64, u64)>::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in reader.lines().flatten().filter(|s| !s.is_empty()) {
        if line.starts_with("fold") {
            let f: Vec<&str> = line.trim().split(' ').nth(2).unwrap().split('=').collect();
            folds.push((if f[0] == "x" { 0 } else { 1 }, f[1].parse().unwrap()));
        } else {
            let v: Vec<u64> = line
                .trim()
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            max_x = max(max_x, v[0]);
            max_y = max(max_y, v[1]);
            points.push((v[0], v[1]));
        }
    }
    let rows = max_y as usize + 1;
    let cols = max_x as usize + 1;
    let mut src = [cols, rows];
    let mut board: Vec<_> = vec![0; rows * cols];
    for (x, y) in points {
        board[y as usize * cols + x as usize] = 1;
    }
    for (n, f) in folds.iter().enumerate() {
        let fold = (f.0 as usize, f.1 as usize);
        for y in (if fold.0 == 1 { fold.1 + 1 } else { 0 })..src[1] {
            for x in (if fold.0 == 0 { fold.1 + 1 } else { 0 })..src[0] {
                if board[y * cols + x] == 1 {
                    if fold.0 == 0 {
                        board[y * cols + (2 * fold.1 - x)] = 1;
                        board[y * cols + x] = 0;
                    } else {
                        board[(2 * fold.1 - y) * cols + x] = 1;
                        board[y * cols + x] = 0;
                    }
                }
            }
        }
        src[fold.0] = fold.1;
        if n == 0 {
            println!("{}", board.iter().sum::<u64>());
        }
    }
    for y in 0..src[1] {
        for x in 0..src[0] {
            print!("{}", if board[y * cols + x] == 1 { '#' } else { '.' });
        }
        println!();
    }
}

fn day_14() {
    let file = File::open("14/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut old_sequence: String = String::from("");
    let mut counters = HashMap::<char, u64>::new();
    let (transitions, mut pair_counters): (HashMap<String, char>, HashMap<String, u64>) = reader
        .lines()
        .flatten()
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                old_sequence = line.trim().to_string();
                None
            } else {
                let kv: Vec<&str> = line.trim().split("->").collect();
                let pair = kv[0].trim().to_string();
                Some((
                    (pair.clone(), kv[1].trim().chars().next().unwrap()),
                    (pair, 0),
                ))
            }
        })
        .flatten()
        .unzip();
    for c in old_sequence.chars() {
        *counters.entry(c).or_insert(0) += 1;
    }
    for i in 0..old_sequence.len() - 1 {
        *pair_counters.get_mut(&old_sequence[i..=i + 1]).unwrap() += 1;
    }
    for n in 0..40 {
        if n == 10 {
            print!(
                "{} ",
                counters.values().max().unwrap() - counters.values().min().unwrap()
            );
        }
        let mut to_add = HashMap::<String, u64>::new();
        for (k, v) in pair_counters.iter() {
            if v > &0 {
                *counters.entry(transitions[k]).or_insert(0) += v;
                *to_add
                    .entry(
                        [k.chars().next().unwrap(), transitions[k]]
                            .iter()
                            .collect::<String>(),
                    )
                    .or_insert(0) += v;
                *to_add
                    .entry(
                        [transitions[k], k.chars().nth(1).unwrap()]
                            .iter()
                            .collect::<String>(),
                    )
                    .or_insert(0) += v;
            }
        }
        pair_counters = to_add;
    }
    println!(
        "{}",
        counters.values().max().unwrap() - counters.values().min().unwrap()
    );
}

fn day_15() {
    let file = File::open("15/input.txt").unwrap();
    let reader = BufReader::new(file);
    let field: Vec<Vec<usize>> = reader
        .lines()
        .flatten()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).map(|n| n as usize))
                .flatten()
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut big_field = Vec::<Vec<usize>>::new();
    big_field.resize(field.len() * 5, vec![0; field[0].len() * 5]);
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            for m in 0..5 {
                for n in 0..5 {
                    let val = field[i][j] + m + n;
                    big_field[field.len() * m + i][n * field[0].len() + j] =
                        if val <= 9 { val } else { val - 9 }
                }
            }
        }
    }
    let neighbours = |field: &Vec<Vec<usize>>, idx: usize| -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let i = idx / field[0].len();
        let j = idx % field[0].len();
        if i > 0 {
            res.push(((i - 1) * field[0].len() + j, field[i - 1][j]));
        }
        if j > 0 {
            res.push((i * field[0].len() + j - 1, field[i][j - 1]));
        }
        if i < field.len() - 1 {
            res.push(((i + 1) * field[0].len() + j, field[i + 1][j]));
        }
        if j < field[0].len() - 1 {
            res.push((i * field[0].len() + j + 1, field[i][j + 1]));
        }
        res
    };
    let astar = |field: &Vec<Vec<usize>>| {
        let h = |u: usize| field[0].len() - u % field[0].len() + field.len() - u / field[0].len();
        let rows = field.len() * field[0].len();
        let mut open_set = BinaryHeap::new();
        open_set.push((Reverse(0), 0));
        let mut score = vec![usize::MAX; rows];
        score[0] = 0;
        while let Some((Reverse(_), idx)) = open_set.pop() {
            if idx == rows - 1 {
                break;
            }
            for (k, v) in neighbours(field, idx) {
                let new_score = score[idx] + v;
                if new_score < score[k] {
                    score[k] = new_score;
                    open_set.push((Reverse(new_score + h(k)), k));
                }
            }
        }
        score[rows - 1]
    };
    println!("{} {}", astar(&field), astar(&big_field));
}

fn day_16() {
    let line = read_to_string("16/input.txt").unwrap().trim().to_string();
    let bits: Vec<char> = line
        .chars()
        .map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<char>>()
        })
        .flatten()
        .collect();
    fn parse_packet(bits: &[char], offset: &mut usize, versions: &mut u16) -> u64 {
        let mut b2s = |n| {
            let v = u16::from_str_radix(&String::from_iter(bits[*offset..*offset + n].iter()), 2)
                .unwrap();
            *offset += n;
            v
        };
        let version: u16 = b2s(3);
        *versions += version;
        let type_id: u16 = b2s(3);
        match type_id {
            4 => {
                let mut sum = 0u64;
                loop {
                    let b: u16 = b2s(5);
                    sum = (sum << 4) | b as u64 & 0xf;
                    if b & 0x10 == 0 {
                        break;
                    }
                }
                sum
            }
            _ => {
                let len_type_id = b2s(1);
                let values: Vec<u64> = if len_type_id == 0 {
                    let total_len = b2s(15);
                    let old_offset = *offset;
                    from_fn(move || {
                        if *offset - old_offset < total_len as usize {
                            Some(parse_packet(bits, offset, versions))
                        } else {
                            None
                        }
                    })
                    .collect()
                } else {
                    let sub_num = b2s(11);
                    (0..sub_num)
                        .map(|_| parse_packet(bits, offset, versions))
                        .collect()
                };
                match type_id {
                    0 => values.iter().sum::<u64>(),
                    1 => values.iter().product::<u64>(),
                    2 => *values.iter().min().unwrap(),
                    3 => *values.iter().max().unwrap(),
                    5 => (values[0] > values[1]) as u64,
                    6 => (values[0] < values[1]) as u64,
                    7 => (values[0] == values[1]) as u64,
                    _ => 0,
                }
            }
        }
    }
    let mut offset: usize = 0;
    let mut versions: u16 = 0;
    let val = parse_packet(&bits, &mut offset, &mut versions);
    println!("{} {}", versions, val);
}

fn day_17() {
    let line = read_to_string("17/input.txt").unwrap().trim().to_string();
    let coords: Vec<i64> = line
        .trim()
        .split(' ')
        .skip(2)
        .map(|s| s.split('=').skip(1))
        .flatten()
        .map(|s| s.split(".."))
        .flatten()
        .map(|s| {
            if s.ends_with(',') {
                s[0..s.len() - 1].parse().unwrap()
            } else {
                s.parse().unwrap()
            }
        })
        .collect();
    let x_range = Range {
        start: coords[0],
        end: coords[1] + 1,
    };
    let y_range = Range {
        start: coords[2],
        end: coords[3] + 1,
    };
    let mut sum = 0u64;
    let mut highest = i64::MIN;
    for i in 0..x_range.end * 2 {
        for j in 0..i64::abs(y_range.start * 2) {
            let mut vel = (i, j - i64::abs(y_range.start));
            let mut pos = (0i64, 0i64);
            let mut local_highest = i64::MIN;
            let mut was_there = false;
            loop {
                pos.0 += vel.0;
                pos.1 += vel.1;
                if pos.0 > x_range.end || pos.1 < y_range.start {
                    break;
                }
                vel.0 -= vel.0.signum();
                vel.1 -= 1;
                local_highest = max(pos.1, local_highest);
                if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
                    if !was_there {
                        was_there = true;
                        sum += 1;
                    }
                    if local_highest > highest {
                        highest = local_highest;
                        break;
                    }
                }
            }
        }
    }
    println!("{} {}", highest, sum);
}

fn day_18() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Pair {
        Dot(Box<Pair>, Box<Pair>),
        Num(u64),
    }
    enum Action {
        Split(usize),
        Explode(usize, u64, u64),
    }
    impl Display for Pair {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Dot(a, b) => write!(f, "[{},{}]", a, b),
                Num(n) => write!(f, "{}", n),
            }
        }
    }
    use Action::{Explode, Split};
    use Pair::{Dot, Num};
    let file = File::open("18/input.txt").unwrap();
    let reader = BufReader::new(file);
    fn parse_int(s: &[char], offset: usize) -> (Pair, usize) {
        let mut res = String::from("");
        let mut i = offset;
        let mut exit = false;
        loop {
            if i == s.len() {
                i -= 1;
                break;
            }
            let c = s[i];
            if c.is_digit(10) {
                if exit {
                    break;
                }
                res.push(c);
            }
            if c == '[' && exit {
                break;
            }
            if c == ',' {
                exit = true;
            }
            if c == ']' {
                break;
            }
            i += 1;
        }
        (Num(res.parse().unwrap()), i)
    }
    fn skip(s: &[char], offset: usize) -> usize {
        let mut exit = false;
        for i in 0.. {
            if offset + i == s.len() {
                return 0;
            }
            if exit && (s[offset + i].is_digit(10) || s[offset + i] == '[' || s[offset + i] == ']')
            {
                return i;
            }
            if s[offset + i] == ']' {
                exit = true;
            }
        }
        0
    }
    fn parse_dot(s: &[char], offset: usize) -> (Pair, usize) {
        let mut new_offset = offset + 1;
        if s[offset] == '[' {
            let first = if s[new_offset] == '[' {
                let (p1, o1) = parse_dot(s, new_offset);
                new_offset = o1;
                p1
            } else {
                let (n1, o1) = parse_int(s, new_offset);
                new_offset = o1;
                n1
            };
            let second = if s[new_offset] == '[' {
                let (p2, o2) = parse_dot(s, new_offset);
                new_offset = o2 + skip(s, o2);
                p2
            } else {
                let (n2, o2) = parse_int(s, new_offset);
                new_offset = o2 + skip(s, o2);
                n2
            };
            (Dot(Box::new(first), Box::new(second)), new_offset)
        } else {
            panic!("ZHEPA!");
        }
    }
    fn check_explode(d: &Pair, depth: usize, n: usize) -> (Pair, usize, Option<Action>) {
        match d {
            Dot(a, b) => {
                if let Dot(o, p) = a.as_ref() {
                    if depth == 3 {
                        if let Num(x) = o.as_ref() {
                            if let Num(y) = p.as_ref() {
                                return (Num(0), 0, Some(Explode(n, *x, *y)));
                            }
                        }
                    }
                }
                let (c, n1, e1) = check_explode(a, depth + 1, n);
                if let Dot(o, p) = b.as_ref() {
                    if depth == 3 {
                        if let Num(x) = o.as_ref() {
                            if let Num(y) = p.as_ref() {
                                return (Num(0), 0, Some(Explode(n1, *x, *y)));
                            }
                        }
                    }
                }
                if e1.is_some() {
                    (Num(0), n1, e1)
                } else {
                    let (d, n2, e2) = check_explode(b, depth + 1, n1);
                    if e2.is_some() {
                        (Num(0), n2, e2)
                    } else {
                        (Dot(Box::new(c), Box::new(d)), n2, e2)
                    }
                }
            }
            Num(_) => (d.clone(), n + 1, None),
        }
    }
    fn check_split(d: &Pair, n: usize) -> (Pair, usize, Option<Action>) {
        match d {
            Dot(a, b) => {
                let (c, n1, e1) = check_split(a, n);
                if e1.is_some() {
                    (Num(0), n1, e1)
                } else {
                    let (d, n2, e2) = check_split(b, n1);
                    if e2.is_some() {
                        (Num(0), n2, e2)
                    } else {
                        (Dot(Box::new(c), Box::new(d)), n2, e2)
                    }
                }
            }
            Num(num) => {
                if num >= &10 {
                    (d.clone(), n + 1, Some(Split(n)))
                } else {
                    (d.clone(), n + 1, None)
                }
            }
        }
    }
    fn split(d: &Pair, n: usize, sp: usize) -> (Pair, usize) {
        match d {
            Dot(a, b) => {
                let (c, n1) = split(a, n, sp);
                let (d, n2) = split(b, n1, sp);
                (Dot(Box::new(c), Box::new(d)), n2)
            }
            Num(num) => {
                if n == sp {
                    (
                        Dot(
                            Box::new(Num((*num as f64 / 2.0).floor() as u64)),
                            Box::new(Num((*num as f64 / 2.0).ceil() as u64)),
                        ),
                        n + 2,
                    )
                } else {
                    (d.clone(), n + 1)
                }
            }
        }
    }
    fn explode(d: &Pair, ex: (usize, u64, u64), n: usize) -> (Pair, usize) {
        let (m, x, y) = ex;
        match d {
            Dot(a, b) => {
                let new_a = if let Dot(o, _) = a.as_ref() {
                    if let Num(k) = o.as_ref() {
                        if n == m && *k == x {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                let (c, n1) = explode(if new_a { &Num(0) } else { a }, ex, n);
                let new_b = if let Dot(o, _) = b.as_ref() {
                    if let Num(k) = o.as_ref() {
                        if n1 == m && *k == x {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                let (d, n2) = explode(if new_b { &Num(0) } else { b }, ex, n1);
                (Dot(Box::new(c), Box::new(d)), n2)
            }
            Num(num) => {
                if n + 1 == m {
                    (Num(num + x), n + 1)
                } else if n - 1 == m {
                    (Num(num + y), n + 1)
                } else {
                    (d.clone(), n + 1)
                }
            }
        }
    }
    fn magnitude(d: &Pair) -> u64 {
        match d {
            Dot(a, b) => {
                let mag1 = magnitude(a);
                let mag2 = magnitude(b);
                mag1 * 3 + mag2 * 2
            }
            Num(num) => *num,
        }
    }
    fn combine(a: Option<Action>, b: Option<Action>) -> Option<Action> {
        if a.is_some() {
            return a;
        }
        if b.is_some() {
            return b;
        }
        None
    }
    fn reduce(pp: &Pair) -> Pair {
        let mut p = pp.clone();
        loop {
            let (_, _, e1) = check_explode(&p, 0, 0);
            let (_, _, e2) = check_split(&p, 0);
            match combine(e1, e2) {
                Some(Explode(n, x, y)) => {
                    let (p1, _) = explode(&p, (n, x, y), 0);
                    p = p1;
                }
                Some(Split(n)) => {
                    let (p1, _) = split(&p, 0, n);
                    p = p1;
                }
                None => {
                    break;
                }
            }
        }
        p
    }
    let mut p = Num(0);
    let mut ps = Vec::<Pair>::new();
    for (i, line) in reader.lines().flatten().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let (pp, _) = parse_dot(&chars, 0);
        ps.push(pp.clone());
        if i == 0 {
            p = pp;
            continue;
        } else {
            p = Dot(Box::new(p), Box::new(pp));
        }
        p = reduce(&p);
    }
    print!("{} ", magnitude(&p));
    let mut max_m = u64::MIN;
    for i in 0..ps.len() {
        for j in 0..ps.len() {
            if i != j {
                let p = reduce(&Dot(Box::new(ps[i].clone()), Box::new(ps[j].clone())));
                let m = magnitude(&p);
                max_m = max(max_m, m);
            }
        }
    }
    println!("{}", max_m);
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
    day_09();
    day_10();
    day_11();
    day_12();
    day_13();
    day_14();
    day_15();
    day_16();
    day_17();
    day_18();
}
