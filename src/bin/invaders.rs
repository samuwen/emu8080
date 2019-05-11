use emu8080::Cpu;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::time::Instant;

const HEIGHT: u32 = 224;
const WIDTH: u32 = 256;

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
            // p0: 0xE,
            p0: 0,
            p1: 1,
            // p1: 0x8,
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
            _ => {
                // do nothing
            }
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
            Keycode::A => self.p1 &= 0xDF,
            Keycode::Kp4 => self.p2 &= 0xDF,
            Keycode::D => self.p1 &= 0xBF,
            Keycode::Kp6 => self.p2 &= 0xBF,
            Keycode::Space => self.p1 &= 0xEF,
            Keycode::Kp0 => self.p2 &= 0xEF,
            _ => {
                // do nothing
            }
        }
    }
}

fn main() {
    const NANOS_PER_SECOND: u64 = 1_000_000_000;
    const CPU_SPEED: u64 = 2_000_000;
    const NANOS_PER_CYCLE: u64 = NANOS_PER_SECOND / CPU_SPEED;
    const VIDEO_INTERRUPT_TIMER: Duration = Duration::from_nanos(16667);
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
        .window("Invaders from Space", HEIGHT, WIDTH)
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("window failed to init to canvas");

    let mut last_interrupt = Instant::now();
    let mut last_cycle = Instant::now();
    let mut cycles_elapsed: u64 = 0;

    let mut count = 0;

    'running: loop {
        handle_events(&cpu, &mut cabinet, &mut event_pump);

        // if Instant::now().duration_since(last_interrupt) >= VIDEO_INTERRUPT_TIMER {
        //     if cpu.interrupts_enabled() {
        //         last_interrupt = Instant::now();
        //     }
        // } else {
        // let nanos_elapsed = Duration::from_nanos(cycles_elapsed * NANOS_PER_CYCLE);
        // if Instant::now().duration_since(last_cycle) > nanos_elapsed {
        let op = cpu.get_current_opcode();
        match op.code {
            0xD3 => {
                let (byte, register) = cpu.output();
                cabinet.output(byte, register.into());
                cpu.increment_pc(1);
            }
            0xDB => {
                let input = cabinet.input(cpu.get_next_byte());
                cpu.input(input);
                cpu.increment_pc(1);
            }
            _ => {
                cycles_elapsed += cpu.execute_opcode(&count) as u64;
                last_cycle = Instant::now();
            }
        }
        // }
        // }
        if count > 50000 {
            draw_to_screen(&mut cpu, &mut canvas);
        }
        count += 1;
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
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => ::std::process::exit(0),
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

fn draw_to_screen(cpu: &mut Cpu, canvas: &mut Canvas<Window>) {
    let white = Color::RGB(255, 255, 255);
    let black = Color::RGB(0, 0, 0);
    canvas.clear();
    let display_buffer = cpu.get_video_memory();
    let shift_end = 7u8;
    for (i, byte) in display_buffer.iter().enumerate() {
        let y = (i * 8) / (WIDTH as usize + 1);

        for shift in 0..shift_end {
            let x = ((i * 8) % WIDTH as usize) + shift as usize;

            if (byte >> shift) & 1 == 0 {
                canvas.set_draw_color(black);
            } else {
                canvas.set_draw_color(white);
            }
            let rect = Rect::new(x as i32, y as i32, 10, 10);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present()
}
