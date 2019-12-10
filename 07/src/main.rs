use std::collections::VecDeque;

fn main() {
    let memory = include_str!("example")
        .split(',')
        .map(str::trim)
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    // Part 1
    // let permutations = gen_permutations([0, 1, 2, 3, 4]);
    let permutations = gen_permutations([5, 6, 7, 8, 9]);

    let mut max_signal = std::i32::MIN;
    for p in permutations {
        let output = try_sequence(&memory, p);
        max_signal = max_signal.max(output);
    }

    println!("{:?}", max_signal);
}

fn gen_permutations(sequence: [i32; 5]) -> Vec<[i32; 5]> {
    let mut out = vec![];
    fn rec(s: [i32; 5], idx: usize, out: &mut Vec<[i32; 5]>) {
        if idx == s.len() {
            out.push(s.clone());
        } else {
            let mut new_s = s.clone();
            for i in idx..s.len() {
                new_s.swap(i, idx);
                rec(new_s, idx+1, out);
                new_s.swap(i, idx);
            }
        }
    };

    rec(sequence, 0, &mut out);

    out
}

fn try_sequence(memory: &[i32], sequence: [i32; 5]) -> i32 {
    let mut prev_out = 0;
    // Part 1
    //for phase_setting in &sequence {
    //    let mut memory_clone = memory.to_vec();
    //    let out = execute(&mut memory_clone, &[*phase_setting, prev_out]);
    //    println!("{:?}", out);
    //    prev_out = out[0];
    //}

    // Part 2
    // Initialize each amplifier with a copy of the program.
    let mut amplifiers = [
        Program::init(memory.to_vec()), // -\
        Program::init(memory.to_vec()), //  |
        Program::init(memory.to_vec()), //  + lol
        Program::init(memory.to_vec()), //  |
        Program::init(memory.to_vec()), // -/
    ];

    let mut outputs = VecDeque::new();

    let first_output = amplifiers[0].execute(&[sequence[0], 0]);
    outputs.push_back(first_output);

    for (i, amplifier) in amplifiers.iter_mut().enumerate().skip(1) {
        let prev_output = outputs.pop_front().unwrap();
        // Execute the amplifier with its phase sequence. Assume it does not
        // output anything without getting more input.
        amplifier.execute(&[sequence[i]]);
        // Then continue execution with the previous amplifier's output.
        let output = amplifier.execute(&prev_output);
        outputs.push_back(output);
    }

    let mut current = 0;
    loop {
        let prev_output = outputs.pop_front().unwrap();
        let next_output = amplifiers[current].execute(&prev_output);
        if amplifiers[4].halted {
            break next_output[0];
        } else {
            outputs.push_back(next_output);
            current = (current + 1) % 5;
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    In(Parameter),
    Out(Parameter),
    Halt,
    Jnz(Parameter, Parameter),
    Jz(Parameter, Parameter),
    Lt(Parameter, Parameter, Parameter),
    Eq(Parameter, Parameter, Parameter),
}

#[derive(Debug)]
enum ParameterMode {
    Immediate,
    Position,
}

impl From<u8> for ParameterMode {
    fn from(byte: u8) -> Self {
        use ParameterMode::*;
        match byte {
            0 => Position,
            1 => Immediate,
            _ => panic!("Unknown parameter mode!"),
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
}

impl Parameter {
    fn get_value(&self, memory: &[i32]) -> i32 {
        match self.mode {
            ParameterMode::Immediate => self.value,
            ParameterMode::Position => memory[self.value as usize],
        }
    }
}

fn decode_instruction(memory: &[i32], i_ptr: usize) -> Instruction {
    let opcode = memory[i_ptr];
    let opcode_bytes: Vec<u8> = opcode.to_string().as_bytes().iter().map(|b| b - 48).collect();
    let p_ptr: i32 = opcode_bytes.len() as i32 - 3;
    let inst = [if p_ptr <= -2 { 0 } else { opcode_bytes[(p_ptr  + 1) as usize] }, opcode_bytes[(p_ptr + 2) as usize]];

    use Instruction::*;
    match inst {
        [0, 1] => {
            // Add, 3 parameters.
            let mut params = get_params(&memory, i_ptr, 3, p_ptr, &opcode_bytes);
            Add(params.next().unwrap(), params.next().unwrap(), params.next().unwrap())
        },
        [0, 2] => {
            // Mul, 3 parameters.
            let mut params = get_params(&memory, i_ptr, 3, p_ptr, &opcode_bytes);
            Mul(params.next().unwrap(), params.next().unwrap(), params.next().unwrap())
        },
        [0, 3] => {
            // In, 1 parameter.
            let mut params = get_params(&memory, i_ptr, 1, p_ptr, &opcode_bytes);
            In(params.next().unwrap())
        },
        [0, 4] => {
            // Out, 1 parameter.
            let mut params = get_params(&memory, i_ptr, 1, p_ptr, &opcode_bytes);
            Out(params.next().unwrap())
        },
        [0, 5] => {
            // Jump if not equal to zero, 2 parameters.
            let mut params = get_params(&memory, i_ptr, 2, p_ptr, &opcode_bytes);
            Jnz(params.next().unwrap(), params.next().unwrap())
        },
        [0, 6] => {
            // Jump if equal to zero, 2 parameters.
            let mut params = get_params(&memory, i_ptr, 2, p_ptr, &opcode_bytes);
            Jz(params.next().unwrap(), params.next().unwrap())
        },
        [0, 7] => {
            // Less than, 3 parameters.
            let mut params = get_params(&memory, i_ptr, 3, p_ptr, &opcode_bytes);
            Lt(params.next().unwrap(), params.next().unwrap(), params.next().unwrap())
        },
        [0, 8] => {
            // Equal to, 3 parameters.
            let mut params = get_params(&memory, i_ptr, 3, p_ptr, &opcode_bytes);
            Eq(params.next().unwrap(), params.next().unwrap(), params.next().unwrap())
        },
        [9, 9] => Halt,
        _ => panic!("Unknown opcode!"),
    }
}

fn get_params<'mem>(
    memory: &'mem [i32],
    i_ptr: usize,
    n: usize,
    p_ptr: i32,
    opcode_bytes: &'mem [u8]
) -> impl Iterator<Item=Parameter> + 'mem {
    (0..n).into_iter().map(move |i| {
        let p_offset = p_ptr - i as i32;
        let mode = if p_offset < 0 { ParameterMode::Position } else { opcode_bytes[p_offset as usize].into() };
        Parameter { mode, value: memory[1 + i_ptr + i as usize] }
    })
}

#[derive(Debug)]
struct Program {
    memory: Vec<i32>,
    halted: bool,
    i_ptr: usize,
}

impl Program {

    fn init(memory: Vec<i32>) -> Self {
        Self {
            memory,
            halted: false,
            i_ptr: 0,
        }
    }

    fn execute(&mut self, input: &[i32]) -> Vec<i32> {
        let mut memory = &mut self.memory;
        let mut input_pos = 0;
        let mut output = vec![];

        loop {
            let instruction = decode_instruction(&memory, self.i_ptr);
            use Instruction::*;
            match instruction {
                Halt => {
                    self.halted = true;
                    break output;
                },
                Add(p1, p2, p3) => {
                    let store = p3.value as usize;
                    let val = p1.get_value(&memory) + p2.get_value(&memory);
                    memory[store] = val;

                    self.i_ptr += 4;
                },
                Mul(p1, p2, p3) => {
                    let store = p3.value as usize;
                    let val = p1.get_value(&memory) * p2.get_value(&memory);
                    memory[store] = val;

                    self.i_ptr += 4;
                },
                In(p1) => {
                    let store = p1.value as usize;
                    if input_pos >= input.len() {
                        break output
                    }
                    memory[store] = input[input_pos];

                    input_pos += 1;
                    self.i_ptr += 2;
                },
                Out(p1) => {
                    let val = p1.get_value(&memory);
                    output.push(val);

                    self.i_ptr += 2;
                },
                Jnz(p1, p2) => {
                    let val = p1.get_value(&memory);
                    if val != 0 {
                        self.i_ptr = p2.get_value(&memory) as usize;
                    } else {
                        self.i_ptr += 3;
                    }
                },
                Jz(p1, p2) => {
                    let val = p1.get_value(&memory);
                    if val == 0 {
                        self.i_ptr = p2.get_value(&memory) as usize;
                    } else {
                        self.i_ptr += 3;
                    }
                },
                Lt(p1, p2, p3) => {
                    if p1.get_value(&memory) < p2.get_value(&memory) {
                        memory[p3.value as usize] = 1;
                    } else {
                        memory[p3.value as usize] = 0;
                    }

                    self.i_ptr += 4;
                },
                Eq(p1, p2, p3) => {
                    if p1.get_value(&memory) == p2.get_value(&memory) {
                        memory[p3.value as usize] = 1;
                    } else {
                        memory[p3.value as usize] = 0;
                    }

                    self.i_ptr += 4;
                },
            }
        }
    }
}
