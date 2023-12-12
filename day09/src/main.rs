use itertools::Itertools;

fn next_value(seq: &[i64]) -> i64 {
    let mut diffs = seq.to_vec();
    let mut next_value = seq.last().copied().unwrap();

    while !diffs.iter().all(|&v| v == 0) {
        diffs = diffs
            .windows(2)
            .map(|values| values[1] - values[0])
            .collect_vec();

        next_value += diffs.last().unwrap();
    }

    next_value
}

fn prev_value(seq: &[i64]) -> i64 {
    let mut diffs = seq.to_vec();
    let mut prev_value = seq.first().copied().unwrap();
    let mut alternate_negation = -1;

    while !diffs.iter().all(|&v| v == 0) {
        diffs = diffs
            .windows(2)
            .map(|values| values[1] - values[0])
            .collect_vec();

        prev_value += alternate_negation * diffs.first().unwrap();
        alternate_negation *= -1;
    }

    prev_value
}

fn main() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let input = include_str!("input.txt");

    let sequences = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let part1 = sequences.iter().map(|seq| next_value(seq)).sum::<i64>();
    println!("part 1: {part1}");

    let part2 = sequences.iter().map(|seq| prev_value(seq)).sum::<i64>();
    println!("part 2: {part2}");
}
