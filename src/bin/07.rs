use std::fmt::Debug;

advent_of_code::solution!(7);

impl Into<Map> for &str {
    fn into(self) -> Map {
        let inner: Vec<Vec<_>> = self
            .lines()
            .map(|row| row.chars().map(|c| MapPoint { value: c }).collect())
            .collect();
        Map {
            split_count: 0,
            cur_pos: vec![Point {
                x: inner[0].len() / 2,
                y: 0,
            }],
            inner,
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            for cell in row {
                write!(f, "{}", cell.value)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "splits: {}", self.split_count)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    split_count: u64,
    cur_pos: Vec<Point>,
    inner: Vec<Vec<MapPoint>>,
}

struct MapPoint {
    value: char,
}

pub struct BeamIter<'a> {
    map: &'a mut Map,
}

impl Map {
    fn width(&self) -> usize {
        self.inner.first().map(|r| r.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.inner.len()
    }

    fn get(&self, p: Point) -> Option<&MapPoint> {
        self.inner.get(p.y).and_then(|row| row.get(p.x))
    }

    fn get_mut(&mut self, p: Point) -> Option<&mut MapPoint> {
        self.inner.get_mut(p.y).and_then(|row| row.get_mut(p.x))
    }

    fn down(&self, p: Point) -> Option<Point> {
        let y = p.y + 1;
        (y < self.height()).then_some(Point { x: p.x, y })
    }

    fn step(&mut self, p: Point) {
        if let Some(cell) = self.get_mut(p) {
            cell.value = '|';
        }
    }

    fn split(&mut self, p: Point) -> impl Iterator<Item = Point> {
        let w = self.width();
        let y = p.y;

        let mut out = Vec::new();

        if p.x > 0 {
            let left = Point { x: p.x - 1, y };
            if let Some(cell) = self.get_mut(left) {
                cell.value = '|';
                out.push(left);
            }
        }

        if p.x + 1 < w {
            let right = Point { x: p.x + 1, y };
            if let Some(cell) = self.get_mut(right) {
                cell.value = '|';
                out.push(right);
            }
        }

        self.split_count += 1;
        out.into_iter()
    }

    fn beam(&mut self) -> Option<()> {
        let positions = self.cur_pos.clone();
        let mut next_positions = Vec::new();

        for pos in positions {
            let Some(n) = self.down(pos) else { continue };

            let v = self.get(n).map(|mp| mp.value);

            match v {
                Some('.') => {
                    self.step(n);
                    next_positions.push(n);
                }
                Some('^') => next_positions.extend(self.split(n)),
                Some(_) => {}
                None => {}
            }
        }

        self.cur_pos = next_positions;

        (!self.cur_pos.is_empty()).then_some(())
    }

    pub fn beam_iter(&mut self) -> BeamIter<'_> {
        BeamIter { map: self }
    }
}

impl<'a> Iterator for BeamIter<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let map = &mut *self.map;

        if map.cur_pos.is_empty() {
            return None;
        }

        map.beam()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map: Map = input.into();
    let mut it = map.beam_iter();
    while it.next().is_some() {}

    // println!("{:?}", map);

    Some(map.split_count)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
