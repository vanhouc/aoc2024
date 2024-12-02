#![feature(test)]

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (mut first_list, mut second_list) = parse_input(INPUT);

    first_list.sort();
    second_list.sort();

    let part1_answer = calculate_distance(&first_list, &second_list);
    let part2_answer = similarity_score(&first_list, &second_list);

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .fold((Vec::new(), Vec::new()), |mut vecs, line| {
            let (first, last) = line
                .split_once("   ")
                .map(|(first, last)| {
                    (
                        first
                            .parse::<u32>()
                            .expect("first segment must be a valid u32"),
                        last.parse::<u32>()
                            .expect("last segment must be a valid u32"),
                    )
                })
                .expect("two segments separated by three spaces");
            vecs.0.push(first);
            vecs.1.push(last);
            vecs
        })
}

// This function requires both lists to be in ascending order
fn calculate_distance(first_list: &[u32], second_list: &[u32]) -> u32 {
    first_list
        .iter()
        .zip(second_list)
        .fold(0, |mut acc, (first, last)| {
            acc += first.abs_diff(*last);
            acc
        })
}

fn similarity_score(first_list: &[u32], second_list: &[u32]) -> u32 {
    first_list.iter().fold(0, |acc, first| {
        acc + (first * (second_list.iter().filter(|second| first == *second).count() as u32))
    })
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;
    use test::Bencher;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    pub fn test_parse_input() {
        let (first_list, second_list) = parse_input(INPUT);

        assert_eq!(first_list, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(second_list, vec![4, 3, 5, 3, 9, 3]);
    }

    #[bench]
    pub fn benchmark_parse_input(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        b.iter(|| parse_input(input));
    }

    #[test]
    pub fn test_calculate_distance() {
        let (mut first_list, mut second_list) = parse_input(INPUT);
        first_list.sort();
        second_list.sort();

        assert_eq!(super::calculate_distance(&first_list, &second_list), 11);
    }

    #[bench]
    pub fn benchmark_calculate_distance(b: &mut Bencher) {
        let (mut first_list, mut second_list) = parse_input(include_str!("../input.txt"));
        first_list.sort();
        second_list.sort();

        b.iter(|| super::calculate_distance(&first_list, &second_list));
    }

    #[test]
    pub fn test_similarity_score() {
        let (first_list, second_list) = parse_input(INPUT);

        assert_eq!(super::similarity_score(&first_list, &second_list), 31);
    }

    #[bench]
    pub fn benchmark_similarity_score(b: &mut Bencher) {
        let (first_list, second_list) = parse_input(include_str!("../input.txt"));

        b.iter(|| super::similarity_score(&first_list, &second_list));
    }
}
