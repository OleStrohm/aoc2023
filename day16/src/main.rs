use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    let input = include_str!("input.txt");

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    fn energized_from_entrance(start: (usize, usize, Dir), map: &[Vec<char>]) -> usize {
        let mut energized_tiles = BTreeSet::<(usize, usize, Dir)>::new();

        let mut current_tiles: BTreeSet<(usize, usize, Dir)> = [start].into_iter().collect();

        while !current_tiles.is_empty() {
            let newly_energized_tiles = energized_tiles.union(&current_tiles).copied().collect();

            current_tiles = current_tiles
                .into_iter()
                .flat_map(|(x, y, dir)| {
                    let mut extra = None;
                    let (dx, dy) = match map[y][x] {
                        '.' => match dir {
                            Dir::Up => (0, -1),
                            Dir::Down => (0, 1),
                            Dir::Left => (-1, 0),
                            Dir::Right => (1, 0),
                        },
                        '/' => match dir {
                            Dir::Up => (1, 0),
                            Dir::Down => (-1, 0),
                            Dir::Left => (0, 1),
                            Dir::Right => (0, -1),
                        },
                        '\\' => match dir {
                            Dir::Up => (-1, 0),
                            Dir::Down => (1, 0),
                            Dir::Left => (0, -1),
                            Dir::Right => (0, 1),
                        },
                        '|' => match dir {
                            Dir::Up => (0, -1),
                            Dir::Down => (0, 1),
                            Dir::Left => {
                                extra = Some((0, 1));
                                (0, -1)
                            }
                            Dir::Right => {
                                extra = Some((0, 1));
                                (0, -1)
                            }
                        },
                        '-' => match dir {
                            Dir::Up => {
                                extra = Some((-1, 0));
                                (1, 0)
                            }
                            Dir::Down => {
                                extra = Some((-1, 0));
                                (1, 0)
                            }
                            Dir::Left => (-1, 0),
                            Dir::Right => (1, 0),
                        },
                        _ => unreachable!(),
                    };

                    [(dx, dy)]
                        .into_iter()
                        .chain(extra)
                        .filter_map(move |(dx, dy)| {
                            let dir = match (dx, dy) {
                                (0, -1) => Dir::Up,
                                (0, 1) => Dir::Down,
                                (-1, 0) => Dir::Left,
                                (1, 0) => Dir::Right,
                                _ => unreachable!(),
                            };

                            let (nx, ny) = (x as isize + dx, y as isize + dy);

                            Some((nx.try_into().ok()?, ny.try_into().ok()?, dir))
                        })
                        .filter(|&(x, y, _)| x < map[0].len() && y < map.len())
                })
                .filter(|pos_dir| !energized_tiles.contains(pos_dir))
                .collect();

            energized_tiles = newly_energized_tiles;
        }

        energized_tiles
            .iter()
            .map(|&(x, y, _)| (x, y))
            .collect::<BTreeSet<_>>()
            .len()
    }

    let part1 = energized_from_entrance((0, 0, Dir::Right), &map);

    println!("Part 1: {part1}");

    let left_right_starts = (0..map.len())
        .flat_map(|y| [(0, y, Dir::Right), (map[0].len() - 1, y, Dir::Left)])
        .collect_vec();
    let top_bottom_starts = (0..map[0].len())
        .flat_map(|x| [(x, 0, Dir::Down), (x, map.len() - 1, Dir::Up)])
        .collect_vec();

    let part2 = left_right_starts
        .into_iter()
        .chain(top_bottom_starts)
        .map(|start| energized_from_entrance(start, &map))
        .max()
        .unwrap();

    println!("part 2: {part2}");
}
