const INPUT: &'static str = include_str!("../inputs/d2");
const SMALL_INPUT: &'static str = include_str!("../inputs/d2small");

fn parse_input(inp: &'static str) -> Vec<(&'static str, &'static str)> {
    inp.trim()
        .split(",")
        .map(|x| x.split_once("-").expect("Bad Input"))
        .collect()
}

pub fn part1() {
    let mut acc = 0;
    for range @ (l_bound, r_bound) in parse_input(INPUT) {
        //If both bounds are odd then there are no ids of even length, and therefore no invalid ids.
        if l_bound.len() % 2 == 0 || r_bound.len() % 2 == 0 {
            for (id, id_str) in expand_bound(range) {
                if check_invalidity_p1(&id_str) {
                    acc += id;
                }
            }
        };
    }
    println!("Day 2 part 1: {acc}")
}

fn check_invalidity_p1(id: &str) -> bool {
    if id.len() % 2 != 0 {
        //odd numbers are always valid
        return false;
    }
    let (l, r) = id.split_at(id.len() / 2);
    l == r
}

pub fn part2() {
    let mut acc = 0;
    for range in parse_input(INPUT) {
        for (id, id_str) in expand_bound(range) {
            if check_invalidity_p2(&id_str) {
                acc += id;
            }
        }
    }
    println!("Day 2 part 1: {acc}")
}

fn check_invalidity_p2(id: &str) -> bool {
    let id_chars: Vec<char> = id.chars().collect();
    let max_chunk_size = id.len().div_ceil(2);
    for chunk_size in 1..=max_chunk_size {
        let mut prev = None;
        let mut flag = false;
        for chunk in id_chars.chunks(chunk_size) {
            // println!("chunk:{chunk:?}, prev:{prev:?}");
            if let Some(p) = prev {
                if p != chunk {
                    flag = false;
                    break;
                } else {
                    flag = true;
                    prev = Some(chunk);
                }
            } else {
                prev = Some(chunk);
            }
        }
        if flag {
            return flag;
        }
    }
    false
}

fn expand_bound((l, r): (&'static str, &'static str)) -> Vec<(usize, String)> {
    let l = str::parse::<usize>(l).expect("Bad input");
    let r = str::parse::<usize>(r).expect("Bad input");
    (l..=r).map(|x| (x, x.to_string())).collect()
}
