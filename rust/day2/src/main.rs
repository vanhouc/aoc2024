#![feature(exact_size_is_empty)]
#![feature(test)]

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let reports = parse_input(INPUT);

    let part_1 = safe_reports(&reports);
    let part_2 = problem_dampener(&reports);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn safe_reports(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count()
}

fn is_report_safe(report: &[i32]) -> bool {
    let mut windows = report.iter().tuple_windows();
    let (first, second) = windows
        .next()
        .expect("report must have at least two entries");
    let kind = match second - first {
        1..4 => ReportKind::Increasing,
        -3..=-1 => ReportKind::Decreasing,
        _ => ReportKind::Invalid,
    };
    windows.all(|(first, second)| match kind {
        ReportKind::Increasing => (1..4).contains(&(second - first)),
        ReportKind::Decreasing => (-3..0).contains(&(second - first)),
        ReportKind::Invalid => false,
    })
}

fn problem_dampener(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            permutations(report)
                .iter()
                .any(|permutation| is_report_safe(permutation))
        })
        .count()
}

fn permutations(reports: &[i32]) -> Vec<Vec<i32>> {
    let mut permutations = Vec::new();
    for i in 0..reports.len() {
        let mut report_permutation = reports.to_owned();
        report_permutation.remove(i);
        permutations.push(report_permutation);
    }
    permutations
}

enum ReportKind {
    Increasing,
    Decreasing,
    Invalid,
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            // Each line is a list of u32s separated by whitespace
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("input must be a valid u32"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_safe_reports() {
        let reports = super::parse_input(INPUT);

        assert_eq!(super::safe_reports(&reports), 2);
    }

    #[test]
    fn test_problem_dampener() {
        let reports = super::parse_input(INPUT);

        assert_eq!(super::problem_dampener(&reports), 4);
    }
}
