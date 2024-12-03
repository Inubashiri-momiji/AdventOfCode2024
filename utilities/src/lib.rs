use std::fs::read_to_string;

pub fn get_file_content(path: &str) -> String{
    read_to_string(path).unwrap()
}

pub fn sort_lists(pair: (Vec<i32>, Vec<i32>)) -> (Vec<i32>, Vec<i32>){
    let mut sort_list = pair.clone();
    sort_list.0.sort();
    sort_list.1.sort();
    sort_list
}

pub fn get_lists_file_content(file: String)-> (Vec<i32>, Vec<i32>){
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
    fn get_lists_file_content_test() {
        let test_content: String = "3 4 4 3 2 5 1 3 3 9 3 3".to_string();
        let first_list = vec![3, 4, 2, 1, 3, 3];
        let second_list = vec![4, 3, 5, 3, 9, 3];

        let result = get_lists_file_content(test_content);

        assert_eq!(first_list, result.0);
        assert_eq!(second_list, result.1);
    }
}
