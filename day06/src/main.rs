use std::iter::zip;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let (times, distances) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|w| w.parse::<u64>().ok())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    let part1 = zip(&times, &distances)
        .map(|(&time, &distance)| {
            (0..time)
                .filter(|&time_held| time_held * (time - time_held) > distance)
                .count()
        })
        .product::<usize>();
    println!("Part 1: {part1}");

    let (time, distance) = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let part2 = (0..time)
        .filter(|&time_held| time_held * (time - time_held) > distance)
        .count();

    println!("Part 2: {part2}");
}

//fn main() {
//    let input = include_str!("input.txt");
//
//    let (times, distances) = input
//        .lines()
//        .map(|l| {
//            l.split_whitespace()
//                .flat_map(|w| w.parse::<u64>().ok())
//                .collect_vec()
//        })
//        .collect_tuple()
//        .unwrap();
//
//    //let mut part1 = 1;
//    //for (&time, &distance) in zip(&times, &distances) {
//    //    let mut number_of_ways = 0;
//    //    for time_holding_down in 0..time {
//    //        let speed = time_holding_down;
//    //        let distance_covered = speed * (time - time_holding_down);
//    //        if distance_covered > distance {
//    //            number_of_ways += 1;
//    //        }
//    //    }
//    //    part1 *= number_of_ways;
//    //}
//
//    let part1 = zip(&times, &distances)
//        .map(|(&time, &distance)| {
//            (0..time)
//                .filter(|&time_held| time_held * (time - time_held) > distance)
//                .count()
//        })
//        .product::<usize>();
//    println!("Part 1: {part1}");
//
//    let (time, distance) = input
//        .lines()
//        .map(|l| {
//            l.chars()
//                .filter(|c| c.is_ascii_digit())
//                .collect::<String>()
//                .parse::<u64>()
//                .unwrap()
//        })
//        .collect_tuple()
//        .unwrap();
//
//    //let mut part2 = 0;
//
//    //for time_holding_down in 0..time {
//    //    let speed = time_holding_down;
//    //    let distance_covered = speed * (time - time_holding_down);
//    //    if distance_covered > distance {
//    //        part2 += 1;
//    //    }
//    //}
//
//    let part2 = (0..time)
//        .filter(|&time_held| time_held * (time - time_held) > distance)
//        .count();
//
//    println!("Part 2: {part2}");
//}
