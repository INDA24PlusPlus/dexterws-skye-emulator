use crate::{display::{HEIGHT, WIDTH}, fastrand::Rand, parser::{DataType, OpCode, OpCodeIdentity, OpCodeType}};

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    pub(crate) head: u8,
    pub(crate) data: [u16; 48],
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

struct VRAM {
    data: [u8; 64 * 32],
}

impl Default for VRAM {
    fn default() -> Self {
        Self {
            data: [0; WIDTH as usize * HEIGHT as usize],
        }
    }
}

impl VRAM {
    fn clear(&mut self) {
        self.data = [0; 64 * 32];
    }

    fn flip(&mut self, x: usize, y: usize) -> bool {
        if x >= 64 || y >= 32 {
            return false;
        }
        self.data[x + y * 64] ^= 1;
        return self.data[x + y * 64] == 0;
    }
}


pub struct Chip8 {
    pc: u16,
    registers_8bit: [u8; 16],
    register_12bit: u16,
    memory: [u8; 4096],
    stack: Stack,
    rand_engine: Rand,
    vram: VRAM,
    vram_changed: bool,
    program: Vec<OpCode>,
    timer: u8,
    sound_timer: u8,
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
            vram: VRAM::default(),
            vram_changed: false,
            timer: 0,
            sound_timer: 0,
        }
    }
}

pub enum CPUError {
    StackOverflow,
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

    fn get_opcode(&mut self) -> Option<OpCode> {
        self.program.get(self.pc as usize).map(|x| *x)
    }

    fn execute_op(&mut self, oc: OpCode) -> Result<(), CPUError> {
        let data = oc.get_data();
        self.inc_pc();
        match oc.oc_id {
            OpCodeIdentity::CallMach => unimplemented!(),
            OpCodeIdentity::ClrDisp => {
                self.vram.clear();
                self.vram_changed = true;
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
                    if let Some(()) = self.stack.push(self.pc) {
                        self.pc = address;
                    } else {
                        return Err(CPUError::StackOverflow);
                    }
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
                                collision = self.vram.flip(x + xline, y + yline);
                            }
                        }
                    }
                    self.registers_8bit[0xF] = collision as u8;
                }
                self.vram_changed = true;
            }
            OpCodeIdentity::SkipKeyPressedR => unimplemented!(),
            OpCodeIdentity::SkipNKeyPressedR => unimplemented!(),
            OpCodeIdentity::GetDelayR => {
                if let DataType::X { x } = data {
                    self.registers_8bit[x as usize] = self.timer;
                }
            }
            OpCodeIdentity::AwaitGetKeyDownR => unimplemented!(),
            OpCodeIdentity::SetDelayR => {
                if let DataType::X { x } = data {
                    self.timer = self.registers_8bit[x as usize];
                }
            }
            OpCodeIdentity::SetSoundR => {
                if let DataType::X { x } = data {
                    self.sound_timer = self.registers_8bit[x as usize];
                }
            }
            OpCodeIdentity::AddAddrRegR => {
                if let DataType::X { x } = data {
                    self.register_12bit += self.registers_8bit[x as usize] as u16;
                }
            }
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
        Ok(())
    }

    pub fn dump_registers(&self) -> [u8; 16] {
        self.registers_8bit
    }

    pub fn dump_large_register(&self) -> u16 {
        self.register_12bit
    }

    pub fn dump_stack(&self) -> Stack {
        self.stack
    }

    pub fn dump_clock(&self) -> (u8, u8) {
        (self.timer, self.sound_timer)
    }

    pub fn cycle(&mut self) -> Option<(Option<&[u8]>, bool)> {
        self.vram_changed = false;
        let oc = self.get_opcode()?;
        let res = self.execute_op(oc);
        match res {
            Ok(_) => (),
            Err(_) => panic!("CPU ERROR"),
        }
        self.timer = self.timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
        if self.vram_changed {
            return Some((Some(&self.vram.data), self.sound_timer > 0));
        }
        Some((None, self.sound_timer > 0))
    }
}

