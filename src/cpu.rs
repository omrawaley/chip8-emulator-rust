use crate::memory::*;
use crate::display::*;
use crate::keypad::*;

pub struct CPU {
    stack: [u16; 16],
    i: u16,
    sp: u16,
    pc: u16,
    v: [u8; 16],
    dt: u8,
    st: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            stack: [0; 16],
            i: 0,
            sp: 0,
            pc: 0x200,
            v: [0; 16],
            dt: 0,
            st: 0,
        }
    }

    pub fn tick(&mut self, memory: &mut Memory, display: &mut Display, keypad: &Keypad) {
        for i in 0..11 {
            self.execute(memory, display, keypad);
        }

        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    fn execute(&mut self, memory: &mut Memory, display: &mut Display, keypad: &Keypad) {
        let opcode = self.fetch_word(memory);

        let nibbles = [
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0xF00) >> 8) as u8,
            ((opcode & 0xF0) >> 4) as u8,
            (opcode & 0xF) as u8
        ];

        let x = nibbles[1];
        let y = nibbles[2];
        let n = nibbles[3];
        let nn = (opcode & 0xFF) as u8;
        let nnn = (opcode & 0xFFF) as u16;

        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => {
                println!("CLS");
                display.clear();
            },
            [0x0, 0x0, 0xE, 0xE] => {
                println!("RET");
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            [0x1, _, _, _] => {
                println!("JP addr");
                self.pc = nnn;
            },
            [0x2, _, _, _] => {
                println!("CALL addr");
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            [0x3, _, _, _] => {
                println!("SE Vx, byte");
                if self.v[x as usize] == nn {
                    self.pc += 2;
                }
            },
            [0x4, _, _, _] => {
                println!("SNE Vx, byte");
                if self.v[x as usize] != nn {
                    self.pc += 2;
                }
            },
            [0x5, _, _, _] => {
                println!("SE Vx, Vy");
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            },
            [0x6, _, _, _] => {
                println!("LD Vx, byte");
                self.v[x as usize] = nn;
            },
            [0x7, _, _, _] => {
                println!("ADD Vx, byte");
                self.v[x as usize] = self.v[x as usize].wrapping_add(nn);
            },
            [0x8, _, _, 0x0] => {
                println!("LD Vx, Vy");
                self.v[x as usize] = self.v[y as usize];
            },
            [0x8, _, _, 0x1] => {
                println!("OR Vx, Vy");
                self.v[x as usize] |= self.v[y as usize];
            },
            [0x8, _, _, 0x2] => {
                println!("AND Vx, Vy");
                self.v[x as usize] &= self.v[y as usize];
            },
            [0x8, _, _, 0x3] => {
                println!("XOR Vx, Vy");
                self.v[x as usize] ^= self.v[y as usize];
            },
            [0x8, _, _, 0x4] => {
                println!("ADD Vx, Vy");
                let sum = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                self.v[x as usize] = (sum & 0xFF) as u8;
                self.v[0xF] = (sum > 0xFF) as u8;
            },
            [0x8, _, _, 0x5] => {
                println!("SUB Vx, Vy");
                let not_borrow = (self.v[x as usize] as u16) >= (self.v[y as usize] as u16);
                self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
                self.v[0xF] = not_borrow as u8;
            },
            [0x8, _, _, 0x6] => {
                println!("SHR Vx, Vy");
                let lsb = self.v[x as usize] & 0x1;
                self.v[x as usize] >>= 1;
                self.v[0xF] = lsb as u8;
            },
            [0x8, _, _, 0x7] => {
                println!("SUBN Vx, Vy");
                let borrow = (self.v[y as usize] as u16) >= (self.v[x as usize] as u16);
                self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);
                self.v[0xF] = borrow as u8;
            },
            [0x8, _, _, 0xE] => {
                println!("SHL Vx, Vy");
                let msb = (self.v[x as usize] & 0x80) >> 7;
                self.v[x as usize] <<= 1;
                self.v[0xF] = msb;
            },
            [0x9, _, _, _] => {
                println!("SNE Vx, Vy");
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            },
            [0xA, _, _, _] => {
                println!("LD i, addr");
                self.i = nnn;
            },
            [0xB, _, _, _] => {
                println!("JP V0, addr");
                self.pc = nnn + self.v[0] as u16;
            },
            [0xC, _, _, _] => {
                println!("RND Vx, byte");
                self.v[x as usize] = rand::random::<u8>() & nn;
            },
            [0xD, _, _, _] => {
                println!("DRW Vx, Vy, nibble");
                let x_position = self.v[x as usize] % 64;
                let y_position = self.v[y as usize] % 32;

                self.v[0xF] = 0;

                for row in 0..n {
                    if y_position + row >= 32 {
                        break;
                    }

                    let sprite_byte = memory.read_byte(self.i + row as u16);

                    for column in 0..8 {
                        if x_position + column >= 64 {
                            break;
                        }

                        if (sprite_byte & (0x80 >> column)) == 0 {
                            continue;
                        }

                        let pixel_position: u16 = (y_position as u16 + row as u16) * 64 + (x_position as u16 + column as u16);

                        if display.get_pixel(pixel_position) == true {
                            self.v[0xF] = 1;
                        }

                        display.set_pixel(pixel_position, !display.get_pixel(pixel_position));
                    }
                }
            },
            [0xE, _, 0x9, 0xE] => {
                println!("SKP Vx");
                if keypad.is_key_pressed(&keypad.keys[self.v[x as usize] as usize]) {
                    self.pc += 2;
                }
            },
            [0xE, _, 0xA, 0x1] => {
                println!("SKNP Vx");
                if !keypad.is_key_pressed(&keypad.keys[self.v[x as usize] as usize]) {
                    self.pc += 2;
                }
            },
            [0xF, _, 0x0, 0x7] => {
                println!("LD Vx, DT");
                self.v[x as usize] = self.dt;
            },
            [0xF, _, 0x0, 0xA] => {
                println!("LD Vx, K");
                let mut key_pressed = false;
                for i in 0..16 {
                    if keypad.is_key_pressed(&keypad.keys[i]) {
                        self.v[x as usize] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }
                if !key_pressed {
                    self.pc -= 2;
                }
            },
            [0xF, _, 0x1, 0x5] => {
                println!("LD DT, Vx");
                self.dt = self.v[x as usize];
            },
            [0xF, _, 0x1, 0x8] => {
                println!("LD ST, Vx");
                self.st = self.v[x as usize];
            },
            [0xF, _, 0x1, 0xE] => {
                println!("ADD I, Vx");
                let sum = self.i as u32 + self.v[x as usize] as u32;
                self.i = (sum & 0xFFF) as u16;
                self.v[0xF] = (sum > 0xFFF) as u8;
            },
            [0xF, _, 0x2, 0x9] => {
                println!("LD F, Vx");
                self.i = ((self.v[x as usize] & 0xF) * 5) as u16;
            },
            [0xF, _, 0x3, 0x3] => {
                println!("LD B, Vx");
                memory.write_byte(self.i + 2, self.v[x as usize] % 10);
                memory.write_byte(self.i + 1, (self.v[x as usize] / 10) % 10);
                memory.write_byte(self.i, self.v[x as usize] / 100);
            },
            [0xF, _, 0x5, 0x5] => {
                println!("LD [I], Vx");
                for i in 0..=x {
                    memory.write_byte((self.i + i as u16) & 0xFFF, self.v[i as usize]);
                }
            },
            [0xF, _, 0x6, 0x5] => {
                println!("LD Vx, [I]");
                for i in 0..=x {
                    self.v[i as usize] = memory.read_byte((self.i + i as u16) & 0xFFF);
                }
            },
            _ => {
                eprint!("Error: Unexpected opcode");
            },
        }
    }

    fn fetch_word(&mut self, memory: &mut Memory) -> u16 {
        let high_byte = memory.read_byte(self.pc);
        let low_byte = memory.read_byte(self.pc + 1);
        let word = ((high_byte as u16) << 8) | low_byte as u16;

        self.pc += 2;
        word
    }
}