use std::fmt;

use aoc::{Map, Point};

enum Tile {
    Racetrack,
    Wall,
    Start,
    Finish,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Racetrack,
            'S' => Tile::Start,
            'E' => Tile::Finish,
            _ => panic!("Invalid tile"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Racetrack => '.',
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::Finish => 'E',
        };
        write!(f, "{}", c)
    }
}

struct Track {
    map: Map<Tile>,
    start: Point,
    length: usize,
}

impl From<&str> for Track {
    fn from(s: &str) -> Self {
        let map = Map::from(s);
        let mut start = (0, 0);
        let mut length = 0;

        map.iter().for_each(|(point, tile)| match tile {
            Tile::Racetrack => length += 1,
            Tile::Finish => length += 1,
            Tile::Start => start = point,
            _ => (),
        });

        Self { map, length, start }
    }
}

impl Track {
    fn times_per_positon(&self) -> Vec<Vec<usize>> {
        let mut position = self.start;
        let mut tiles_left = self.length;
        let mut times = vec![vec![usize::MAX; self.map.height]; self.map.width];

        loop {
            times[position.0][position.1] = tiles_left;
            if tiles_left == 0 {
                break;
            }
            tiles_left -= 1;


            for (np, tile) in self.map.cardinal_neighbours(position) {
                if times[np.0][np.1] != usize::MAX {
                    // We already visited this position
                    continue;
                }

                if matches!(tile, Some(Tile::Racetrack) | Some(Tile::Finish)) {
                    position = np;
                    break;
                }
            }
        }

        times
    }

    fn completion_times(&self, cheats: usize) -> Vec<usize> {
        let mut times = vec![];
        let times_per_position = self.times_per_positon();

        for (point, tile) in self.map.iter() {
            if matches!(tile, Tile::Racetrack | Tile::Start) {
                let normal_time = times_per_position[point.0][point.1];

                // Draw a cheat-size diamond around this point and see if there's time to save
                for i in 0..=cheats * 2 {
                    for j in 0..=cheats * 2 {
                        let distance = i.abs_diff(cheats) + j.abs_diff(cheats);

                        if distance <= cheats {
                            let x = point.0.wrapping_add_signed(i as isize - cheats as isize);
                            let y = point.1.wrapping_add_signed(j as isize - cheats as isize);

                            if let Some(cheat_time) =
                                times_per_position.get(x).and_then(|row| row.get(y))
                            {
                                if *cheat_time == usize::MAX {
                                    continue;
                                }
                                let new_time = *cheat_time + distance;
                                if normal_time > new_time {
                                    times.push(normal_time - new_time);
                                }
                            }
                        }
                    }
                }
            }
        }

        times
    }
}

fn main() {
    let input = aoc::input();
    let track = Track::from(input.trim());

    println!("Track:\n{}", track.map);

    println!("Part 1: {}", track.completion_times(2).iter().filter(|&t| *t >= 100).count());
    println!("Part 2: {}", track.completion_times(20).iter().filter(|&t| *t >= 100).count());
}
