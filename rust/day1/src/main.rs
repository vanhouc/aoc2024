fn main() {
    let input = include_str!("../input.txt");

    let (first_list, second_list) = parse_input(input);

    // Calculate the answer to part 2 first since the answer to part 1 requires
    // the lists to be sorted. This prevents having to clone the lists.
    let part2_answer = similarity_score(&first_list, &second_list);

    let part1_answer = calculate_distance(first_list, second_list);

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

fn calculate_distance(mut first_list: Vec<u32>, mut second_list: Vec<u32>) -> u32 {
    first_list.sort();
    second_list.sort();
    first_list
        .into_iter()
        .zip(second_list)
        .fold(0, |mut acc, (first, last)| {
            acc += first.abs_diff(last);
            acc
        })
}

fn similarity_score(first_list: &[u32], second_list: &[u32]) -> u32 {
    first_list.iter().fold(0, |acc, first| {
        acc + (first
            * (second_list
                .iter()
                .filter(|second| *first == **second)
                .count() as u32))
    })
}
