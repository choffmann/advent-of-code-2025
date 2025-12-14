advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let directions: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let fields: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut marks: u64 = 0;

    for (i, row) in fields.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '@' {
                continue;
            }

            let neighbors = directions
                .iter()
                .filter_map(|&(di, dj)| {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni < 0 || nj < 0 {
                        return None;
                    }
                    fields
                        .get(ni as usize)
                        .and_then(|r| r.get(nj as usize))
                        .copied()
                })
                .filter(|&c| c == '@')
                .count();

            if neighbors < 4 {
                marks += 1;
            } 
        }
    }

    Some(marks)
}

fn count_neighbors(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    DIRECTIONS
        .iter()
        .filter_map(|&(di, dj)| {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || nj < 0 {
                return None;
            }
            grid.get(ni as usize)?.get(nj as usize)
        })
        .filter(|&&c| c == '@')
        .count()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut loops = 0;

    loop {
        let mut to_remove = Vec::new();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '@' && count_neighbors(&grid, i, j) < 4 {
                    to_remove.push((i, j));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (i, j) in &to_remove {
            grid[*i][*j] = '.';
        }

        loops += to_remove.len();
    }

    Some(loops.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
