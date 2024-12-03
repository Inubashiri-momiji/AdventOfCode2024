use std::collections::HashMap;
use utilities::sort_lists;

pub fn part1(content: (Vec<i32>, Vec<i32>)){
    let sorted_list = sort_lists(content);
    let result_sum = sum_distances(sorted_list.0, sorted_list.1);
    println!("Sum: {:?}", { result_sum });
}

pub fn part2(sorted_lists: (Vec<i32>, Vec<i32>)){
    let result_similarity = calc_similarity_score(sorted_lists.0, sorted_lists.1);
    println!("Similarity: {:?}", { result_similarity });
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
    fn calc_similarity_score_test() {
        let first_list = vec![3, 4, 2, 1, 3, 3];
        let second_list = vec![4, 3, 5, 3, 9, 3];

        let result = calc_similarity_score(first_list, second_list);

        assert_eq!(31, result);
    }
}