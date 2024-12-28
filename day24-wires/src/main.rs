use std::{collections::{HashMap, VecDeque}, ops::BitOr};

#[derive(Debug,Clone,Copy)]
enum Op {
    XOR,
    OR,
    AND
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        match input {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => panic!("Invalid operation")
        }
    }
}

type Operation = (String, Op, String, String);

#[derive(Debug)]
struct Puzzle {
    gates: HashMap<String, bool>,
    operations: Vec<Operation>
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let (gate_data, operations_data) = input.split_once("\n\n").unwrap();
        let gates = gate_data.lines().map(|line| {
            let (gate, value) = line.split_once(": ").unwrap();

            (gate.to_string(), value == "1")
        }).collect();

        let operations = operations_data.lines().map(|line| {
            let mut parts = line.split_whitespace();
            let input1 = parts.next().unwrap();
            let operation = parts.next().unwrap().into();
            let input2 = parts.next().unwrap();
            parts.next(); // Skip "->"
            let output = parts.next().unwrap();

            (input1.to_string(), operation, input2.to_string(), output.to_string())
        }).collect();

        Self {
            gates,
            operations
        }
    }
}

impl Puzzle {
    fn solve(&mut self, operation: &Operation) -> Option<bool> {
        let (input1, operation, input2, output) = operation;

        let &value1 = self.gates.get(input1)?;
        let &value2 = self.gates.get(input2)?;

        let result = match operation {
            Op::AND => value1 & value2,
            Op::OR => value1 | value2,
            Op::XOR => value1 ^ value2
        };

        self.gates.insert(output.clone(), result);

        Some(result)
    }

    fn solve_all(&mut self) {
        let mut operation_queue = VecDeque::from(self.operations.clone());

        while let Some(operation) = operation_queue.pop_front() {
            if self.solve(&operation).is_none() {
                operation_queue.push_back(operation);
            }
        }
    }

    fn score(&self) -> usize {
        let mut z_gates: Vec<(&String, &bool)> = self.gates.iter().filter(|(gate, _)| gate.starts_with("z")).collect();
        z_gates.sort_unstable_by_key(|&(gate, _)| gate.clone() );

        z_gates.iter().enumerate().fold(0, |acc, (i, (_, &value))| {
            acc | (value as usize) << i
        })
    }
}

fn main() {
    let input = aoc::input();
    let mut puzzle: Puzzle = input.as_str().into();

    puzzle.solve_all();
    println!("Part 1: {}", puzzle.score());
}
