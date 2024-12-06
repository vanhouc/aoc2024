use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");

    let part_1 = multiply(INPUT);

    println!("Part 1: {}", part_1);
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

fn do_multiply(input: &str) -> u32 {}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_multiply() {
        let multiply = multiply(INPUT);

        assert_eq!(multiply, 161);
    }
}
