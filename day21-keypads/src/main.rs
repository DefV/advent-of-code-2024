use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Button {
    Number(char),
    Enter,
    Movement(isize, isize),
}

impl Display for Button {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Button::Number(c) => write!(f, "{}", c),
            Button::Enter => write!(f, "A"),
            Button::Movement(dx, dy) => {
                match (dx, dy) {
                    (-1, 0) => write!(f, "<"),
                    (1, 0) => write!(f, ">"),
                    (0, -1) => write!(f, "^"),
                    (0, 1) => write!(f, "v"),
                    _ => unreachable!(),
                }
            },
        }
    }
}

fn display_buttons(buttons: &[Button]) -> String {
    buttons.iter().map(|b| b.to_string()).collect::<String>()
}

struct Keypad {
    keys: HashMap<Button, (isize, isize)>,
    position: (isize, isize),
}

impl Keypad {
    fn fastest_move_and_enter(&mut self, button: Button) -> Vec<Button> {
        let mut paths: Vec<Vec<Button>> = vec![];
        let &(dx,dy) = self.keys.get(&button).unwrap();
        let (x, y) = self.position;

        let mut shortest_path = usize::MAX;
        let mut queue = vec![(vec![], x, y)];

        while let Some((path, x, y)) = queue.pop() {
            if path.len() > shortest_path {
                // Abandon paths that are longer than the shortest path found so far
                continue;
            }

            if (dx, dy) == (x, y) {
                shortest_path = path.len();
                paths.push(path);
                continue;
            }

            if dx > x && self.is_position_available(x + 1, y) {
                let mut new_path = path.clone();
                new_path.push(RIGHT);
                queue.push((new_path, x + 1, y));
            }

            if dy < y && self.is_position_available(x, y - 1) {
                let mut new_path = path.clone();
                new_path.push(UP);
                queue.push((new_path, x, y - 1));
            }

            if dy > y && self.is_position_available(x, y + 1) {
                let mut new_path = path.clone();
                new_path.push(DOWN);
                queue.push((new_path, x, y + 1));
            }

            if dx < x && self.is_position_available(x - 1, y) {
                let mut new_path = path.clone();
                new_path.push(LEFT);
                queue.push((new_path, x - 1, y));
            }
        }

        self.position = (dx, dy);

        // Prefer parts that are short, have identical moves together, and goes left first
        let mut fastest_path = paths.iter().max_by_key(|path| {
            let mut score = 0isize;
            let mut last_move = Button::Enter;
            for (idx, &mv) in path.iter().enumerate() {
                if mv == last_move {
                    score += 1000;
                }

                if mv == LEFT {
                    score -= idx as isize * 2;
                }

                if mv == DOWN {
                    score -= idx as isize;
                }

                last_move = mv;
            }
            score
        }).unwrap().clone();
        fastest_path.push(Button::Enter);
        fastest_path
    }

    fn move_and_enter(&mut self, button: Button) -> Vec<Button> {
        let mut moves_needed = vec![];
        let &(dx,dy) = self.keys.get(&button).unwrap();
        let (mut x, mut y) = self.position;

        while (dx, dy) != (x, y) {
            if dx > x && self.is_position_available(x + 1, y) {
                x += 1;
                moves_needed.push(RIGHT);
            } else if dy < y && self.is_position_available(x, y - 1) {
                y -= 1;
                moves_needed.push(UP);
            } else if dy > y && self.is_position_available(x, y + 1) {
                y += 1;
                moves_needed.push(DOWN);
            } else if dx < x && self.is_position_available(x - 1, y) {
                x -= 1;
                moves_needed.push(LEFT);
            }
        }

        moves_needed.push(Button::Enter);

        // println!("Moves to get from {:?} to {:?}: {}", self.position, (dx, dy),display_buttons(&moves_needed));
        self.position = (dx, dy);

        moves_needed
    }

    fn is_position_available(&self, x: isize, y: isize) -> bool {
        self.keys.values().any(|&pos| pos == (x, y))
    }
}

const LEFT: Button = Button::Movement(-1, 0);
const RIGHT: Button = Button::Movement(1, 0);
const UP: Button = Button::Movement(0, -1);
const DOWN: Button = Button::Movement(0, 1);

struct Puzzle {
    keypads: Vec<Keypad>,
    code: Vec<Button>,
}

impl Puzzle {
    fn numeric_keypad() -> Keypad {
        Keypad {
            keys: [
                (Button::Number('7'), (0, 0)),
                (Button::Number('8'), (1, 0)),
                (Button::Number('9'), (2, 0)),
                (Button::Number('4'), (0, 1)),
                (Button::Number('5'), (1, 1)),
                (Button::Number('6'), (2, 1)),
                (Button::Number('1'), (0, 2)),
                (Button::Number('2'), (1, 2)),
                (Button::Number('3'), (2, 2)),
                (Button::Number('0'), (1, 3)),
                (Button::Enter, (2, 3)),
            ]
            .into(),
            position: (2, 3),
        }
    }

    fn directional_keypad() -> Keypad {
        Keypad {
            keys: [
                (LEFT, (0, 1)),
                (RIGHT, (2, 1)),
                (UP, (1, 0)),
                (DOWN, (1, 1)),
                (Button::Enter, (2, 0)),
            ]
            .into(),
            position: (2, 0),
        }
    }

    fn part1(code: &str) -> Self {
        Puzzle {
            keypads: vec![
                Self::numeric_keypad(),
                Self::directional_keypad(),
                Self::directional_keypad(),
            ],
            code: code.chars().map(|c| Button::Number(c) ).collect(),
        }
    }

    fn moves_to_solve(&mut self) -> Vec<Button> {
        let mut moves_needed = vec![];
        let mut buttons_needed: Vec<Button> = self.code.clone();
        buttons_needed.push(Button::Enter);

        println!("Moves needed: {}", display_buttons(&buttons_needed));
        for keypad in &mut self.keypads {
            moves_needed = vec![];

            for button in buttons_needed {
                moves_needed.extend(keypad.fastest_move_and_enter(button));
            }
            println!("Moves needed: {}", display_buttons(&moves_needed));
            buttons_needed = moves_needed.clone();
        }

        moves_needed.clone()
    }
}

fn main() {
    let input = aoc::input();
    let score = input.trim().lines().map(|line| {
        let number = &line[..line.len() - 1];
        let mut puzzle = Puzzle::part1(number);
        let moves = puzzle.moves_to_solve();

        println!("{}A: {} ({})", number, moves.iter().map(|b| b.to_string()).collect::<String>(), moves.len());
        number.parse::<usize>().unwrap() * moves.len()
    });

    println!("Part 1 score is: {}", score.sum::<usize>());
}
