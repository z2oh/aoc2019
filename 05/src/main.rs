fn main() {
    let mut memory = include_str!("input")
        .split(',')
        .map(str::trim)
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    let output = execute(&mut memory, &[5]);

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

fn execute(memory: &mut [i32], input: &[i32]) -> Vec<i32> {
    let mut i_ptr = 0;
    let mut input_pos = 0;
    let mut output = vec![];

    loop {
        let instruction = decode_instruction(memory, i_ptr);
        use Instruction::*;
        match instruction {
            Halt => break output,
            Add(p1, p2, p3) => {
                let store = p3.value as usize;
                let val = p1.get_value(&memory) + p2.get_value(&memory);
                memory[store] = val;

                i_ptr += 4;
            },
            Mul(p1, p2, p3) => {
                let store = p3.value as usize;
                let val = p1.get_value(&memory) * p2.get_value(&memory);
                memory[store] = val;

                i_ptr += 4;
            },
            In(p1) => {
                let store = p1.value as usize;
                memory[store] = input[input_pos];

                input_pos += 1;
                i_ptr += 2;
            },
            Out(p1) => {
                let val = p1.get_value(&memory);
                output.push(val);

                i_ptr += 2;
            },
            Jnz(p1, p2) => {
                let val = p1.get_value(&memory);
                if val != 0 {
                    i_ptr = p2.get_value(&memory) as usize;
                } else {
                    i_ptr += 3;
                }
            },
            Jz(p1, p2) => {
                let val = p1.get_value(&memory);
                if val == 0 {
                    i_ptr = p2.get_value(&memory) as usize;
                } else {
                    i_ptr += 3;
                }
            },
            Lt(p1, p2, p3) => {
                if p1.get_value(&memory) < p2.get_value(&memory) {
                    memory[p3.value as usize] = 1;
                } else {
                    memory[p3.value as usize] = 0;
                }

                i_ptr += 4;
            },
            Eq(p1, p2, p3) => {
                if p1.get_value(&memory) == p2.get_value(&memory) {
                    memory[p3.value as usize] = 1;
                } else {
                    memory[p3.value as usize] = 0;
                }

                i_ptr += 4;
            },
        }
    }
}
