fn main() {
    let mut memory = include_str!("input")
        .split(',')
        .map(str::trim)
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();


    // Give the computer an extra 40000 slots to work with.
    memory.extend_from_slice(&vec![0; 40000]);

    let mut program = Program::init(memory);
    let output = &program.execute(&[2]);

    println!("{:?}", output);
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
    ChangeBase(Parameter),
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

impl From<u8> for ParameterMode {
    fn from(byte: u8) -> Self {
        use ParameterMode::*;
        match byte {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Unknown parameter mode!"),
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i64,
}

impl Parameter {
    fn get_value(&self, program: &Program) -> i64 {
        match self.mode {
            ParameterMode::Immediate => self.value,
            ParameterMode::Position => program.memory[self.value as usize],
            ParameterMode::Relative => program.memory[(self.value + program.relative_base) as usize],
        }
    }

    fn get_value_write(&self, program: &Program) -> usize {
        match self.mode {
            ParameterMode::Immediate => self.value as usize,
            ParameterMode::Position => self.value as usize,
            ParameterMode::Relative => (self.value + program.relative_base) as usize,
        }
    }
}

fn decode_instruction(memory: &[i64], i_ptr: usize) -> Instruction {
    let opcode = memory[i_ptr];
    let opcode_bytes: Vec<u8> = opcode.to_string().as_bytes().iter().map(|b| b - 48).collect();
    let p_ptr: i64 = opcode_bytes.len() as i64 - 3;
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
        [0, 9] => {
            // Change relative base, 1 parameter.
            let mut params = get_params(&memory, i_ptr, 1, p_ptr, &opcode_bytes);
            ChangeBase(params.next().unwrap())
        },
        [9, 9] => Halt,
        _ => panic!("Unknown opcode!"),
    }
}

fn get_params<'mem>(
    memory: &'mem [i64],
    i_ptr: usize,
    n: usize,
    p_ptr: i64,
    opcode_bytes: &'mem [u8]
) -> impl Iterator<Item=Parameter> + 'mem {
    (0..n).into_iter().map(move |i| {
        let p_offset = p_ptr - i as i64;
        let mode = if p_offset < 0 { ParameterMode::Position } else { opcode_bytes[p_offset as usize].into() };
        Parameter { mode, value: memory[1 + i_ptr + i as usize] }
    })
}

#[derive(Debug)]
struct Program {
    memory: Vec<i64>,
    halted: bool,
    i_ptr: usize,
    relative_base: i64,
}

impl Program {

    fn init(memory: Vec<i64>) -> Self {
        Self {
            memory,
            halted: false,
            i_ptr: 0,
            relative_base: 0,
        }
    }

    fn execute(&mut self, input: &[i64]) -> Vec<i64> {
        let mut input_pos = 0;
        let mut output = vec![];

        loop {
            let instruction = decode_instruction(&self.memory, self.i_ptr);
            use Instruction::*;
            match instruction {
                Halt => {
                    self.halted = true;
                    break output;
                },
                Add(p1, p2, p3) => {
                    let store = p3.get_value_write(&self);
                    let val = p1.get_value(&self) + p2.get_value(&self);
                    self.memory[store] = val;

                    self.i_ptr += 4;
                },
                Mul(p1, p2, p3) => {
                    let store = p3.get_value_write(&self);
                    let val = p1.get_value(&self) * p2.get_value(&self);
                    self.memory[store] = val;

                    self.i_ptr += 4;
                },
                In(p1) => {
                    let store = p1.get_value_write(&self);
                    if input_pos >= input.len() {
                        break output
                    }
                    self.memory[store] = input[input_pos];

                    input_pos += 1;
                    self.i_ptr += 2;
                },
                Out(p1) => {
                    let val = p1.get_value(&self);
                    output.push(val);

                    self.i_ptr += 2;
                },
                Jnz(p1, p2) => {
                    let val = p1.get_value(&self);
                    if val != 0 {
                        self.i_ptr = p2.get_value(&self) as usize;
                    } else {
                        self.i_ptr += 3;
                    }
                },
                Jz(p1, p2) => {
                    let val = p1.get_value(&self);
                    if val == 0 {
                        self.i_ptr = p2.get_value(&self) as usize;
                    } else {
                        self.i_ptr += 3;
                    }
                },
                Lt(p1, p2, p3) => {
                    let store = p3.get_value_write(&self);
                    if p1.get_value(&self) < p2.get_value(&self) {
                        self.memory[store] = 1;
                    } else {
                        self.memory[store] = 0;
                    }

                    self.i_ptr += 4;
                },
                Eq(p1, p2, p3) => {
                    let store = p3.get_value_write(&self);
                    if p1.get_value(&self) == p2.get_value(&self) {
                        self.memory[store] = 1;
                    } else {
                        self.memory[store] = 0;
                    }

                    self.i_ptr += 4;
                },
                ChangeBase(p1) => {
                    self.relative_base += p1.get_value(&self);

                    self.i_ptr += 2;
                }
            }
        }
    }
}
