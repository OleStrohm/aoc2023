use itertools::Itertools;

fn main() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let input = include_str!("input.txt");

    let part1 = input
        .lines()
        .map(|l| {
            let (record, contiguous) = l.split_once(' ').unwrap();
            let springs = record.len() as u64;
            let known_working = record
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == '.')
                .fold(0, |res, (i, _)| res | (1 << i));
            let known_broken = record
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .fold(0, |res, (i, _)| res | (1 << i));

            let contiguous = contiguous
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec();

            let areas = contiguous.len() as u64 + 1;
            let num_broken = contiguous.iter().sum::<u64>();

            let fixed_to_place = springs + 1 - num_broken - contiguous.len() as u64;
            (0..areas)
                .map(|area| {
                    let min_fixed = if area == 0 || area == areas - 1 { 0 } else { 1 };
                    min_fixed..=fixed_to_place + min_fixed
                })
                .multi_cartesian_product()
                .filter(|allocations| allocations.iter().sum::<u64>() + num_broken == springs)
                .map(|allocations| {
                    allocations
                        .into_iter()
                        .map(|n| vec![0_u64; n as usize])
                        .interleave(contiguous.iter().map(|&n| vec![1; n as usize]))
                        .flatten()
                        .enumerate()
                        .fold(0, |res, (i, b)| res | (b << i))
                })
                .filter(|v| {
                    let matches_working = (v & known_working) == 0;
                    let matches_broken = (v & known_broken) == known_broken;

                    matches_working && matches_broken
                })
                .count()
        })
        .sum::<usize>();

    println!("Part 1: {part1}");

    fn possible_ways_dynamic(reference: &[char], contiguous: &[usize]) -> u64 {
        let mut memo = vec![vec![0; contiguous.len() + 1]; reference.len() + 1];

        let reference_length = reference.len();

        for ref_index in (0..=reference.len()).rev() {
            for contig_index in (0..=contiguous.len()).rev() {
                let reference = &reference[ref_index..];
                let contiguous = &contiguous[contig_index..];

                if reference.is_empty() {
                    if contiguous.is_empty() {
                        memo[ref_index][contig_index] = 1;
                    } else {
                        memo[ref_index][contig_index] = 0;
                    }
                    continue;
                }

                if contiguous.is_empty() {
                    if !reference.iter().any(|&c| c == '#') {
                        memo[ref_index][contig_index] = 1;
                    } else {
                        memo[ref_index][contig_index] = 0;
                    }
                    continue;
                }

                let contig_size = contiguous[0];

                let mut possibilities = 0;

                if reference[0] != '#' {
                    possibilities += memo
                        .get((ref_index + 1).min(reference_length))
                        .map(|memo| memo[contig_index])
                        .unwrap_or(0);
                }

                if reference
                    .get(..contig_size)
                    .map(|reference| reference.iter().all(|c| matches!(c, '#' | '?')))
                    .unwrap_or(false)
                    && reference.get(contig_size) != Some(&'#')
                {
                    possibilities += memo
                        .get((ref_index + contig_size + 1).min(reference_length))
                        .and_then(|memo| memo.get(contig_index + 1))
                        .unwrap_or(&0);
                }

                memo[ref_index][contig_index] = possibilities;
            }
        }

        memo[0][0]
    }

    let part2 = input
        .lines()
        .map(|l| {
            let (record, contiguous) = l.split_once(' ').unwrap();
            #[allow(unstable_name_collisions)]
            let record = std::iter::repeat(record)
                .take(5)
                .intersperse("?")
                .flat_map(|r| r.chars())
                .collect_vec();

            let contiguous = contiguous
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            let contiguous = std::iter::repeat(contiguous)
                .take(5)
                .flatten()
                .collect_vec();

            possible_ways_dynamic(&record, &contiguous)
        })
        .sum::<u64>();

    println!("Part 2: {part2}");
}
