use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");

    let part_1 = multiply(INPUT);
    let part_2 = do_multiply(INPUT);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn multiply(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("regex must be valid");
    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [first, second])| {
            let first: u32 = first.parse().expect("first capture must be a valid u32");
            let second: u32 = second.parse().expect("second capture must be a valid u32");
            first * second
        })
        .sum()
}

fn do_multiply(input: &str) -> u32 {
    let ranges = do_ranges(input);
    ranges.into_iter().map(multiply).sum()
}

fn do_ranges(mut input: &str) -> Vec<&str> {
    let mut ranges = Vec::new();
    loop {
        let Some((enabled, remaining)) = input.split_once("don't()") else {
            ranges.push(input);
            return ranges;
        };
        input = remaining;
        ranges.push(enabled);
        let Some((_, remaining)) = input.split_once("do()") else {
            return ranges;
        };
        input = remaining;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        let multiply =
            multiply("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        assert_eq!(multiply, 161);
    }

    #[test]
    fn test_do_multiply() {
        let multiply = do_multiply(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        assert_eq!(multiply, 48);
    }
}
