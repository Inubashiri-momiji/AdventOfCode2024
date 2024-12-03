pub fn part1(content: Vec<Vec<i32>>){
    let safe_lines = get_safe_lines(content);
    println!("Safe lines: {:?}", { safe_lines.len() });
}

pub fn part2(sorted_lists: Vec<Vec<i32>>){
    let safe_lines = get_possible_safe_lines(sorted_lists);
    println!("Safe lines with some modification: {:?}", safe_lines.len());
}

fn get_safe_lines(content : Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut safe_lines : Vec<Vec<i32>>= Vec::new();
    for line in content {
        let is_safe = is_line_safe(line.clone());
        if is_safe {
            safe_lines.push(line);
        }
    }
    safe_lines
}

fn get_possible_safe_lines(content : Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut safe_lines : Vec<Vec<i32>>= Vec::new();

    for line in content {
        let is_safe = is_line_safe(line.clone());
        if is_safe {
            safe_lines.push(line);
        }else {
            for index in 0..line.len(){
                let mut new_line_option = line[..index].to_vec();
                let mut second_split = line[index+1..].to_vec();
                new_line_option.append(&mut second_split);
                let can_be_saved : bool = is_line_safe(new_line_option.clone());
                if can_be_saved{
                    safe_lines.push(new_line_option);
                    break;
                }
            }
        }
    }
    safe_lines
}

fn is_line_safe(line: Vec<i32>) -> bool {
    let is_going_down : bool = line[0] > line[1];
    line
        .iter()
        .zip(line.iter().skip(1))
        .all(|(value, next_value)| is_safe(*value, *next_value, is_going_down))
}

fn is_safe(value: i32, next_value : i32, is_going_down : bool) -> bool {
    if is_going_down { value > next_value && value - next_value <= 3 }
    else { value < next_value && next_value - value <= 3 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_safe_lines_test() {
        let test_content: Vec<Vec<i32>> = Vec::from([
            Vec::from([7, 6, 4, 2, 1]),
            Vec::from([1, 2, 7, 8, 9]),
            Vec::from([9, 7, 6, 2, 1]),
            Vec::from([1, 3, 2, 4, 5]),
            Vec::from([8, 6, 4, 4, 1]),
            Vec::from([1, 3, 6, 7, 9]),
        ]);

        let result = get_safe_lines(test_content);

        assert_eq!(2, result.len());
        assert_eq!(Vec::from([7, 6, 4, 2, 1]), result[0]);
        assert_eq!(Vec::from([1, 3, 6, 7, 9]), result[1]);
    }

    #[test]
    fn get_possible_safe_lines_test() {
        let test_content: Vec<Vec<i32>> = Vec::from([
            Vec::from([7, 6, 4, 2, 1]),
            Vec::from([1, 2, 7, 8, 9]),
            Vec::from([9, 7, 6, 2, 1]),
            Vec::from([1, 3, 2, 4, 5]),
            Vec::from([8, 6, 4, 4, 1]),
            Vec::from([1, 3, 6, 7, 9]),
        ]);

        let result = get_possible_safe_lines(test_content);

        assert_eq!(4, result.len());
        assert_eq!(Vec::from([7, 6, 4, 2, 1]), result[0]);
        assert_eq!(Vec::from([1, 2, 4, 5]), result[1]); // Safe by removing the second level, 3.
        assert_eq!(Vec::from([8, 6, 4, 1]), result[2]); //Safe by removing the third level, 4.
        assert_eq!(Vec::from([1, 3, 6, 7, 9]), result[3]);
    }
}