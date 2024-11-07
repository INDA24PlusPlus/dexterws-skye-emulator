use dexterws_skye_emulator::cpu::Chip8;

fn main() {
    let input = "00E0";
    let parsed = dexterws_skye_emulator::parser::parse_text(input.to_owned());
    Chip8::new(parsed).run();
}
