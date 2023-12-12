use itertools::Itertools;

fn main() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let input = include_str!("input.txt");

    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let (mut v, next) = line.chars().enumerate().fold(
                (Vec::<((usize, usize), u64)>::new(), None),
                |(mut v, prev), (x, c)| {
                    let next = match prev {
                        Some(n) => {
                            if let Some(d) = c.to_digit(10) {
                                Some(n * 10 + d as u64)
                            } else {
                                v.push(((y, x - 1), n));
                                None
                            }
                        }
                        None => c.to_digit(10).map(|d| d as u64),
                    };

                    (v, next)
                },
            );

            v.extend(next.map(|n| ((y, line.len() - 1), n)));
            v
        })
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if !c.is_ascii_digit() && c != '.' {
                    Some((y, x, c))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;

    fn overlaps(nx: usize, ny: usize, sx: usize, sy: usize, num: u64) -> bool {
        let num_length = format!("{num}").len();
        ny.abs_diff(sy) <= 1 && (sx <= nx + 1 && nx <= sx + num_length)
    }

    for &((ny, nx), num) in &numbers {
        let is_part_number = symbols
            .iter()
            .any(|&(sy, sx, _)| overlaps(nx, ny, sx, sy, num));
        if is_part_number {
            part1 += num;
        }
    }

    println!("part1: {part1}");

    let mut part2 = 0;

    for &(sy, sx, symbol) in &symbols {
        if symbol == '*' {
            if let Some((p1, p2)) = numbers
                .iter()
                .filter(|&&((ny, nx), num)| overlaps(nx, ny, sx, sy, num))
                .map(|(_, num)| num)
                .collect_tuple()
            {
                part2 += p1 * p2;
            }
        }
    }

    println!("part2: {part2}");
}
