use crate::{cpu::Stack, parser::OpCode};


pub struct Debugger {
    locations: DebugLocations,
}

pub struct DebugLocations {
    pub reg_locations: (usize, usize),
    pub code_locations: (usize, usize),
    pub large_reg_location: (usize, usize),
    pub stack_location: (usize, usize),
    pub clock_location: (usize, usize),
}

impl Debugger {
    pub fn new(locations: DebugLocations) -> Self {
        Self {
            locations
        }
    }

    pub fn print_registers(&self, registers: &[u8; 16], large_reg: u16) {
        let reg_loc = self.locations.reg_locations;
        let large_reg_loc = self.locations.large_reg_location;
        // Goto
        print!("\x1B[{};{}H", reg_loc.1, reg_loc.0);
        print!("8bit registers:");
        for i in 0..16 {
            print!("\x1B[{};{}H", reg_loc.1 + i + 1, reg_loc.0);
            print!("V{:X} = {:#04X} ", i, registers[i as usize]);
        }
        print!("\x1B[{};{}H", large_reg_loc.1, large_reg_loc.0);
        print!("12bit register:");
        print!("\x1B[{};{}H", large_reg_loc.1 + 1, large_reg_loc.0);
        println!("I = {:#05X}", large_reg);
    }

    pub fn print_stack(&self, stack: &Stack) {
        let stack_loc = self.locations.stack_location;
        print!("\x1B[{};{}H", stack_loc.1, stack_loc.0);
        print!("Stack:");
        for i in 0..stack.data.len() {
            print!("\x1B[{};{}H", stack_loc.1 + i + 1, stack_loc.0);
            if i >= stack.head as usize {
                print!("        ");
                continue;
            }
            print!("{:#05X} ", stack.data[i as usize]);
        }
    }
    
    pub fn print_codes(&self, program:&[OpCode], pc:u16){
        let prog_loc = self.locations.code_locations;
        let list_len=if 30>program.len()as u16 {program.len() as u16} else {30};
        let disp_start = (pc/list_len)*list_len;
        print!("\x1b[{};{}H Program:", prog_loc.1, prog_loc.0);
        for n in 0..list_len{
            print!("\x1b[{};{}H {:#02X}:                  ",prog_loc.1+1+n as usize,prog_loc.0,pc);
            if disp_start+n<program.len() as u16{
                let mut point=" ";
                if disp_start+n==pc{
                    point=">";
                }
                print!("\x1b[{};{}H{}{:#02X}:{:#04X}",prog_loc.1+1+n as usize,prog_loc.0,point,pc,program[(disp_start+n) as usize].op_code);
            }

        }
        
    }

    pub fn print_clock(&self, clocks: (u8, u8)) {
        let clock_loc = self.locations.clock_location;
        print!("\x1B[{};{}H", clock_loc.1, clock_loc.0);
        print!("Clocks:");
        print!("\x1B[{};{}H", clock_loc.1 + 1, clock_loc.0);
        println!("Delay: {:#04X}", clocks.0);
        print!("\x1B[{};{}H", clock_loc.1 + 2, clock_loc.0);
        println!("Sound: {:#04X}", clocks.1);
    }
}
