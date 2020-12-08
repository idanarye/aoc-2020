use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let mut parts = line.split(" ");
        let op = parts.next().unwrap();
        let arg = parts.next().unwrap().parse().unwrap();
        if parts.next().is_some() {
            panic!("Too many parts in {:?}", line);
        }
        match op {
            "nop" => Instruction::Nop(arg),
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            _ => panic!("Unsupported op {:?}", op),
        }
    }).collect()
}

fn run_program(program: &[Instruction]) -> Result<isize, isize> {
    let mut instruction_index = 0;
    let mut visited_instructions = HashSet::new();
    let mut accumulator = 0;

    loop {
        if instruction_index == program.len() {
            return Ok(accumulator);
        }
        if visited_instructions.contains(&instruction_index) {
            return Err(accumulator);
        }
        visited_instructions.insert(instruction_index);
        match program[instruction_index] {
            Instruction::Nop(_) => {
                instruction_index += 1;
            }
            Instruction::Acc(arg) => {
                accumulator += arg;
                instruction_index += 1;
            }
            Instruction::Jmp(arg) => {
                instruction_index = (instruction_index as isize + arg) as usize;
            }
        }
    }
}

pub fn part_1(input: &[Instruction]) -> isize {
    run_program(input).err().unwrap()
}

pub fn part_2(input: &[Instruction]) -> isize {
    let mut modified_program: Vec<Instruction> = input.iter().cloned().collect();
    for (idx, orig_instruction) in input.iter().enumerate() {
        modified_program[idx] = match orig_instruction {
            Instruction::Acc(_) => continue,
            Instruction::Nop(arg) => Instruction::Jmp(*arg),
            Instruction::Jmp(arg) => Instruction::Nop(*arg),
        };
        if let Ok(result) = run_program(&modified_program) {
            return result;
        }
        modified_program[idx] = orig_instruction.clone();
    }
    panic!("No can do");
}
