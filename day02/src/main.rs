use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&'_ str> for Color {
    fn from(color: &'_ str) -> Self {
        match color {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Not a color: {color:?}"),
        }
    }
}

fn part1(input: &str) -> u64 {
    let bag = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    input
        .lines()
        .enumerate()
        .filter(|(_, game)| {
            let (_, rounds) = game.split_once(':').unwrap();

            rounds.split(';').all(|round| {
                let round_counts: HashMap<Color, i32> =
                    HashMap::from_iter(round.split(',').map(|cube| {
                        let (count, color) = cube.trim().split_once(' ').unwrap();
                        (
                            Color::from(color.trim()),
                            count.trim().parse::<i32>().unwrap(),
                        )
                    }));

                bag.keys()
                    .all(|color| bag.get(color).unwrap() >= round_counts.get(color).unwrap_or(&0))
            })
        })
        .map(|(id, _)| id as u64 + 1)
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|game| {
            let (_, rounds) = game.split_once(':').unwrap();

            let minimum_set =
                rounds
                    .split(';')
                    .fold(HashMap::new(), |mut res: HashMap<Color, u64>, round| {
                        let round_counts: HashMap<Color, u64> =
                            HashMap::from_iter(round.split(',').map(|cube| {
                                let (count, color) = cube.trim().split_once(' ').unwrap();
                                (
                                    Color::from(color.trim()),
                                    count.trim().parse::<u64>().unwrap(),
                                )
                            }));

                        for (key, value) in &round_counts {
                            let v = res.entry(*key).or_insert(*value);
                            *v = *value.max(&*v);
                        }

                        res
                    });

            minimum_set.values().product::<u64>()
        })
        .sum::<u64>()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, part1(input));
    }

    #[test]
    fn example_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, part2(input));
    }
}
