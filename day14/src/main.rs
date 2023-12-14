use itertools::Itertools;

fn main() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let input = include_str!("input.txt");

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Kind {
        Round,
        Square,
        Empty,
    }

    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Kind::Round,
                    '#' => Kind::Square,
                    '.' => Kind::Empty,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    fn tilt_north(mut map: Vec<Vec<Kind>>) -> Vec<Vec<Kind>> {
        for col in 0..map[0].len() {
            let mut next_available_row = 0;
            for row in 0..map.len() {
                match map[row][col] {
                    Kind::Round => {
                        map[row][col] = Kind::Empty;
                        map[next_available_row][col] = Kind::Round;
                        next_available_row += 1;
                    }
                    Kind::Square => {
                        next_available_row = row + 1;
                    }
                    Kind::Empty => {}
                }
            }
        }

        map
    }

    fn tilt_west(mut map: Vec<Vec<Kind>>) -> Vec<Vec<Kind>> {
        for row in 0..map.len() {
            let mut next_available_col = 0;
            for col in 0..map[0].len() {
                match map[row][col] {
                    Kind::Round => {
                        map[row][col] = Kind::Empty;
                        map[row][next_available_col] = Kind::Round;
                        next_available_col += 1;
                    }
                    Kind::Square => {
                        next_available_col = col + 1;
                    }
                    Kind::Empty => {}
                }
            }
        }

        map
    }

    fn tilt_south(mut map: Vec<Vec<Kind>>) -> Vec<Vec<Kind>> {
        for col in 0..map[0].len() {
            let mut next_available_row = map.len() - 1;
            for row in (0..map.len()).rev() {
                match map[row][col] {
                    Kind::Round => {
                        map[row][col] = Kind::Empty;
                        map[next_available_row][col] = Kind::Round;
                        next_available_row = next_available_row.saturating_sub(1);
                    }
                    Kind::Square => {
                        next_available_row = row.saturating_sub(1);
                    }
                    Kind::Empty => {}
                }
            }
        }

        map
    }

    fn tilt_east(mut map: Vec<Vec<Kind>>) -> Vec<Vec<Kind>> {
        for row in 0..map.len() {
            let mut next_available_col = map[0].len() - 1;
            for col in (0..map[0].len()).rev() {
                match map[row][col] {
                    Kind::Round => {
                        map[row][col] = Kind::Empty;
                        map[row][next_available_col] = Kind::Round;
                        next_available_col = next_available_col.saturating_sub(1);
                    }
                    Kind::Square => {
                        next_available_col = col.saturating_sub(1);
                    }
                    Kind::Empty => {}
                }
            }
        }

        map
    }

    fn tilt_cycle(map: Vec<Vec<Kind>>) -> Vec<Vec<Kind>> {
        tilt_east(tilt_south(tilt_west(tilt_north(map))))
    }

    fn calculate_load(map: &[Vec<Kind>]) -> usize {
        let num_rows = map.len();

        map.iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .filter(|kind| matches!(kind, Kind::Round))
                    .map(move |_| num_rows - row_index)
            })
            .sum::<usize>()
    }

    let part1 = calculate_load(&tilt_north(map.clone()));

    println!("Part 1: {part1}");

    let mut previous = vec![map.clone()];
    let mut latest = map;

    while !previous
        .split_last()
        .unwrap()
        .1
        .iter()
        .rev()
        .any(|prev| prev == &latest)
    {
        latest = tilt_cycle(latest);
        previous.push(latest.clone());
    }

    let (loop_start, loop_end) = previous
        .iter()
        .enumerate()
        .filter(|(_, prev)| prev == &previous.last().unwrap())
        .map(|(i, _)| i)
        .collect_tuple()
        .unwrap();

    let remainder = (1000000000 - loop_start) % (loop_end - loop_start);

    let mut remainder_map = previous.last().unwrap().clone();

    for _ in 0..remainder {
        remainder_map = tilt_cycle(remainder_map);
    }

    println!("Part 2: {}", calculate_load(&remainder_map));
}
