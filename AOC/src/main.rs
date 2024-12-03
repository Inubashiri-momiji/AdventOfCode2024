mod day1;

use utilities::get_file_content;
use utilities::get_lists_file_content;

const DAY1FILE: &str = "input\\input1.txt";

fn main() {
    call_day1();
}

fn call_day1(){
    println!("Start day1");
    let file1_content = get_file_content(DAY1FILE);
    let content = get_lists_file_content(file1_content);
    day1::part1(content.clone());
    day1::part2(content);
    println!("End day1");
}

