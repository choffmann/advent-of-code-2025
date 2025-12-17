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
                    _ => panic!("got {}", rotated[idx][0]),
                })
        })
        .sum::<Option<_>>()?;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.lines();
    let operators: Vec<_> = input
        .by_ref()
        .rev()
        .take(1)
        .flat_map(|x| x.split_whitespace())
        .collect();

    let symbols: Vec<Vec<_>> = input
        .rev()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();

    let h = symbols.len();
    let w = symbols.iter().map(|r| r.len()).max().unwrap_or(0);

    let at = |y: usize, x: usize| -> char {
        symbols
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(' ')
    };

    let mut out: Vec<Vec<u64>> = Vec::new();
    let mut cur: Vec<u64> = Vec::new();

    for x in 0..w {
        let has_digit = (0..h).any(|y| at(y, x).is_ascii_digit());
        if !has_digit {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }

        let s: String = (0..h)
            .rev()
            .filter_map(|y| {
                let c = symbols[y][x];
                c.is_ascii_digit().then_some(c)
            })
            .collect();

        if !s.is_empty() {
            cur.push(s.parse::<u64>().unwrap());
        }
    }

    if !cur.is_empty() {
        out.push(cur);
    }

    let result: u64 = out
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            x.into_iter().reduce(|acc, v| match operators[i] {
                "*" => acc * v,
                "+" => acc + v,
                _ => panic!("got {}", operators[i]),
            })
        })
        .sum::<Option<_>>()?;

    Some(result)
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
        assert_eq!(result, Some(3263827));
    }
}
