use emu8080::Cpu;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    env_logger::init();

    let mut cpu = Cpu::new();
    read_space_invaders_into_memory(&mut cpu);
    let limit_base = 49000;
    let factor = 1;
    let limit = limit_base * factor - (6 * (factor - 1));
    for i in 0..limit {
        cpu.execute_opcode(i);
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

/*
[2019-04-30T02:14:08Z DEBUG emu8080::cpu] (39) (0) (0) (1c) (0) (21) (0)
[2019-04-30T02:14:08Z DEBUG emu8080::cpu] Opcode  CALL : cd |  1e6
*/
