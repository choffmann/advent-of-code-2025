advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current = 50;
    let mut result = 0;

    input.lines().for_each(|line| {
        let value: i32 = line.split_at(1).1.parse().unwrap();
        let direction = if line.starts_with("L") { -1 } else { 1 };
        current = ((current % 100) + ((((direction * value) % 100) + 100) % 100)) % 100;
        if current == 0 {
            result += 1
        }
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current = 50;
    let mut result = 0;

    input.lines().for_each(|line| {
        let value: i32 = line.split_at(1).1.parse().unwrap();
        let direction = if line.starts_with("L") { -1 } else { 1 };
        let zero_passed = if line.starts_with("L") {
            let inverted = if current > 0 { 100 - current } else { 0 };
            ((inverted + value) / 100).abs()
        } else {
            ((current + value) / 100).abs()
        };
        current = ((current % 100) + ((((direction * value) % 100) + 100) % 100)) % 100;
        result += zero_passed
    });

    Some(result.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
