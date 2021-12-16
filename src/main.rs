use std::cmp::{max, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::{read_to_string, File};
use std::io::{prelude::*, BufReader};
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
    let mut graph = Vec::<HashMap<usize, usize>>::new();
    graph.resize(field.len() * field[0].len(), HashMap::<usize, usize>::new());
    let mut big_field = Vec::<Vec<usize>>::new();
    big_field.resize(field.len() * 5, vec![0; field[0].len() * 5]);
    let mut big_graph = Vec::<HashMap<usize, usize>>::new();
    big_graph.resize(
        big_field.len() * big_field[0].len(),
        HashMap::<usize, usize>::new(),
    );
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
    let fill_graph = |graph: &mut Vec<HashMap<usize, usize>>, field: &Vec<Vec<usize>>| {
        let sat = |r: usize, c: usize| r * field[0].len() + c;
        for i in 0..field.len() {
            for j in 0..field[0].len() {
                let r = sat(i, j);
                if i > 0 {
                    graph[r].insert(sat(i - 1, j), field[i - 1][j]);
                }
                if j > 0 {
                    graph[r].insert(sat(i, j - 1), field[i][j - 1]);
                }
                if i < field.len() - 1 {
                    graph[r].insert(sat(i + 1, j), field[i + 1][j]);
                }
                if j < field[0].len() - 1 {
                    graph[r].insert(sat(i, j + 1), field[i][j + 1]);
                }
            }
        }
    };
    fill_graph(&mut graph, &field);
    fill_graph(&mut big_graph, &big_field);
    let astar = |graph: &Vec<HashMap<usize, usize>>, field: &Vec<Vec<usize>>| {
        let h = |u: usize| field[0].len() - u % field[0].len() + field.len() - u / field[0].len();
        let rows = field.len() * field[0].len();
        let mut open_set = BinaryHeap::new();
        open_set.push((Reverse(0), 0));
        let mut came_from = HashMap::<usize, usize>::new();
        let mut score = vec![usize::MAX; rows];
        score[0] = 0;
        while let Some((Reverse(_), idx)) = open_set.pop() {
            if idx == rows - 1 {
                break;
            }
            for (&k, &v) in graph[idx].iter() {
                let new_score = score[idx] + v;
                if new_score < score[k] {
                    came_from.insert(k, idx);
                    score[k] = new_score;
                    open_set.push((Reverse(new_score + h(k)), k));
                }
            }
        }
        score[rows - 1]
    };
    print!(
        "{} {}",
        astar(&graph, &field),
        astar(&big_graph, &big_field)
    );
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
}
