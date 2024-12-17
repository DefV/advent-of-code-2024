#[derive(Debug, Clone)]
struct Computer {
    registers: [u64;3],
    opcodes: Vec<u8>,
    instruction_index: u8,
    output: Vec<u8>
}

// register-indexes
const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

// Opcodes
const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

impl Default for Computer {
    fn default() -> Self {
        Computer {
            registers: [0;3],
            opcodes: Vec::new(),

            instruction_index: 0,
            output: Vec::new()
        }
    }
}

impl From<&str> for Computer {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let mut registers = [0;3];
        registers[A] = lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
        registers[B] = lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
        registers[C] = lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
        lines.next();
        let opcodes = lines.next().unwrap().split_once(": ").unwrap().1.split(",").map(|x| x.parse().unwrap()).collect();

        Computer {
            registers,
            opcodes,
            ..Self::default()
        }
    }

}

impl Computer {
    fn combo_value(&self, code: u8) -> u64 {
        match code {
            0..=3 => code as u64,
            4 => self.registers[A],
            5 => self.registers[B],
            6 => self.registers[C],
            _ => panic!("Invalid code")
        }
    }

    fn step(&mut self, opcode: u8, operand: u8) -> u8 {
        let combo_value = || self.combo_value(operand);
        let literal_value = operand as u64;

        match opcode {
            ADV => { // 0
                self.registers[A] = self.registers[0] / 2u64.pow(combo_value() as u32);
            },
            BXL => { // 1
                self.registers[B] = self.registers[B] ^ literal_value;
            },
            BST => { // 2
                self.registers[B] = combo_value() % 8;
            },
            JNZ => { // 3
                if self.registers[A] != 0 {
                    return literal_value as u8
                }
            },
            BXC => { // 4
                self.registers[B] = self.registers[B] ^ self.registers[C];
            },
            OUT => { // 5
                self.output.push((combo_value() % 8) as u8);
            },
            BDV => { // 6
                self.registers[B] = self.registers[0] / 2u64.pow(combo_value() as u32);
            },
            CDV => { // 7
                self.registers[C] = self.registers[0] / 2u64.pow(combo_value() as u32);
            },
            _ => panic!("Invalid opcode")
        }

        self.instruction_index + 2
    }

    fn run(&mut self) {
        while let Some(&opcode) = self.opcodes.get(self.instruction_index as usize) {
            let operand = self.opcodes[self.instruction_index as usize + 1];
            let next_instruction = self.step(opcode, operand);
            self.instruction_index = next_instruction;
        }
    }
}

fn main() {
    let input = aoc::input();
    let input = input.trim();
    let computer = Computer::from(input);
    let mut part1_computer = computer.clone();
    part1_computer.run();

    let output: String = part1_computer.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    println!("Part 1: {}", output);

    let mut candidates = vec![(0, 0u64)];
    let command_length = computer.opcodes.len();
    let mut correct_answers: Vec<u64> = vec![];

    while let Some((digit, candidate)) = candidates.pop() {
        for x in 0..=7 {
            let mut part2_computer = computer.clone();
            part2_computer.registers[A] = candidate + x;
            part2_computer.run();

            if part2_computer.opcodes[command_length - digit - 1] == part2_computer.output[0] {
                if part2_computer.opcodes == part2_computer.output {
                    correct_answers.push(candidate + x);
                } else {
                    candidates.push((digit + 1, (candidate + x) * 8));
                }
            }
        }
    }

    println!("All correct answers: {:?}", correct_answers);
    println!("Part 2: {}", correct_answers.iter().min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_computer {
        ($registers:expr, $opcodes:expr) => {
            Computer {
                registers: $registers,
                opcodes: $opcodes,
                ..Computer::default()
            }
        };
    }

    #[test]
    fn test_run_example1() {
        let mut computer = test_computer!([0, 0, 9], vec![2, 6]);

        computer.step(2, 6);
        assert_eq!(computer.registers, [0, 1, 9]);
    }

    #[test]
    fn test_run_example2() {
        let mut computer = test_computer!([10, 0, 0], vec![5,0,5,1,5,4]);

        computer.run();

        assert_eq!(computer.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_run_example3() {
        let mut computer = test_computer!([2024, 0, 0], vec![0,1,5,4,3,0]);

        computer.run();

        assert_eq!(computer.output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(computer.registers[A], 0);
    }

    #[test]
    fn test_run_example4() {
        let mut computer = test_computer!([0, 29, 0], vec![1,7]);

        computer.run();

        assert_eq!(computer.registers[B], 26);
    }

    #[test]
    fn test_run_example5() {
        let mut computer = test_computer!([0, 2024, 43690], vec![4, 0]);

        computer.run();

        assert_eq!(computer.registers[B], 44354);
    }
}
