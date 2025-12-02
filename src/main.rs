use std::env::args;

mod common;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = args().collect();
    match args[1].as_str() {
        "d1" => match args[2].as_str() {
            "p1" => day1::part1(),
            "p2" => day1::part2(),

            _ => panic!("bad input"),
        },
        "d2" => match args[2].as_str() {
            "p1" => day2::part1(),
            "p2" => day2::part2(),

            _ => panic!("bad input"),
        },
        "d3" => match args[2].as_str() {
            "p1" => day3::part1(),
            "p2" => day3::part2(),

            _ => panic!("bad input"),
        },
        "d4" => match args[2].as_str() {
            "p1" => day4::part1(),
            "p2" => day4::part2(),

            _ => panic!("bad input"),
        },
        "d5" => match args[2].as_str() {
            "p1" => day5::part1(),
            "p2" => day5::part2(),

            _ => panic!("bad input"),
        },
        "d6" => match args[2].as_str() {
            "p1" => day6::part1(),
            "p2" => day6::part2(),

            _ => panic!("bad input"),
        },
        "d7" => match args[2].as_str() {
            "p1" => day7::part1(),
            "p2" => day7::part2(),

            _ => panic!("bad input"),
        },
        "d8" => match args[2].as_str() {
            "p1" => day8::part1(),
            "p2" => day8::part2(),

            _ => panic!("bad input"),
        },
        "d9" => match args[2].as_str() {
            "p1" => day9::part1(),
            "p2" => day9::part2(),

            _ => panic!("bad input"),
        },
        "d10" => match args[2].as_str() {
            "p1" => day10::part1(),
            "p2" => day10::part2(),

            _ => panic!("bad input"),
        },
        "d11" => match args[2].as_str() {
            "p1" => day11::part1(),
            "p2" => day11::part2(),

            _ => panic!("bad input"),
        },
        "d12" => match args[2].as_str() {
            "p1" => day12::part1(),
            "p2" => day12::part2(),

            _ => panic!("bad input"),
        },
        _ => panic!("bad input"),
    }
}
