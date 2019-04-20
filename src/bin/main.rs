use emu8080::Cpu;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    env_logger::init();

    let mut cpu = Cpu::new();
    read_space_invaders_into_memory(&mut cpu);
    for _ in 0x0..0x15 {
        cpu.execute_opcode();
    }
}

fn read_space_invaders_into_memory(cpu: &mut Cpu) {
    let path1 = String::from("src/roms/invaders.h");
    let path2 = String::from("src/roms/invaders.g");
    let path3 = String::from("src/roms/invaders.f");
    let path4 = String::from("src/roms/invaders.e");
    let path_list = vec![path1, path2, path3, path4];
    for (i, path) in path_list.iter().enumerate() {
        let mut file = File::open(&path).expect("File not found");
        let mut buffer = [0u8; 0x7FF];
        file.read(&mut buffer).expect("Error reading file.");
        cpu.load_rom_into_memory(0x7FF * i + i, &buffer);
    }
}
