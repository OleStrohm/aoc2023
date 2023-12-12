use itertools::Itertools;

fn main() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let input = include_str!("input.txt");

    let empty_map = input
        .lines()
        .map(|l| l.chars().map(|c| c == '.').collect_vec())
        .collect_vec();

    let mut galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect_vec();

    let empty_rows = empty_map
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&empty| empty))
        .map(|(r, _)| r)
        .collect_vec();
    let empty_cols = (0..empty_map[0].len())
        .filter(|&col| empty_map.iter().all(|r| r[col]))
        .collect_vec();

    for (x, y) in &mut galaxies {
        let (Ok(x_expansion) | Err(x_expansion)) = empty_cols.binary_search(x);
        let (Ok(y_expansion) | Err(y_expansion)) = empty_rows.binary_search(y);

        *x += x_expansion;
        *y += y_expansion;
    }

    let part1 = galaxies
        .iter()
        .cartesian_product(&galaxies)
        .map(|(&(x1, y1), &(x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum::<usize>() / 2;

    println!("Part 1: {part1}");

    let mut galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect_vec();

    for (x, y) in &mut galaxies {
        let (Ok(x_expansion) | Err(x_expansion)) = empty_cols.binary_search(x);
        let (Ok(y_expansion) | Err(y_expansion)) = empty_rows.binary_search(y);

        *x += x_expansion * 999999;
        *y += y_expansion * 999999;
    }

    let part2 = galaxies
        .iter()
        .cartesian_product(&galaxies)
        .map(|(&(x1, y1), &(x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum::<usize>() / 2;

    println!("Part 2: {part2}");
}
