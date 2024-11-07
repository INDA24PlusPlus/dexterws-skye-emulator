
use dexterws_skye_emulator::{cpu::Chip8, debugger::{DebugLocations, Debugger}, display::{Display, WIDTH}};

const CLOCK_CYCLE: u64 = 500;
const SLEEP_TIME: u64 = 1000 / CLOCK_CYCLE;

fn main() {
    let file = std::env::args().nth(1).expect("No file provided");
    let parsed = dexterws_skye_emulator::parser::parse_file(&file);
    let mut cpu = Chip8::new(parsed);
    let debug_locations = DebugLocations {
        reg_locations: (WIDTH + 2, 1),
        large_reg_location: (WIDTH + 2, 18),
        stack_location: (WIDTH + 20, 1),
        clock_location: (WIDTH + 40, 1),
    };
    let debugger = Debugger::new(debug_locations);
    // Clear screen from clutter
    print!("\x1B[2J");
    // Hide cursor
    print!("\x1B[?25l");
    loop {
        let res = if let Some(res) = cpu.cycle() {
            res
        } else {
            break;
        };
        if let Some(vram) = res.0 {
            Display::draw(&vram);
        }
        let registers = cpu.dump_registers();
        let large_reg = cpu.dump_large_register();
        let stack = cpu.dump_stack();
        debugger.print_registers(&registers, large_reg);
        debugger.print_stack(&stack);
        debugger.print_clock(cpu.dump_clock());
        std::thread::sleep(std::time::Duration::from_millis(SLEEP_TIME));
    }
}
