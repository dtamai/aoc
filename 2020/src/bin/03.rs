fn main() -> eyre::Result<()> {
    let map = std::fs::read_to_string("data/03.txt")?;
    let map = Map::parse(map);
    let slope = Vec2(3, 1);

    // first part
    let count = count_trees(&map, slope);
    println!("{} trees until the bottom", count);

    // second part
    let slopes = vec![Vec2(1, 1), Vec2(3, 1), Vec2(5, 1), Vec2(7, 1), Vec2(1, 2)];
    let prod: usize = slopes.into_iter().map(|s| count_trees(&map, s)).product();
    println!("{} for multiplying all tree counts", prod);

    Ok(())
}

fn count_trees(map: &Map, slope: Vec2) -> usize {
    let me = Toboggan {};

    let mut count = 0;
    for pos in me.slide_to_bottom(&map, slope) {
        match pos {
            Position::Open => continue,
            Position::Tree => count += 1,
        }
    }

    count
}

#[derive(Clone, Copy, Debug)]
struct Vec2(usize, usize);

impl Vec2 {
    fn wrapping_x_add(self, rhs: Self, max: usize) -> Self {
        let new_x = (self.0 + rhs.0) % max;

        Vec2(new_x, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy, Debug)]
enum Position {
    Open,
    Tree,
}

#[derive(Debug)]
struct Map {
    lines: Vec<Vec<Position>>,
}

impl Map {
    fn parse(raw: String) -> Self {
        let lines: Vec<Vec<Position>> = raw
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Position::Open,
                        '#' => Position::Tree,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Map { lines }
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn width(&self) -> usize {
        self.lines[0].len()
    }

    fn at(&self, coord: Vec2) -> Option<Position> {
        if coord.1 >= self.height() {
            return None;
        }

        Some(self.lines[coord.1][coord.0])
    }
}

struct SlideIterator<'m> {
    map: &'m Map,
    slope: Vec2,
    curr_coord: Vec2,
}

impl<'m> Iterator for SlideIterator<'m> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let new_coord = self.curr_coord.wrapping_x_add(self.slope, self.map.width());
        self.curr_coord = new_coord;
        self.map.at(new_coord)
    }
}

struct Toboggan {}

impl Toboggan {
    fn slide_to_bottom<'m>(&self, map: &'m Map, slope: Vec2) -> SlideIterator<'m> {
        SlideIterator {
            map,
            slope,
            curr_coord: Vec2(0, 0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let map = "..##.......\n\
                        #...#...#..\n\
                        .#....#..#.\n\
                        ..#.#...#.#\n\
                        .#...##..#.\n\
                        ..#.##.....\n\
                        .#.#.#....#\n\
                        .#........#\n\
                        #.##...#...\n\
                        #...##....#\n\
                        .#..#...#.#";

        let map = Map::parse(map.to_owned());
        assert_eq!(7, count_trees(&map, Vec2(3, 1)));
    }
}
