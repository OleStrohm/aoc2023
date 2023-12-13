use itertools::Itertools;

fn main() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let input = include_str!("input.txt");

    let part1 = input
        .split("\n\n")
        .map(|pattern| {
            let pattern = pattern
                .lines()
                .map(|l| l.chars().collect_vec())
                .collect_vec();

            fn num_diffs(left: &[char], right: &[char]) -> usize {
                left.iter()
                    .zip(right)
                    .filter(|(left, right)| left != right)
                    .count()
            }

            let potential_horizontal_mid = (0..pattern.len())
                .tuple_windows()
                .map(|(first, right)| (first, right, num_diffs(&pattern[first], &pattern[right])))
                .filter(|&(_, _, diffs)| diffs <= 1)
                .collect_vec();

            let potential_vertical_mid = (0..pattern[0].len())
                .tuple_windows()
                .map(|(first, second)| {
                    (
                        first,
                        second,
                        (0..pattern.len())
                            .filter(|&r| pattern[r][first] != pattern[r][second])
                            .count(),
                    )
                })
                .filter(|&(_, _, diffs)| diffs <= 1)
                .collect_vec();

            let mut symmetry_value = 0;
            for (first, second, diffs) in potential_horizontal_mid {
                let total_diffs =
                    std::iter::successors(Some((first, second, diffs)), |(first, second, _)| {
                        let first = first.checked_sub(1)?;
                        let second = second + 1;
                        (second < pattern.len()).then(|| {
                            let diffs = num_diffs(&pattern[first], &pattern[second]);
                            (first, second, diffs)
                        })
                    })
                    .map(|(_, _, diffs)| diffs)
                    .sum::<usize>();
                if total_diffs == 1 {
                    symmetry_value = 100 * (first + 1);
                }
            }

            for (first, second, diffs) in potential_vertical_mid {
                let total_diffs =
                    std::iter::successors(Some((first, second, diffs)), |(first, second, _)| {
                        let first = first.checked_sub(1)?;
                        let second = second + 1;
                        (second < pattern[0].len()).then(|| {
                            let diffs = (0..pattern.len())
                                .filter(|&r| pattern[r][first] != pattern[r][second])
                                .count();
                            (first, second, diffs)
                        })
                    })
                    .map(|(_, _, diffs)| diffs)
                    .sum::<usize>();
                if total_diffs == 1 {
                    symmetry_value = first + 1;
                }
            }

            symmetry_value
        })
        .sum::<usize>();

    println!("Part 1: {part1}");
}
