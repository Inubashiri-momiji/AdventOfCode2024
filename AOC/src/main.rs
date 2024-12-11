mod day1;
mod day2;
mod day3;

use utilities::{get_file_content, get_file_content_as_lines};
use utilities::get_lists_file_content_day1;
use utilities::get_file_lines;

const DAY1FILE: &str = "input\\input1.txt";
const DAY2FILE: &str = "input\\input2.txt";
const DAY3FILE: &str = "input\\input3.txt";

fn main() {
    // TODO: Add match to ask for the day to execute.
    call_day1();
    call_day2();
    call_day3();
}

fn call_day1(){
    println!("Start day1");
    let file1_content = get_file_content(DAY1FILE);
    let content = get_lists_file_content_day1(file1_content);
    day1::part1(content.clone());
    day1::part2(content);
    println!("End day1");
}

fn call_day2(){
    println!("Start day2");
    let file_lines = get_file_lines(DAY2FILE);
    let file_lines_as_numbers = get_file_content_as_lines(file_lines);
    day2::part1(file_lines_as_numbers.clone());
    day2::part2(file_lines_as_numbers.clone());
    println!("End day2");
}

fn call_day3(){
    println!("Start day3");
    let file_lines = get_file_lines(DAY3FILE);
    let file_in_single_line = file_lines.join("");
    day3::part1(&file_in_single_line);
    day3::part2(&file_in_single_line);
    println!("End day3");
}
