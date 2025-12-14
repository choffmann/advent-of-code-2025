advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .map(|bank| {
            let mut largest = 1;
            let chars: Vec<char> = bank.chars().collect();

            for i in 0..chars.len() {
                for j in i + 1..chars.len() {
                    let combine = format!("{}{}", chars[i], chars[j]).parse::<u64>().unwrap();
                    if combine > largest {
                        largest = combine
                    }
                }
            }

            largest
        })
        .sum();

    Some(result)
}

fn g(bank: &[u64], remaining: u64, total: u64) -> u64 {
    if remaining == 0 {
        return total;
    }

    let (i, largest) = bank[..bank.len() - remaining as usize + 1]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, v)| *v)
        .unwrap();

    g(
        &bank[i + 1..],
        remaining - 1,
        total + largest * 10u64.pow(remaining as u32 - 1),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|lines| {
            let bank: Vec<u64> = lines
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();
            g(&bank, 12, 0)
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
