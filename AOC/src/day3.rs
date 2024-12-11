use regex::Regex;

pub fn part1(content: &str){
    let mux_result = parse_line_with_regex(content, false);
    println!("Valid mux result: {:?}", { mux_result });
}

pub fn part2(content: &str){
    let mux_result = parse_line_with_regex(content, true);
    println!("filtered mux result: {:?}", { mux_result });
}

fn parse_line_with_regex(line: &str, filter_do: bool) -> i32{
    if filter_do {
        let mut total = 0;
        let do_donot_regex: Regex = Regex::new(r"(?:^|do\(\))(.*?)(?:$|don't\(\))").unwrap();
        let do_regex_group =  do_donot_regex.captures_iter(line).map(|c| c.extract());
        for (_, [filtered_line]) in do_regex_group{
            total += process_mul_on_line(filtered_line)
        }
        total
    }
    else { process_mul_on_line(line) }
}

fn process_mul_on_line(line: &str) -> i32{
    let mul_regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    let regex_group = mul_regex.captures_iter(line)
        .map(|capture| capture.extract());
    for (_, [num1, num2]) in regex_group{
        total += mul(num1, num2);
    }
    total
}

fn mul(number1_as_string: &str, number2_as_string: &str) -> i32{
    let number1 = number1_as_string.parse::<i32>().unwrap();
    let number2 = number2_as_string.parse::<i32>().unwrap();
    number1 * number2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_with_regex_on_false_should_use_entire_string_test() {
        let test_content: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = parse_line_with_regex(test_content, false);

        assert_eq!(161, result);
    }

    #[test]
    fn parse_line_with_regex_on_true_should_filter_string_test() {
        let test_content: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = parse_line_with_regex(test_content, true);

        assert_eq!(48, result);
    }
}