advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let id_ranges: Vec<_> = lines
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let (start, end) = x.split_once('-')?;
            Some(start.parse::<usize>().ok()?..=end.parse().ok()?)
        })
        .collect::<Option<_>>()?;

    let result = lines
        .map(|l| l.parse::<u64>().ok())
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .map(|id| id_ranges.iter().any(|r| r.contains(&(id as usize))) as u64)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<(usize, usize)> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let (start, end) = x.split_once('-')?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect::<Option<_>>()?;

    ranges.sort_unstable_by_key(|&(s, _)| s);

    let mut total: u64 = 0;
    let mut cur_start = ranges[0].0;
    let mut cur_end = ranges[0].1;

    for &(s, e) in &ranges[1..] {
        if s <= cur_end.saturating_add(1) {
            cur_end = cur_end.max(e)
        } else {
            total += (cur_end - cur_start + 1) as u64;
            cur_start = s;
            cur_end = e;
        }
    }

    total += (cur_end - cur_start + 1) as u64;
    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
