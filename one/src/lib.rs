// TODO string non-digits from the input string?

pub fn sum(input: &str) -> u32 {
    let numbers: Vec<u32> = input 
        .chars()
        .map(|s| s.to_digit(10).unwrap() )
        .collect();
    let sum_numbers_followed_by_match = |acc, index| {
        let compare_offset = (index+1) % numbers.len();
        if numbers[index] == numbers[compare_offset]
        {
            acc + numbers[index]
        } else
        {
            acc
        }
    };
    let sum = (0..numbers.len()).fold(0, sum_numbers_followed_by_match);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_equals_zero() {
        assert_eq!(0, sum(""));
    }

    #[test]
    fn it_equals_three() {
        assert_eq!(3, sum("1122"));
    }

    #[test]
    fn it_equals_four() {
        assert_eq!(4, sum("1111"));
    }

    #[test]
    fn it_equals_zero() {
        assert_eq!(0, sum("1234"));
    }

    #[test]
    fn it_equals_nine() {
        assert_eq!(9, sum("91212129"));
    }
}
