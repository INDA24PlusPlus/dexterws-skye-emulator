use crate::{fastrand::Rand, parser::{DataType, OpCode, OpCodeIdentity, OpCodeType}};

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

struct Display {
    data: [u8; 64 * 32],
}

impl Default for Display {
    fn default() -> Self {
        Self {
            data: [0; 64 * 32],
        }
    }
}

impl Display {
    fn clear(&mut self) {
        self.data = [0; 64 * 32];
    }

    fn draw(&self) {
        print!("\x1B[1;1H");
        for y in 0..32 {
            for x in 0..64 {
                print!("{}", if self.data[x + y * 64] == 1 { "#" } else { " " });
            }
            println!();
        }
    }
}

pub struct Chip8 {
    pc: u16,
    registers_8bit: [u8; 16],
    register_12bit: u16,
    memory: [u8; 4096],
    stack: Stack,
    rand_engine: Rand,
    display: Display,
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
            display: Default::default(),
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

    fn execute_op(&mut self) {
        let oc = self.get_opcode();
        let data = oc.get_data();
        self.inc_pc();
        match oc.oc_id {
            OpCodeIdentity::CallMach => unimplemented!(),
            OpCodeIdentity::ClrDisp => {
                self.display.clear();
            }
            OpCodeIdentity::RetSub => {
                self.pc = self.stack.pop().unwrap();
            }
            OpCodeIdentity::JumpAddr => {
                if let DataType::NNN { address } = data {
                    self.pc = address;
                }
            }
            OpCodeIdentity::CallSub => {
                if let DataType::NNN { address } = data {
                    self.stack.push(self.pc);
                    self.pc = address;
                }
            }
            OpCodeIdentity::SkipEqRC => {
                if let DataType::XNN { x, constant } = data {
                    if self.registers_8bit[x as usize] == constant {
                        self.inc_pc();
                    }
                }
            }
            OpCodeIdentity::SkipNqRC => {
                if let DataType::XNN { x, constant } = data {
                    if self.registers_8bit[x as usize] != constant {
                        self.inc_pc();
                    }
                }
            }
            OpCodeIdentity::SkipEqRR => {
                if let DataType::XY { x, y } = data {
                    if self.registers_8bit[x as usize] == self.registers_8bit[y as usize] {
                        self.inc_pc();
                    }
                }
            }
            OpCodeIdentity::SetRC => {
                if let DataType::XNN { x, constant } = data {
                    self.registers_8bit[x as usize] = constant;
                }
            }
            OpCodeIdentity::AddNcRC => {
                if let DataType::XNN { x, constant } = data {
                    self.registers_8bit[x as usize] += constant;
                }
            }
            OpCodeIdentity::SetRR => {
                if let DataType::XY { x, y } = data {
                    self.registers_8bit[x as usize] = self.registers_8bit[y as usize];
                }
            }
            OpCodeIdentity::OrRR => {
                if let DataType::XY { x, y } = data {
                    self.registers_8bit[x as usize] |= self.registers_8bit[y as usize];
                }
            }
            OpCodeIdentity::AndRR => {
                if let DataType::XY { x, y } = data {
                    self.registers_8bit[x as usize] &= self.registers_8bit[y as usize];
                }
            }
            OpCodeIdentity::XorRR => {
                if let DataType::XY { x, y } = data {
                    self.registers_8bit[x as usize] ^= self.registers_8bit[y as usize];
                }
            }
            OpCodeIdentity::AddRR => {
                if let DataType::XY { x, y } = data {
                    let (result, overflow) = self.registers_8bit[x as usize].overflowing_add(self.registers_8bit[y as usize]);
                    self.registers_8bit[x as usize] = result;
                    self.registers_8bit[0xF] = overflow as u8;
                }
            }
            OpCodeIdentity::SubRRR => {
                if let DataType::XY { x, y } = data {
                    let (result, overflow) = self.registers_8bit[x as usize].overflowing_sub(self.registers_8bit[y as usize]);
                    self.registers_8bit[x as usize] = result;
                    self.registers_8bit[0xF] = !overflow as u8;
                }
            }
            OpCodeIdentity::RshiftR => {
                if let DataType::X { x } = data {
                    self.registers_8bit[0xF] = self.registers_8bit[x as usize] & 0x1;
                    self.registers_8bit[x as usize] >>= 1;
                }
            }
            OpCodeIdentity::SubLRR => {
                if let DataType::XY { x, y } = data {
                    let (result, overflow) = self.registers_8bit[y as usize].overflowing_sub(self.registers_8bit[x as usize]);
                    self.registers_8bit[x as usize] = result;
                    self.registers_8bit[0xF] = !overflow as u8;
                }
            }
            OpCodeIdentity::LshiftR => {
                if let DataType::X { x } = data {
                    self.registers_8bit[0xF] = self.registers_8bit[x as usize] >> 7;
                    self.registers_8bit[x as usize] <<= 1;
                }
            }
            OpCodeIdentity::SkipNqRR => {
                if let DataType::XY { x, y } = data {
                    if self.registers_8bit[x as usize] != self.registers_8bit[y as usize] {
                        self.inc_pc();
                    }
                }
            }
            OpCodeIdentity::SetAddrRegC => {
                if let DataType::NNN { address } = data {
                    self.register_12bit = address;
                }
            }
            OpCodeIdentity::JumpAddrCR => {
                if let DataType::NNN { address } = data {
                    self.pc = self.registers_8bit[0] as u16 + address;
                }
            }
            OpCodeIdentity::RandRC => {
                if let DataType::XNN { x, constant } = data {
                    self.registers_8bit[x as usize] = self.rand_engine.rand() as u8 & constant;
                }
            }
            OpCodeIdentity::DrawDispRRC => {
                if let DataType::XYN { x, y, constant: height } = data {
                    let x = self.registers_8bit[x as usize] as usize;
                    let y = self.registers_8bit[y as usize] as usize;
                    let height = height as usize;
                    let mut collision = false;
                    for yline in 0..height {
                        let pixel = self.memory[(self.register_12bit + yline as u16) as usize];
                        for xline in 0..8 {
                            if (pixel & (0x80 >> xline)) != 0 {
                                let x = (x + xline) % 64;
                                let y = (y + yline) % 32;
                                if self.display.data[x + y * 64] == 1 {
                                    collision = true;
                                }
                                self.display.data[x + y * 64] ^= 1;
                            }
                        }
                    }
                    self.registers_8bit[0xF] = collision as u8;
                }
            }
            OpCodeIdentity::SkipKeyPressedR => todo!(),
            OpCodeIdentity::SkipNKeyPressedR => todo!(),
            OpCodeIdentity::GetDelayR => todo!(),
            OpCodeIdentity::AwaitGetKeyDownR => todo!(),
            OpCodeIdentity::SetDelayR => todo!(),
            OpCodeIdentity::SetSoundR => todo!(),
            OpCodeIdentity::AddAddrRegR => todo!(),
            OpCodeIdentity::SetAddrRegSpriteR => todo!(),
            OpCodeIdentity::SetBcdR => {
                if let DataType::X { x } = data {
                    let x = self.registers_8bit[x as usize];
                    self.memory[self.register_12bit as usize] = x / 100;
                    self.memory[(self.register_12bit + 1) as usize] = (x / 10) % 10;
                    self.memory[(self.register_12bit + 2) as usize] = x % 10;
                }
            }
            OpCodeIdentity::DumpRegsToMemR => {
                if let DataType::X { x } = data {
                    for i in 0..=x {
                        self.memory[(self.register_12bit + i as u16) as usize] = self.registers_8bit[i as usize];
                    }
                }
            }
            OpCodeIdentity::LoadRegsFromMemR => {
                if let DataType::X { x } = data {
                    for i in 0..=x {
                        self.registers_8bit[i as usize] = self.memory[(self.register_12bit + i as u16) as usize];
                    }
                }
            }
        }
    }
    pub fn run(&mut self) {
        loop {
            self.execute_op();
            self.display.draw();
        }
    }
}

