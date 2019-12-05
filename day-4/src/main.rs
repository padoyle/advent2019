fn main() {
    let mut matching = 0;
    for option in 168630..=718098 {
        if meets_criteria(option) {
            println!("Found valid option: {}", option);
            matching += 1;
        }
    }

    println!("Result: {}", matching);
}

fn meets_criteria(option: i32) -> bool {
    let digits = get_digits(option);

    // check doubles
    let mut has_valid_doubles = false;
    let mut ascending = true;
    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            let prev_is_different = i == 0 || digits[i - 1] != digits[i];
            let next_is_different = i == 4 || digits[i + 2] != digits[i];

            has_valid_doubles |= prev_is_different && next_is_different;
        }
        if digits[i] > digits[i + 1] {
            ascending = false;
        }
    }

    ascending && has_valid_doubles
}

fn get_digits(value: i32) -> Vec<i32> {
    let mut value = value;
    let mut result = Vec::with_capacity(6);
    while value != 0 {
        result.push(value % 10);
        value = value / 10;
    }

    result.reverse();
    result
}

mod test {
    use super::*;

    #[test]
    fn get_digits_test() {
        assert_eq!(vec![1, 2, 3, 4, 5, 6], get_digits(123456));
    }

    #[test]
    fn examples() {
        assert_eq!(meets_criteria(111111), false);
        assert_eq!(meets_criteria(223450), false);
        assert_eq!(meets_criteria(123789), false);
        assert_eq!(meets_criteria(112233), true);
        assert_eq!(meets_criteria(123444), false);
        assert_eq!(meets_criteria(111234), false);
        assert_eq!(meets_criteria(111122), true);
        println!("Final test");
        assert_eq!(meets_criteria(112222), true);
    }
}
