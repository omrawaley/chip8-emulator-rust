extern crate sdl2;

use std::time::Duration;
use sdl2::{event::Event, keyboard::Keycode, pixels::{Color, PixelFormat, PixelFormatEnum}};
use chip8::{cpu::CPU, display::Display, keypad::{Key, KeyState, Keypad}, memory::Memory};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No program provided");
        std::process::exit(1);
    }

    let mut memory = Memory::new();
    let mut cpu = CPU::new();
    let mut display = Display::new();
    let mut keypad = Keypad::new();

    let bytes= std::fs::read(args[1].as_str()).unwrap();
    memory.load_program(&bytes);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 Emulator Rust", 64 * 16, 32 * 16)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(255, 255, 255));
    // canvas.clear();
    // canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 64, 32).unwrap();

    let mut framebuffer = [0u8; (64 * 32 * 3) as usize];
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        let pixels = display.get_data();

        for i in 0..pixels.len() {
            let pixel_index = i * 3;
            let color = if pixels[i] {
                [255, 255, 255]
            } else {
                [0, 0, 0]
            };
            framebuffer[pixel_index] = color[0];
            framebuffer[pixel_index + 1] = color[1];
            framebuffer[pixel_index + 2] = color[2];
        }

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..32 {
                for x in 0..64 {
                    let index = (y * 64 + x) as usize;
                    let buffer_offset = y as usize * pitch + x as usize * 3;
                    buffer[buffer_offset..buffer_offset + 3].copy_from_slice(&framebuffer[index * 3..index * 3 + 3]);
                }
            }
        }).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let keyboard_state = event_pump.keyboard_state();

        // First row: 1 2 3 C
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Num1) {
            keypad.set_key(Key::One(KeyState::Pressed));
        } else {
            keypad.set_key(Key::One(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Num2) {
            keypad.set_key(Key::Two(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Two(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Num3) {
            keypad.set_key(Key::Three(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Three(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Num4) {
            keypad.set_key(Key::C(KeyState::Pressed));
        } else {
            keypad.set_key(Key::C(KeyState::Released));
        }
        
        // Second row: 4 5 6 D
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Q) {
            keypad.set_key(Key::Four(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Four(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            keypad.set_key(Key::Five(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Five(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::E) {
            keypad.set_key(Key::Six(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Six(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::R) {
            keypad.set_key(Key::D(KeyState::Pressed));
        } else {
            keypad.set_key(Key::D(KeyState::Released));
        }
        
        // Third row: 7 8 9 E
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            keypad.set_key(Key::Seven(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Seven(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            keypad.set_key(Key::Eight(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Eight(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            keypad.set_key(Key::Nine(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Nine(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::F) {
            keypad.set_key(Key::E(KeyState::Pressed));
        } else {
            keypad.set_key(Key::E(KeyState::Released));
        }
        
        // Fourth row: A 0 B F
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Z) {
            keypad.set_key(Key::A(KeyState::Pressed));
        } else {
            keypad.set_key(Key::A(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::X) {
            keypad.set_key(Key::Zero(KeyState::Pressed));
        } else {
            keypad.set_key(Key::Zero(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::C) {
            keypad.set_key(Key::B(KeyState::Pressed));
        } else {
            keypad.set_key(Key::B(KeyState::Released));
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::V) {
            keypad.set_key(Key::F(KeyState::Pressed));
        } else {
            keypad.set_key(Key::F(KeyState::Released));
        }

        cpu.tick(&mut memory, &mut display, &keypad);

        canvas.copy(&texture, None, sdl2::rect::Rect::new(0, 0, 64 * 16, 32 * 16)).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}