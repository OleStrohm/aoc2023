use std::iter;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let input = "
F-S---7.
|.....|.
|.F-7.|.
L-J.L-J.
";
    let input = input.trim();
    let input = include_str!("input.txt");

    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let start = map
        .iter()
        .enumerate()
        .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .find(|&(_, _, c)| c == 'S')
        .map(|(x, y, _)| (x, y))
        .unwrap();

    fn next_connected_dir((x, y): (isize, isize), dir: Dir, map: &[Vec<char>]) -> Option<Dir> {
        let y: usize = y.try_into().ok()?;
        let x: usize = x.try_into().ok()?;
        let &c = map.get(y).and_then(|r| r.get(x))?;

        use Dir::*;
        match dir {
            Up => match c {
                '|' => Some(Up),
                '7' => Some(Left),
                'F' => Some(Right),
                _ => None,
            },
            Down => match c {
                '|' => Some(Down),
                'J' => Some(Left),
                'L' => Some(Right),
                _ => None,
            },
            Right => match c {
                '-' => Some(Right),
                'J' => Some(Up),
                '7' => Some(Down),
                _ => None,
            },
            Left => match c {
                '-' => Some(Left),
                'L' => Some(Up),
                'F' => Some(Down),
                _ => None,
            },
        }
    }

    fn move_pos_in_dir((x, y): (isize, isize), dir: Dir) -> (isize, isize) {
        match dir {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Right => (x + 1, y),
            Dir::Left => (x - 1, y),
        }
    }

    let start = (start.0 as isize, start.1 as isize);
    let beginnings = [Dir::Up, Dir::Down, Dir::Right, Dir::Left]
        .into_iter()
        .map(|d| (d, move_pos_in_dir(start, d)))
        .filter(|&(dir, p)| next_connected_dir(p, dir, &map).is_some())
        .collect_tuple()
        .unwrap();
    let simulatenous_traversal = iter::successors(Some(beginnings), |&((dir1, p1), (dir2, p2))| {
        let next_dir1 = next_connected_dir(p1, dir1, &map).unwrap();
        let next_p1 = move_pos_in_dir(p1, next_dir1);
        let next_dir2 = next_connected_dir(p2, dir2, &map).unwrap();
        let next_p2 = move_pos_in_dir(p2, next_dir2);

        if next_p1 == next_p2 || [p1, p2] == [next_p2, next_p1] {
            None
        } else {
            Some(((next_dir1, next_p1), (next_dir2, next_p2)))
        }
    });

    println!("part 1: {}", simulatenous_traversal.count() + 1);

    let first_piece = beginnings.0;
    let path = [start, first_piece.1]
        .into_iter()
        .chain(
            iter::successors(Some(first_piece), |&(dir1, p1)| {
                let next_dir1 = next_connected_dir(p1, dir1, &map).unwrap();
                let next_p1 = move_pos_in_dir(p1, next_dir1);

                if next_p1 == start {
                    None
                } else {
                    Some((next_dir1, next_p1))
                }
            })
            .map(|(_, p)| p),
        )
        .collect_vec();

    let part_of_loop = (0..map.len())
        .map(|y| {
            (0..map[0].len())
                .map(|x| path.contains(&(x as isize, y as isize)))
                .collect_vec()
        })
        .collect_vec();

    let ((dir1, _), (dir2, _)) = beginnings;
    let starting_piece = match (dir1, dir2) {
        (Dir::Up, Dir::Down) => '|',
        (Dir::Up, Dir::Right) => 'L',
        (Dir::Up, Dir::Left) => 'J',
        (Dir::Down, Dir::Right) => 'F',
        (Dir::Down, Dir::Left) => '7',
        (Dir::Right, Dir::Left) => '-',
        _ => unreachable!(),
    };

    let mut area = 0;
    (0..map.len()).for_each(|y| {
        let mut enclosed_in_loop = false;
        for x in 0..map[0].len() {
            if part_of_loop[y][x] {
                let piece = match map[y][x] {
                    'S' => starting_piece,
                    p => p,
                };
                if matches!(piece, '|' | '7' | 'F' | 'S') {
                    enclosed_in_loop = !enclosed_in_loop;
                }
            } else if enclosed_in_loop {
                area += 1;
            }
        }
    });

    println!("part 2: {area}");
}
