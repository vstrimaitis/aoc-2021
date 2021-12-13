use std::collections::HashMap;

static SMALL_LETTERS: &str = "
.##..###...##..####.####..##..#..#.###...##.#..#.#.....##..###..###...###.#..#.#...#.####......
#..#.#..#.#..#.#....#....#..#.#..#..#.....#.#.#..#....#..#.#..#.#..#.#....#..#.#...#....#.#####
#..#.###..#....###..###..#....####..#.....#.##...#....#..#.#..#.#..#.#....#..#..#.#....#..#...#
####.#..#.#....#....#....#.##.#..#..#.....#.#.#..#....#..#.###..###...##..#..#...#....#...#...#
#..#.#..#.#..#.#....#....#..#.#..#..#..#..#.#.#..#....#..#.#....#.#.....#.#..#...#...#....#...#
#..#.###...##..####.#.....###.#..#.###..##..#..#.####..##..#....#..#.###...##....#...####.#####
";

pub fn parse(s: &String) -> String {
    let letter_map = get_letter_map();
    let letter_arts = split_into_letters(s);

    let mut ans = String::new();
    for l in letter_arts.iter() {
        let matching_letters: Vec<char> = letter_map.iter()
            .filter(|&(_, s)| patterns_equal(l, s))
            .map(|(&c, _)| c)
            .collect();
        assert!(matching_letters.len() <= 1);
        ans.push(if matching_letters.len() == 0 {
            '?'
        } else {
            matching_letters[0]
        });
    }

    ans
}

fn patterns_equal(a: &String, b: &String) -> bool {
    let mut aa = trim_pattern(a);
    let mut bb = trim_pattern(b);
    let ha = get_height(&aa);
    let hb = get_height(&bb);
    if ha < hb {
        aa = pad_pattern(&aa, hb);
    } else if hb < ha {
        bb = pad_pattern(&bb, ha);
    }
    
    aa == bb
}

fn get_height(pattern: &String) -> usize {
    pattern.split('\n').count()
}

fn get_width(pattern: &String) -> usize {
    pattern.find('\n').unwrap_or(pattern.len())
}

fn pad_pattern(a: &String, height: usize) -> String {
    let mut n = get_height(a);
    let m = get_width(a);
    let row = ".".repeat(m) + "\n";

    let mut ans = String::new();
    while n < height {
        n += 1;
        ans.push_str(&row);
    }
    ans.push_str(a);
    ans
}

fn trim_pattern(a: &String) -> String {
    let grid = to_grid(a);
    let n = grid.len();
    let m = grid[0].len();
    let mut i_from = 0;
    let mut i_to = n-1;
    let mut j_from = 0;
    let mut j_to = m-1;
    while i_from < n && grid[i_from].iter().all(|&c| c == '.') {
        i_from += 1;
    }
    while grid[i_to].iter().all(|&c| c == '.') {
        i_to -= 1;
    }
    while j_from < m && (i_from..i_to+1).map(|i| grid[i][j_from]).all(|c| c == '.') {
        j_from += 1;
    }
    while (i_from..i_to+1).map(|i| grid[i][j_to]).all(|c| c == '.') {
        j_to -= 1;
    }
    if i_from > i_to || j_from > j_to {
        return "".to_string();
    }
    (i_from..i_to+1)
    .map(|i|
        (j_from..j_to+1)
        .map(|j|grid[i][j])
        .collect::<String>()
    )
    .collect::<Vec<String>>()
    .join("\n")
}

fn get_last<T: Copy>(v: &Vec<T>) -> Option<T> {
    if v.len() == 0 {
        None
    } else {
        Some(v[v.len()-1])
    }
}

fn split_into_letters(s: &String) -> Vec<String> {
    let grid = to_grid(s);
    let n = grid.len();
    let m = grid[0].len();

    let mut ranges: Vec<(usize, usize)> = vec![];
    for j in 0..m {
        let mut col = String::new();
        for i in 0..n {
            col.push(grid[i][j]);
        }
        if col.chars().all(|c| c == '.') {
            let prev_end = match get_last(&ranges) {
                Some((_, x)) => x + 2,
                None => 0
            };
            ranges.push((prev_end, j-1));
        }
    }
    ranges.push((match get_last(&ranges) {
        Some((_, x)) => x + 2,
        None => 0
    }, m-1));

    let mut ans = vec![];
    for (col_from, col_to) in ranges {
        if col_from >= col_to {
            continue;
        }
        let mut letter_art = String::new();
        for i in 0..n {
            for j in col_from..col_to+1 {
                letter_art.push(grid[i][j]);
            }
            letter_art.push('\n');
        }
        ans.push(letter_art.trim().to_string());
    }
    ans
}

fn to_grid(s: &String) -> Vec<Vec<char>> {
    s.trim()
        .split('\n')
        .map(|r| r.chars().collect())
        .collect()
}

fn get_letter_map() -> HashMap<char, String> {
    let mut map = HashMap::new();

    let letters = "ABCEFGHIJKLOPRSUYZ□";
    let letter_arts = split_into_letters(&SMALL_LETTERS.to_string());

    for (letter, letter_art) in letters.chars().zip(letter_arts.iter()) {
        map.insert(letter, letter_art.to_string());
    }

    map
}