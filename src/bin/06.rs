advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let numbers: Vec<Vec<&str>> = input
        .lines()
        .map(|row| row.split_whitespace().collect())
        .collect();

    let h = numbers.len();
    let w = numbers.first().map_or(0, |r| r.len());

    let rotated: Vec<Vec<&str>> = (0..w)
        .map(|x| (0..h).rev().map(|y| numbers[y][x]).collect())
        .collect();

    let result: u64 = rotated
        .iter()
        .by_ref()
        .enumerate()
        .map(|(idx, rows)| {
            rows.iter()
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap())
                .reduce(|acc, x| match rotated[idx][0] {
                    "*" => acc * x,
                    "+" => acc + x,
                    _ => panic!("got {}", x),
                })
        })
        .sum::<Option<_>>()?;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
