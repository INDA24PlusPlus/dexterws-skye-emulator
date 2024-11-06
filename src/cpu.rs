use crate::{fastrand::Rand, opcodes::{Data, OpCode, OpCodeType}};

pub struct Stack {
    head: u8,
    data: [u16; 48],
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            head: 0,
            data: [0; 48],
        }
    }
}

impl Stack {
    fn pop(&mut self) -> Option<u16> {
        if self.head == 0 {
            return None;
        }
        self.head -= 1;
        let data = self.data[self.head as usize];
        return Some(data);
    }

    fn push(&mut self, data: u16) -> Option<()> {
        if self.head == 48 {
            return None;
        }
        self.data[self.head as usize] = data;
        self.head += 1;
        return Some(());
    }
}

pub struct Chip8 {
    pc: u16,
    registers_8bit: [u8; 16],
    register_12bit: u16,
    memory: [u8; 4096],
    stack: Stack,
    rand_engine: Rand,
    program: Vec<OpCode>,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            pc: 0,
            registers_8bit: [0; 16],
            register_12bit: 0,
            memory: [0; 4096],
            stack: Default::default(),
            program: Vec::new(),
            rand_engine: Default::default(),
        }
    }
}

impl Chip8 {
    pub fn new(program: Vec<OpCode>) -> Self {
        Self {
            program,
            ..Default::default()
        }
    }

    fn inc_pc(&mut self) {
        self.pc += 1;
    }

    fn get_opcode(&mut self) -> OpCode {
        *self.program.get(self.pc as usize).unwrap()
    }

    fn execute(&mut self) {
        let oc = self.get_opcode();
        let data = oc.get_data();
        self.inc_pc();
        match oc.oc_type {
            OpCodeType::CALL(_) => unimplemented!(),
            // Clear the display
            OpCodeType::DISPLAY(0) => {
                print!("{}[2J", 27 as char);
            }
            // Return from subroutine
            OpCodeType::FLOW(0) => {
                let pc = self.stack.pop().unwrap();
                self.pc = pc;
            }
            // Jump to addr
            OpCodeType::FLOW(1) => {
                if let Data::NNN(addr) = data {
                    self.pc = addr;
                }
            }
            // Jump to subroutine
            OpCodeType::FLOW(2) => {
                self.stack.push(self.pc).unwrap();
                if let Data::NNN(addr) = data {
                    self.pc = addr;
                }
            }
            // Jump to V0 + addr
            OpCodeType::FLOW(4) => {
                if let Data::NNN(addr) = data {
                    let v0 = self.registers_8bit[0] as u16;
                    self.pc = v0 + addr;
                }
            }
            OpCodeType::FLOW(_) => todo!(),
            // VX == NN
            OpCodeType::COND(0) => {
                if let Data::XNN(x, nn) = data {
                    if self.registers_8bit[x as usize] == nn {
                        self.inc_pc();
                    }
                }
            }
            // VX != NN
            OpCodeType::COND(1) => {
                if let Data::XNN(x, nn) = data {
                    if self.registers_8bit[x as usize] != nn {
                        self.inc_pc();
                    }
                }
            }
            // VX == VY
            OpCodeType::COND(2) => {
                if let Data::XY(x, y) = data {
                    if self.registers_8bit[x as usize] == self.registers_8bit[y as usize] {
                        self.inc_pc();
                    }
                }
            }
            // VX != VY
            OpCodeType::COND(4) => {
                if let Data::XY(x, y) = data {
                    if self.registers_8bit[x as usize] != self.registers_8bit[y as usize] {
                        self.inc_pc();
                    }
                }
            }
            OpCodeType::COND(_) => todo!(),
            // VX = NN
            OpCodeType::CONST(0) => {
                if let Data::XNN(x, nn) = data {
                    self.registers_8bit[x as usize] = nn;
                }
            }
            // VX += NN
            OpCodeType::CONST(1) => {
                if let Data::XNN(x, nn) = data {
                    self.registers_8bit[x as usize] += nn;
                }
            }
            OpCodeType::CONST(_) => todo!(),
            // VX = VY
            OpCodeType::ASSIG(_) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] = self.registers_8bit[y as usize];
                }
            }
            // VX = VX | VY
            OpCodeType::BITOP(0) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] |= self.registers_8bit[y as usize];
                }
            }
            // VX = VX & VY
            OpCodeType::BITOP(1) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] &= self.registers_8bit[y as usize];
                }
            }
            // VX = VX ^ VY
            OpCodeType::BITOP(2) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] ^= self.registers_8bit[y as usize];
                }
            }
            // VX >>= VY
            OpCodeType::BITOP(4) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] >>= self.registers_8bit[y as usize];
                }
            }
            // VX <<= VY
            OpCodeType::BITOP(8) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] <<= self.registers_8bit[y as usize];
                }
            }
            OpCodeType::BITOP(_) => todo!(),
            // VX += VY
            OpCodeType::MATH(0) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] += self.registers_8bit[y as usize];
                }
            }
            // VX -= VY
            OpCodeType::MATH(1) => {
                if let Data::XY(x, y) = data {
                    self.registers_8bit[x as usize] -= self.registers_8bit[y as usize];
                }
            }
            // VX = VY - VX
            OpCodeType::MATH(2) => {
                if let Data::XY(x, y) = data {
                    let vx = self.registers_8bit[x as usize];
                    let vy = self.registers_8bit[y as usize];
                    self.registers_8bit[x as usize] = vy - vx;
                }
            }
            OpCodeType::MATH(_) => todo!(),
            // I = NNN
            OpCodeType::MEM(0) => {
                if let Data::NNN(addr) = data {
                    self.register_12bit = addr;
                }
            }
            // I += VX
            OpCodeType::MEM(1) => {
                if let Data::X(x) = data {
                    self.register_12bit += self.registers_8bit[x as usize] as u16;
                }
            }
            // I = sprite[VX]
            OpCodeType::MEM(2) => {
                todo!()
            }
            // REGDUMP
            OpCodeType::MEM(4) => {
                if let Data::X(x) = data {
                    let reg_i = self.register_12bit as usize;
                    for i in 0..=x as usize {
                        self.memory[reg_i + i] = self.registers_8bit[i];
                    }
                }
            }
            // REGLOAD
            OpCodeType::MEM(8) => {
                if let Data::X(x) = data {
                    let reg_i = self.register_12bit as usize;
                    for i in 0..=x as usize {
                        self.registers_8bit[i] = self.memory[reg_i + i];
                    }
                }
            }
            OpCodeType::MEM(_) => todo!(),
            OpCodeType::RAND(_) => {
                if let Data::XNN(x, nn) = data {
                    let rand = self.rand_engine.rand() as u8;
                    self.registers_8bit[x as usize] = rand & nn;
                }
            }
            OpCodeType::KEYOP(_) => todo!(),
            OpCodeType::TIMER(_) => todo!(),
            OpCodeType::SOUND(_) => todo!(),
            OpCodeType::BCD(_) => todo!(),
            _ => todo!(),
        }
    }
}

