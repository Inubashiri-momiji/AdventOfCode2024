use std::fs::read_to_string;
use std::collections::HashMap;

const FILE1PATH: &str = "input\\input1.txt";

fn main() {
    let file1_content = get_file_content(FILE1PATH);
    let content = get_lists_file_content(file1_content);
    part1(content.clone());
    part2(content);
}

fn part1(content: (Vec<i32>, Vec<i32>)){
    let sorted_list = sort_lists(content);
    let result_sum = sum_distances(sorted_list.0, sorted_list.1);
    println!("Sum: {:?}", { result_sum });
}

fn part2(sorted_lists: (Vec<i32>, Vec<i32>)){
    let result_similarity = calc_similarity_score(sorted_lists.0, sorted_lists.1);
    println!("Similarity: {:?}", { result_similarity });
}

fn get_file_content(path: &str) -> String{
    read_to_string(path).unwrap()
}

fn get_lists_file_content(file: String)-> (Vec<i32>, Vec<i32>){
    let split_list_by_whitespace = file.split_whitespace().collect::<Vec<&str>>();
    let capacity = split_list_by_whitespace.len() / 2;
    let mut sorted_list1 : Vec<i32> = Vec::with_capacity(capacity);
    let mut sorted_list2 : Vec<i32> = Vec::with_capacity(capacity);

    for (index, content) in split_list_by_whitespace.iter().enumerate(){
        let parsed_content : i32 = content.parse().unwrap();
        if index % 2 == 0{
            sorted_list1.push(parsed_content);
        }
        else {
            sorted_list2.push(parsed_content);
        }
    }

    (sorted_list1, sorted_list2)
}

fn sort_lists(pair: (Vec<i32>, Vec<i32>)) -> (Vec<i32>, Vec<i32>){
    let mut sort_list = pair.clone();
    sort_list.0.sort();
    sort_list.1.sort();
    sort_list
}

fn sum_distances(vec1 : Vec<i32>, vec2 : Vec<i32>) -> i32{
    let capacity = vec1.len();
    let mut count : usize = 0;
    let mut distance_list: Vec<i32> = Vec::with_capacity(capacity);

    while count < capacity {
        distance_list.push(((vec1[count] - vec2[count]) as i32).abs());
        count += 1;
    }

    distance_list.iter().sum()
}

fn calc_similarity_score(vec1 : Vec<i32>, vec2 : Vec<i32>) -> i32{
    let mut result = 0;
    let mut hashmap = vec2.iter().map(|num| (*num, 0)).collect::<HashMap<_,_>>();
    for num in vec2 {
        *hashmap.entry(num).or_insert(0) += 1;
    }

    for number in vec1 {
        result += number * (hashmap.get(&number).unwrap_or(&0));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_distances_test() {
        let mut first_list = vec![3, 4, 2, 1, 3, 3];
        let mut second_list = vec![4, 3, 5, 3, 9, 3];
        first_list.sort();
        second_list.sort();

        let result = sum_distances(first_list, second_list);

        assert_eq!(11, result);
    }

    #[test]
    fn get_lists_file_content_test() {
        let test_content: String = "3 4 4 3 2 5 1 3 3 9 3 3".to_string();
        let first_list = vec![3, 4, 2, 1, 3, 3];
        let second_list = vec![4, 3, 5, 3, 9, 3];

        let result = get_lists_file_content(test_content);

        assert_eq!(first_list, result.0);
        assert_eq!(second_list, result.1);
    }

    #[test]
    fn calc_similarity_score_test() {
        let first_list = vec![3, 4, 2, 1, 3, 3];
        let second_list = vec![4, 3, 5, 3, 9, 3];

        let result = calc_similarity_score(first_list, second_list);

        assert_eq!(31, result);
    }
}