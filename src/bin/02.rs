advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .filter(|&i: &u64| {
            let s = i.to_string();
            let (a, b) = s.split_at(s.len() / 2);
            a == b
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result: u64 = input
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .filter(|&i: &u64| {
            let s = i.to_string();
            let bytes = s.as_bytes();
            let n = bytes.len();

            if (1..=n / 2).filter(|&k| n % k == 0).any(|k| {
                let repeats = n / k;
                repeats >= 2 && bytes.chunks(k).all(|chunk| chunk == &bytes[..k])
            }) {
                // println!("invalid id: {}", i);
                true
            } else {
                false
            }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
