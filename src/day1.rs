use crate::common::{LeftOrRight, div_rem};

const INPUT: &'static str = include_str!("../inputs/d1");
const SMALL_INPUT: &'static str = include_str!("../inputs/d1small");
const NATALIE_INPUT: &'static str = include_str!("../inputs/d1natalie");

pub fn parse_input(inp: &'static str) -> Vec<(LeftOrRight, isize)> {
    inp.lines()
        .map(|line| {
            let (direction, num) = line.split_at(1);
            let direction = match direction {
                "L" => LeftOrRight::Left,
                "R" => LeftOrRight::Right,
                _ => panic!("Bad input"),
            };
            let num = str::parse::<isize>(num).expect("Bad input");
            (direction, num)
        })
        .collect()
}

pub fn part1() {
    let mut dial_num = 50;
    let mut acc = 0;
    for (direction, amount) in parse_input(INPUT) {
        dial_num = match direction {
            LeftOrRight::Left => {
                let res = (dial_num - amount) % 100;
                if res < 0 { 100 - res.abs() } else { res }
            }
            LeftOrRight::Right => (dial_num + amount) % 100,
        };
        if dial_num == 0 {
            acc += 1;
        }
    }
    println!("Day 1 Part 1: {acc}")
}
pub fn part2() {
    let mut dial_num = 50;
    let mut acc = 0;
    for (direction, amount) in parse_input(SMALL_INPUT) {
        dial_num = match direction {
            LeftOrRight::Left => {
                let (passes, res) = div_rem(dial_num - amount, 100);
                if dial_num == 0 {
                    //if starting on 0
                    acc -= 1;
                }
                if res == 0 {
                    //if ending on 0
                    acc += 1;
                }
                acc += passes.abs();
                if res < 0 {
                    acc += 1;
                    100 - res.abs()
                } else {
                    res
                }
            }
            LeftOrRight::Right => {
                let (passes, res) = div_rem(dial_num + amount, 100);
                acc += passes.abs();
                res
            }
        };
    }

    println!("Day 1 Part 2: {acc}")
}
