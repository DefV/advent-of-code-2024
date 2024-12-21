use std::{collections::BinaryHeap, fmt};

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

#[derive(PartialEq, Eq)]
struct RacePosition {
    position: Point,
    cheats: usize,
    visited: Vec<Point>,
    time: usize,
}

impl Ord for RacePosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for RacePosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Track {
    fn times_per_positon(&self) -> Vec<Vec<usize>> {
        let mut position = self.start;
        let mut tiles_left = self.length;
        let mut times = vec![vec![0; self.map.height]; self.map.width];

        loop {
            times[position.0][position.1] = tiles_left;
            tiles_left -= 1;

            if tiles_left == 0 {
                break;
            }

            for (np, tile) in self.map.cardinal_neighbours(position) {
                if times[np.0][np.1] > 0 {
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

    const MIN_TIME_TO_SAVE: usize = 100;
    fn completion_times(&self, cheats: usize) -> Vec<usize> {
        let mut times = vec![];
        let mut queue: BinaryHeap<RacePosition> = BinaryHeap::from([RacePosition {
            position: self.start,
            cheats: cheats,
            visited: vec![self.start],
            time: 0,
        }]);

        let times_per_position = self.times_per_positon();

        while let Some(RacePosition { position, time , cheats, visited}) = queue.pop() {
            for (np, tile) in self.map.cardinal_neighbours(position) {
                let mut visited = visited.clone();
                if visited.contains(&np) {
                    continue;
                } else {
                    visited.push(np);
                }

                match tile {
                    Some(Tile::Racetrack) => {
                        if cheats == 0 {
                            let calculated_time = time + 1 + times_per_position[np.0][np.1];

                            if calculated_time < self.length {
                                times.push(calculated_time);
                            }

                            continue;
                        }

                        queue.push(RacePosition {
                            position: np,
                            time: time + 1,
                            cheats,
                            visited
                        });
                    },
                    Some(Tile::Finish) => {
                        if time + 1 < self.length {
                            times.push(time + 1);
                        }
                    },
                    Some(Tile::Wall) => {
                        if cheats > 0 {
                            queue.push(RacePosition {
                                position: np,
                                time: time + 1,
                                cheats: cheats - 1,
                                visited: visited
                            });
                        }
                    }
                    _ => (),
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
    let completion_times = track.completion_times(1);
    for time in &completion_times {
        println!("Time saved: {}", track.length - time);
    }
    println!("Completion times: {:?}", completion_times.iter().filter(|&t| track.length - t >= 100).count());
}
