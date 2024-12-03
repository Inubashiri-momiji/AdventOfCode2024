use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_file_content(path: &str) -> String{
    read_to_string(path).unwrap()
}

pub fn sort_lists(pair: (Vec<i32>, Vec<i32>)) -> (Vec<i32>, Vec<i32>){
    let mut sort_list = pair.clone();
    sort_list.0.sort();
    sort_list.1.sort();
    sort_list
}

pub fn get_file_lines(path: &str) -> Vec<String>{
    if let Ok(lines) = read_lines(path) {
        return lines.flatten().collect::<Vec<String>>();
    }
    panic!("Failed to read file at: {}", path);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_file_content_as_lines(lines: Vec<String>) -> Vec<Vec<i32>>{
    let mut content : Vec<Vec<i32>> = Vec::new();
    for line in lines{
        content.push(line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }
    content
}

pub fn get_lists_file_content_day1(file: String) -> (Vec<i32>, Vec<i32>){
    let split_list_by_whitespace = file.split_whitespace().collect::<Vec<&str>>();
    let capacity = split_list_by_whitespace.len() / 2;
    let mut list1: Vec<i32> = Vec::with_capacity(capacity);
    let mut list2: Vec<i32> = Vec::with_capacity(capacity);

    for (index, content) in split_list_by_whitespace.iter().enumerate(){
        let parsed_content : i32 = content.parse().unwrap();
        if index % 2 == 0{
            list1.push(parsed_content);
        }
        else {
            list2.push(parsed_content);
        }
    }

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_lists_file_content_day1_test() {
        let test_content: String = "3 4 4 3 2 5 1 3 3 9 3 3".to_string();
        let first_list = vec![3, 4, 2, 1, 3, 3];
        let second_list = vec![4, 3, 5, 3, 9, 3];

        let result = get_lists_file_content_day1(test_content);

        assert_eq!(first_list, result.0);
        assert_eq!(second_list, result.1);
    }

    #[test]
    fn get_file_content_as_lines_test() {

        let test_content: Vec<String> = Vec::from(["7 6 4 2 1".to_string(), "1 2 7 8 9".to_string()]);

        let result = get_file_content_as_lines(test_content);

        assert_eq!(result.len(), 2);
        assert_eq!(Vec::from([7, 6, 4, 2, 1]), result[0]);
        assert_eq!(Vec::from([1, 2, 7, 8, 9]), result[1]);
    }
}
