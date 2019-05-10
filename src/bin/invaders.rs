use emu8080::Cpu;
use emu8080::EventSignal;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::fs::File;
use std::io::prelude::*;
use std::mem;

struct Cabinet {
    p0: u8,
    p1: u8,
    p2: u8,
    p3: u8,
    p4: u8,
    p5: u8,
    p6: u8,
    p7: u8,
    shift: u16,
    offset: u8,
}

// move game stuff inside of the impl Cabinet block. The game runs as part of the full cabinet implementation.

impl Cabinet {
    fn new() -> Self {
        Cabinet {
            p0: 0xE,
            p1: 0x8,
            p2: 0,
            p3: 0,
            p4: 0,
            p5: 0,
            p6: 0,
            p7: 0,
            shift: 0,
            offset: 0,
        }
    }

    fn input(&mut self, port: u8) -> u8 {
        match port {
            0 => self.p0,
            1 => self.p1,
            2 => self.p2,
            3 => ((self.shift >> (8 - self.offset)) & 0xFF) as u8,
            _ => panic!("Invalid port selection"),
        }
    }

    fn output(&mut self, port: u8, value: u8) {
        match port {
            2 => self.offset = value & 0x7,
            3 => {
                // sound stuff
            }
            4 => self.shift = (self.shift >> 8) | ((value as u16) << 8),
            _ => panic!("Invalid port selection"),
        }
    }

    fn key_down(&mut self, key: Keycode) {
        match key {
            Keycode::A => self.p1 |= 0x20,
            Keycode::Kp4 => self.p2 |= 0x20,
            Keycode::D => self.p1 |= 0x40,
            Keycode::Kp6 => self.p2 |= 0x40,
            Keycode::Space => self.p1 |= 0x10,
            Keycode::Kp0 => self.p2 |= 0x10,
            _ => {
                // do nothing
            }
        }
    }

    fn key_up(&mut self, key: Keycode) {
        match key {
            Keycode::A => self.p1 ^= 0x20,
            Keycode::Kp4 => self.p2 ^= 0x20,
            Keycode::D => self.p1 ^= 0x40,
            Keycode::Kp6 => self.p2 ^= 0x40,
            Keycode::Space => self.p1 ^= 0x10,
            Keycode::Kp0 => self.p2 ^= 0x10,
            _ => {
                // do nothing
            }
        }
    }
}

fn main() {
    env_logger::init();
    let sdl = sdl2::init().expect("Sdl failed to init. Big mistake.");

    let mut cabinet = Cabinet::new();
    let mut cpu = Cpu::new();
    read_space_invaders_into_memory(&mut cpu);

    let mut event_pump = sdl
        .event_pump()
        .expect("Event pump failed. Possibly another is running?");

    let audio = sdl.audio().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("Insert Rom Name", 224 * 2, 256 * 2)
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("window failed to init to canvas");

    loop {
        handle_events(&cpu, &mut cabinet, &mut event_pump);

        let op = cpu.get_current_opcode();
        match op.code {
            0xD3 => {
                let (byte, register) = cpu.output();
                cabinet.output(byte, register.into());
            }
            0xDB => {
                let input = cabinet.input(cpu.get_next_byte());
                cpu.input(input);
            }
            _ => cpu.execute_opcode(),
        }
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

fn handle_events(cpu: &Cpu, cabinet: &mut Cabinet, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => ::std::process::exit(0),
            Event::KeyDown {
                keycode: Some(key), ..
            } => cabinet.key_down(key),
            Event::KeyUp {
                keycode: Some(key), ..
            } => cabinet.key_up(key),
            _ => {
                // do nothing
            }
        }
    }
}
