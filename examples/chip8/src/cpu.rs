use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::misc::{V, I, FONT, Instruction};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Cpu {
    pub memory: [u8; 4096],
    pub registers: [V; 16],
    pub stack: [u16; 16],
    pub i_reg: I,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub display: [u32; WIDTH * HEIGHT],

    pub pc: u16,
    pub sp: u8,
    pub keys: [bool; 16],
    pub rng: rand::rngs::SmallRng,
}

impl Default for Cpu {
    fn default() -> Self {
        let mut memory = [0; 4096];
        memory[0x050..0x050 + FONT.len()].copy_from_slice(&FONT);

        Self {
            memory,
            registers: [0; 16],
            stack: [0; 16],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [0; WIDTH * HEIGHT],

            pc: 0x200,
            sp: 0,
            keys: [false; 16],
            rng: SmallRng::seed_from_u64(0),
        }
    }
}

impl Cpu {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }

    pub fn decode(instruction: u16) -> Instruction {
        let group = instruction & 0xF000;
        let vx = ((instruction & 0x0F00) >> 8) as u8;
        let vy = ((instruction & 0x00F0) >> 4) as u8;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        match group {
            0x0000 => match instruction {
                0x00E0 => Instruction::CLEAR,
                0x00EE => Instruction::RETURN,
                _ => panic!("unknown 0x0--- instruction: {instruction:#X}"),
            },
            0x1000 => Instruction::JUMP { addr: nnn },
            0x2000 => Instruction::CALL { addr: nnn },
            0x3000 => Instruction::SKIPEQ { vx, byte: nn },
            0x4000 => Instruction::SKIPNE { vx, byte: nn },
            0x5000 => Instruction::SKIPEQREG { vx, vy },
            0x6000 => Instruction::SET { vx, byte: nn },
            0x7000 => Instruction::ADD { vx, byte: nn },
            0x8000 => match n {
                0x0 => Instruction::MOV { vx, vy },
                0x1 => Instruction::OR { vx, vy },
                0x2 => Instruction::AND { vx, vy },
                0x3 => Instruction::XOR { vx, vy },
                0x4 => Instruction::ADDREG { vx, vy },
                0x5 => Instruction::SUB { vx, vy },
                0x6 => Instruction::SHR { vx },
                0x7 => Instruction::SUBREVERSE { vx, vy },
                0xE => Instruction::SHL { vx },
                _ => panic!("unknown 0x8--n instruction: {instruction:#X}"),
            },
            0x9000 => Instruction::SKIPNEREG { vx, vy },
            0xA000 => Instruction::SETI { addr: nnn },
            0xB000 => Instruction::JUMPV0 { addr: nnn },
            0xC000 => Instruction::RAND { vx, mask: nn },
            0xD000 => Instruction::DRAW { vx, vy, n },
            0xE000 => match nn {
                0x9E => Instruction::KEYDOWN { vx },
                0xA1 => Instruction::KEYUP { vx },
                _ => panic!("unknown 0xe-nn instruction: {instruction:#X}"),
            },
            0xF000 => match nn {
                0x07 => Instruction::GETDELAY { vx },
                0x0A => Instruction::WAITKEY { vx },
                0x15 => Instruction::SETDELAY { vx },
                0x18 => Instruction::SETSOUND { vx },
                0x1E => Instruction::ADDI { vx },
                0x29 => Instruction::FONT { vx },
                0x33 => Instruction::BCD { vx },
                0x55 => Instruction::STORE { vx },
                0x65 => Instruction::LOAD { vx },
                _ => panic!("unknown 0xf-nn instruction: {instruction:#X}"),
            },
            _ => panic!("unknown instruction: {instruction:#X}"),
        }
    }

    pub const fn skip(&mut self) {
        self.pc += 2;
    }

    const fn set_pixel_xor(&mut self, x: usize, y: usize, bit: u8) -> bool {
        if bit == 0 {
            return false;
        }

        let idx = (y % HEIGHT) * WIDTH + (x % WIDTH);
        let prev_set = self.display[idx] != 0;
        let new_set = !prev_set;
        self.display[idx] = if new_set { 0xFF_FF_FF_FF } else { 0 };
        prev_set
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::CLEAR => self.display.fill(0),
            Instruction::RETURN => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            Instruction::JUMP { addr } => {
                self.pc = addr;
            }
            Instruction::CALL { addr } => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            Instruction::SKIPEQ { vx, byte } => {
                if self.registers[vx as usize] == byte {
                    self.skip();
                }
            }
            Instruction::SKIPNE { vx, byte } => {
                if self.registers[vx as usize] != byte {
                    self.skip();
                }
            }
            Instruction::SKIPEQREG { vx, vy } => {
                if self.registers[vx as usize] == self.registers[vy as usize] {
                    self.skip();
                }
            }
            Instruction::SKIPNEREG { vx, vy } => {
                if self.registers[vx as usize] != self.registers[vy as usize] {
                    self.skip();
                }
            }
            Instruction::SET { vx, byte } => {
                self.registers[vx as usize] = byte;
            }
            Instruction::ADD { vx, byte } => {
                self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(byte);
            }
            Instruction::MOV { vx, vy } => {
                self.registers[vx as usize] = self.registers[vy as usize];
            }
            Instruction::OR { vx, vy } => {
                self.registers[vx as usize] |= self.registers[vy as usize];
            }
            Instruction::AND { vx, vy } => {
                self.registers[vx as usize] &= self.registers[vy as usize];
            }
            Instruction::XOR { vx, vy } => {
                self.registers[vx as usize] ^= self.registers[vy as usize];
            }
            Instruction::ADDREG { vx, vy } => {
                let (res, overflow) =
                    self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);
                self.registers[vx as usize] = res;
                self.registers[0xF] = u8::from(overflow);
            }
            Instruction::SUB { vx, vy } => {
                let vx_val = self.registers[vx as usize];
                let vy_val = self.registers[vy as usize];

                self.registers[0xF] = u8::from(vx_val > vy_val);
                self.registers[vx as usize] = vx_val.wrapping_sub(vy_val);
            }
            Instruction::SUBREVERSE { vx, vy } => {
                let vx_val = self.registers[vx as usize];
                let vy_val = self.registers[vy as usize];

                self.registers[0xF] = u8::from(vy_val > vx_val);
                self.registers[vx as usize] = vy_val.wrapping_sub(vx_val);
            }
            Instruction::SHR { vx } => {
                self.registers[0xF] = self.registers[vx as usize] & 0x1;
                self.registers[vx as usize] >>= 1;
            }
            Instruction::SHL { vx } => {
                self.registers[0xF] = (self.registers[vx as usize] & 0x80) >> 7;
                self.registers[vx as usize] <<= 1;
            }
            Instruction::SETI { addr } => {
                self.i_reg = addr;
            }
            Instruction::JUMPV0 { addr } => {
                self.pc = addr + u16::from(self.registers[0]);
            }
            Instruction::RAND { vx, mask } => {
                self.registers[vx as usize] = self.rng.random::<u8>() & mask;
            }
            Instruction::DRAW { vx, vy, n } => {
                let x0 = self.registers[vx as usize] as usize;
                let y0 = self.registers[vy as usize] as usize;
                let height = n as usize;

                // VF = 1 if any pixels are erased (collision), otherwise 0
                self.registers[0xF] = 0;

                for row in 0..height {
                    let mem_idx = self.i_reg as usize + row;
                    if mem_idx >= self.memory.len() {
                        break;
                    } // safety
                    let sprite_byte = self.memory[mem_idx];

                    for bit in 0..8 {
                        let pixel = (sprite_byte >> (7 - bit)) & 1;
                        let x = (x0 + bit) % WIDTH;
                        let y = (y0 + row) % HEIGHT;

                        if self.set_pixel_xor(x, y, pixel) {
                            self.registers[0xF] = 1;
                        }
                    }
                }
            }
            Instruction::KEYDOWN { vx } => {
                let key = self.registers[vx as usize] as usize;
                if key < 16 && self.keys[key] {
                    self.skip();
                }
            }
            Instruction::KEYUP { vx } => {
                let key = self.registers[vx as usize] as usize;
                if key < 16 && !self.keys[key] {
                    self.skip();
                }
            }
            Instruction::GETDELAY { vx } => {
                self.registers[vx as usize] = self.delay_timer;
            }
            Instruction::WAITKEY { vx } => {
                if let Some((key, _)) = self.keys.iter().enumerate().find(|&(_, &pressed)| pressed)
                {
                    self.registers[vx as usize] = key as u8;
                } else {
                    self.pc = self.pc.wrapping_sub(2);
                }
            }
            Instruction::SETDELAY { vx } => {
                self.delay_timer = self.registers[vx as usize];
            }
            Instruction::SETSOUND { vx } => {
                self.sound_timer = self.registers[vx as usize];
            }
            Instruction::ADDI { vx } => {
                self.i_reg = u16::from(self.registers[vx as usize]);
            }
            Instruction::FONT { vx } => {
                let digit = self.registers[vx as usize] as usize;
                self.i_reg = 0x050 + (digit * 5) as u16;
            }
            Instruction::BCD { vx } => {
                let value = self.registers[vx as usize];
                let i = self.i_reg as usize;

                self.memory[i] = value / 100;
                self.memory[i + 1] = (value / 10) % 10;
                self.memory[i + 2] = value % 10;
            }
            Instruction::STORE { vx } => {
                let i = self.i_reg as usize;
                for reg in 0..=vx as usize {
                    self.memory[i + reg] = self.registers[reg];
                }
            }
            Instruction::LOAD { vx } => {
                let i = self.i_reg as usize;
                for reg in 0..=vx as usize {
                    self.registers[reg] = self.memory[i + reg];
                }
            }
        }
    }

    pub fn step(&mut self) {
        let pc_index = self.pc as usize;
        let instruction = u16::from_be_bytes([self.memory[pc_index], self.memory[pc_index + 1]]);
        self.pc = self.pc.wrapping_add(2);
        let decoded = Self::decode(instruction);
        self.execute(decoded);
    }
}
