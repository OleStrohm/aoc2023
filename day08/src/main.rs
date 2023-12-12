use std::collections::{BTreeMap, BTreeSet};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::Integer;
use num::integer::{gcd, lcm};

fn main() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let input = include_str!("input.txt");

    let (steps, nodes) = input.split_once("\n\n").unwrap();

    let nodes = nodes
        .lines()
        .map(|line| {
            let node = &line[0..3];
            let l = &line[7..10];
            let r = &line[12..15];

            (node, (l, r))
        })
        .collect::<BTreeMap<_, _>>();

    //let part1 = steps
    //    .chars()
    //    .cycle()
    //    .enumerate()
    //    .fold_while(("AAA", 0), |(node, _), (steps, dir)| {
    //        if node == "ZZZ" {
    //            Done((node, steps))
    //        } else {
    //            Continue((
    //                match dir {
    //                    'L' => nodes.get(node).unwrap().0,
    //                    'R' => nodes.get(node).unwrap().1,
    //                    _ => unreachable!(),
    //                },
    //                0,
    //            ))
    //        }
    //    })
    //    .into_inner()
    //    .1;

    //println!("Part 1: {part1}");

    let starting_nodes = nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .copied()
        .collect::<BTreeSet<_>>();

    dbg!(&starting_nodes);

    let looping_paths = starting_nodes
        .iter()
        .map(|&start| {
            let (_, pre_loop, in_loop) = steps
                .chars()
                .cycle()
                .fold_while(
                    (start, vec![start], vec![]),
                    |(cur, mut pre_loop, mut in_loop), dir| {
                        let next = match dir {
                            'L' => nodes.get(cur).unwrap().0,
                            'R' => nodes.get(cur).unwrap().1,
                            _ => unreachable!(),
                        };
                        if Some(&next) == in_loop.first() {
                            Done((next, pre_loop, in_loop))
                        } else {
                            if next.ends_with('Z') || !in_loop.is_empty() {
                                in_loop.push(next);
                            } else {
                                pre_loop.push(next);
                            }
                            Continue((next, pre_loop, in_loop))
                        }
                    },
                )
                .into_inner();
            (pre_loop, in_loop)
        })
        .collect_vec();

    //dbg!(&looping_paths);

    let steps_until_looping = looping_paths
        .iter()
        .map(|(pre_loop, _)| pre_loop.len())
        .collect_vec();

    dbg!(steps_until_looping);
    let loop_lengths = looping_paths
        .iter()
        .map(|(_, in_loop)| in_loop.len())
        .collect_vec();

    let overlap = loop_lengths.iter().copied().reduce(lcm).unwrap();
    println!("Part 2: {overlap}");

    //let times_at_end = looping_paths
    //    .iter()
    //    .map(|(pre_loop, _)| {
    //        pre_loop
    //            .iter()
    //            .enumerate()
    //            .filter(|(_, n)| n.ends_with('Z'))
    //            .map(|(steps, _)| steps)
    //            .collect_vec()
    //    })
    //    .collect_vec();
    //dbg!(times_at_end);
    //let states = starting_nodes.into_iter().collect_vec();

    //let part2 = steps
    //    .chars()
    //    .cycle()
    //    .enumerate()
    //    .fold_while((starting_nodes, 0), |(in_nodes, _), (steps, dir)| {
    //        if in_nodes.iter().all(|n| n.ends_with('Z')) {
    //            Done((in_nodes, steps))
    //        } else {
    //            Continue((
    //                in_nodes
    //                    .into_iter()
    //                    .map(|node| match dir {
    //                        'L' => nodes.get(node).unwrap().0,
    //                        'R' => nodes.get(node).unwrap().1,
    //                        _ => unreachable!(),
    //                    })
    //                    .collect::<BTreeSet<_>>(),
    //                0,
    //            ))
    //        }
    //    })
    //    .into_inner()
    //    .1;

    //println!("Part 2: {part2}");
}
