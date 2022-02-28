use std::fs;
use std::process;

use crate::errors::*;
use crate::util::*;

// ascii table for quick string building

pub static ASCII: [&'static str; 128] = [ "\x00", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", "\x09", "\x0a", "\x0b", "\x0c", "\x0d", "\x0e", "\x0f", "\x10", "\x11", "\x12", "\x13", "\x14", "\x15", "\x16", "\x17", "\x18", "\x19", "\x1a", "\x1b", "\x1c", "\x1d", "\x1e", "\x1f", "\x20", "\x21", "\x22", "\x23", "\x24", "\x25", "\x26", "\x27", "\x28", "\x29", "\x2a", "\x2b", "\x2c", "\x2d", "\x2e", "\x2f", "\x30", "\x31", "\x32", "\x33", "\x34", "\x35", "\x36", "\x37", "\x38", "\x39", "\x3a", "\x3b", "\x3c", "\x3d", "\x3e", "\x3f", "\x40", "\x41", "\x42", "\x43", "\x44", "\x45", "\x46", "\x47", "\x48", "\x49", "\x4a", "\x4b", "\x4c", "\x4d", "\x4e", "\x4f", "\x50", "\x51", "\x52", "\x53", "\x54", "\x55", "\x56", "\x57", "\x58", "\x59", "\x5a", "\x5b", "\x5c", "\x5d", "\x5e", "\x5f", "\x60", "\x61", "\x62", "\x63", "\x64", "\x65", "\x66", "\x67", "\x68", "\x69", "\x6a", "\x6b", "\x6c", "\x6d", "\x6e", "\x6f", "\x70", "\x71", "\x72", "\x73", "\x74", "\x75", "\x76", "\x77", "\x78", "\x79", "\x7a", "\x7b", "\x7c", "\x7d", "\x7e", "\x7f" ];

pub struct VMLCpu {
    registers: Vec<u64>,
    return_stack: Vec<usize>,
    memory: Vec<u8>,
    stack: Vec<u64>,
    pc: usize,
    flags: u8
}

impl VMLCpu {
    pub fn new() -> Self {
        return VMLCpu {
            registers: vec![0; 16],
            memory: vec![0; 134217728],
            stack: Vec::new(),
            return_stack: Vec::new(),
            pc: 0,
            flags: 0
        }
    }
    
    pub fn read_u64(self: &VMLCpu, index: usize, rom: &Vec<u8>) -> u64 {
        let mut val: u64 = 0;
        for i in 0..8 {
            val += (rom[index + i as usize] as u64) << (i * 8);
        }

        return val;
    }

    pub fn read_usize(self: &VMLCpu, index: usize, rom: &Vec<u8>) -> usize {
        let mut val: usize = 0;
        for i in 0..4 {
            val += (rom[index + i as usize] as usize) << (i * 8);
        }
        
        return val;
    }

    #[allow(non_snake_case)]
    pub fn read_NTString(self: &VMLCpu, index: usize, rom: &Vec<u8>) -> String {
        let mut ret: String = String::new();
        let mut ind: usize = index;
        while rom[ind] != 0 {
            ret += ASCII[rom[ind] as usize];
            ind += 1;
        }
        return ret;
    }
    
    #[allow(non_snake_case)]
    pub fn read_buffered_NTString(self: &VMLCpu, index: usize) -> String {
        let mut ret: String = String::new();
        let mut ind: usize = index;
        while self.memory[ind] != 0 {
            ret += ASCII[self.memory[ind] as usize];
            ind += 1;
        }
        return ret;
    }

    pub fn exec(self: &mut VMLCpu, rom: &Vec<u8>, code_len: &usize) {
        let mut args: u8;
        let mut jump_amnt: usize;
        while (self.pc+1 < *code_len) && (self.flags & 0x01 == 0x00){
            args = rom[self.pc + 1];
            jump_amnt = 2;
            match &rom[self.pc] {
                0x00 => {
                    self.registers[(args & 0x0F) as usize] = self.read_u64(self.pc + 2, rom);
                    self.pc += 8;
                },
                0x01 => {
                    self.registers[
                        (args & 0x0F) as usize] = self.memory[self.read_usize(
                            self.pc + 2,
                            rom)] as u64;
                    self.pc += 4;
                },
                0x02 => {
                    self.registers[(args & 0x0F) as usize] = self.memory[self.read_usize(
                        self.pc + 2,
                        rom) as usize + (self.registers[((args & 0xF0) >> 4) as usize]) as usize] as u64;
                    self.pc += 4;
                },
                0x03 => {
                    self.registers[(args & 0x0F) as usize] = self.registers[
                        ((args & 0xF0) >> 4) as usize];
                },
                0x04 => {
                    let mem = self.read_usize(self.pc + 2, rom);
                    self.memory[mem] = (self.registers[(
                        args & 0x0F) as usize] & 0xFF) as u8;
                    self.pc += 4;
                },
                0x05 => {
                    let mem = self.read_usize(self.pc + 2, rom) + self.registers[((args & 0xf0) >> 4) as usize] as usize;
                    self.memory[mem] = (self.registers[(args & 0x0F) as usize] & 0xFF) as u8;
                    self.pc += 4;
                },
                0x06 => {
                    self.stack.push(self.registers[(args & 0x0F) as usize]);
                },
                0x07 => {
                    self.registers[(args & 0x0F) as usize] = self.stack.pop().unwrap();
                },
                0x08 => {
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] + self.registers[
                        ((args & 0xF0) >> 4) as usize];
                },
                0x09 => {
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] - self.registers[
                        ((args & 0xF0) >> 4) as usize];
                },
                0x0A => { 
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] * self.registers[
                        ((args & 0xF0) >> 4) as usize];
                },
                0x0B => {
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] / self.registers[
                        ((args & 0xF0) >> 4) as usize];
                },
                0x0C => {
                    self.registers[(args & 0x0F) as usize] = to_u64(to_f64(self.registers[(args & 0x0F) as usize]) + to_f64(self.registers[((args & 0xF0) >> 4) as usize]));
                },
                0x0D => {
                    self.registers[(args & 0x0F) as usize] = to_u64(to_f64(self.registers[(args & 0x0F) as usize]) - to_f64(self.registers[((args & 0xF0) >> 4) as usize]));
                },
                0x0E => {
                    self.registers[(args & 0x0F) as usize] = to_u64(to_f64(self.registers[(args & 0x0F) as usize]) * to_f64(self.registers[((args & 0xF0) >> 4) as usize]));
                },
                0x0F => {
                    self.registers[(args & 0x0F) as usize] = to_u64(to_f64(self.registers[(args & 0x0F) as usize]) / to_f64(self.registers[((args & 0xF0) >> 4) as usize]));
                },
                0x10 => {
                    self.registers[(args & 0x0F) as usize] = to_u64(to_f64(self.registers[(args & 0x0F) as usize]));
                },
                0x11 => {
                    self.registers[(args & 0x0F) as usize] = i64_bits(to_f64(self.registers[(args & 0x0F) as usize]) as i64);
                },
                0x12 => {
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] << self.registers[((args & 0xF0) >> 4) as usize];
                },
                0x13 => {
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] >> self.registers[((args & 0xF0) >> 4) as usize];
                },
                0x14 => {
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] & self.registers[((args & 0xF0) >> 4) as usize];
                },
                0x15 => {
                    self.registers[(args & 0x0F) as usize] = self.registers[(args & 0x0F) as usize] | self.registers[((args & 0xF0) >> 4) as usize];
                },
                0x16 => {
                    self.registers[(args & 0x0F) as usize] = !self.registers[(args & 0x0F) as usize];
                },
                0x17 => {
                    self.flags = 0;

                    let reg1_c: u64 = self.registers[(args & 0x0F) as usize];
                    let reg2_c: u64 = self.registers[((args & 0xF0) >> 4) as usize];
                    if reg1_c == reg2_c { self.flags = self.flags | 0b00000100; }
                    if reg1_c > reg2_c  { self.flags = self.flags | 0b01000000; }
                    if reg1_c < reg2_c  { self.flags = self.flags | 0b00100000; }
                },
                0x18 => {
                    self.flags = 0;

                    let reg1_c: f64 = to_f64(self.registers[(args & 0x0F) as usize]);
                    let reg2_c: f64 = to_f64(self.registers[((args & 0xF0) >> 4) as usize]);
                    if reg1_c == reg2_c { self.flags = self.flags | 0b00000100; }
                    if reg1_c > reg2_c  { self.flags = self.flags | 0b01000000; }
                    if reg1_c < reg2_c  { self.flags = self.flags | 0b00100000; }
                },
                0x19 => {
                    jump_amnt = 0;
                    self.pc = self.read_usize(self.pc + 2, rom);
                },
                0x1A => {
                    if (self.flags & 0b00000100) != 0 {
                        self.pc = self.read_usize(self.pc + 2, rom);
                        jump_amnt = 0;
                    } else {
                        self.pc += 4;
                    }
                },
                0x1B => {
                    if (self.flags & 0b00000100) == 0 {
                        self.pc = self.read_usize(self.pc + 2, rom);
                        jump_amnt = 0;
                    } else {
                        self.pc += 4;
                    }
                },
                0x1C => {
                    if (self.flags & 0b01000000) != 0 {
                        self.pc = self.read_usize(self.pc + 2, rom);
                        jump_amnt = 0;
                    } else {
                        self.pc += 4;
                    }
                },
                0x1D => {
                    if (self.flags & 0b00100000) != 0 {
                        self.pc = self.read_usize(self.pc + 2, rom);
                        jump_amnt = 0;
                    } else {
                        self.pc += 4;
                    }
                },
                0x1E => {
                    self.return_stack.push(self.pc + 6);
                    self.pc = self.read_usize(self.pc + 2, rom);
                    jump_amnt = 0;
                },
                0x1F => {
                    self.pc = self.return_stack.pop().unwrap();
                    jump_amnt = 0;
                },
                0x20 => {
                    self.handle_syscalls(self.read_usize(self.pc + 2, rom), rom);
                    self.pc += 4;
                },
                0x22 => {
                    self.flags = self.flags | 0b10000000;
                }
                0x23 => {
                    self.registers[(args & 0x0F) as usize] = self.read_usize(self.pc + 2, rom) as u64;
                    self.pc += 4;
                },
                0x24 => {
                    self.registers[(args & 0x0F) as usize] = self.memory[(self.registers[((args & 0xF0) >> 4) as usize]) as usize] as u64;
                }
                0x25 => {
                    let mut val: u64 = 0;
                    for i in 0..2 {
                        val += (self.memory[(self.registers[((args & 0xF0) >> 4) as usize] + i) as usize] as u64) << (i * 8);
                    }
                    self.registers[(args & 0x0F) as usize] = val;
                },
                0x26 => {
                    let mut val: u64 = 0;
                    for i in 0..4 {
                        val += (self.memory[(self.registers[((args & 0xF0) >> 4) as usize] + i) as usize] as u64) << (i * 8);
                    }
                    self.registers[(args & 0x0F) as usize] = val;
                }
                0x27 => {
                    let mut val: u64 = 0;
                    for i in 0..8 {
                        val += (self.memory[(self.registers[((args & 0xF0) >> 4) as usize] + i) as usize] as u64) << (i * 8);
                    }
                    self.registers[(args & 0x0F) as usize] = val;
                }
                0x28 => {
                    self.memory[(self.registers[((args & 0xF0) >> 4) as usize]) as usize] = self.registers[(args & 0x0F) as usize] as u8;
                }
                0x29 => {
                    for i in 0..2 {
                        self.memory[(self.registers[((args & 0xF0) >> 4) as usize] + i) as usize] = (self.registers[(args & 0x0F) as usize] >> (i * 8)) as u8;
                    }
                },
                0x2A => {
                    for i in 0..4 {
                        self.memory[(self.registers[((args & 0xF0) >> 4) as usize] + i) as usize] = (self.registers[(args & 0x0F) as usize] >> (i * 8)) as u8;
                    }
                }
                0x2B => {
                    for i in 0..8 {
                        self.memory[(self.registers[((args & 0xF0) >> 4) as usize] + i) as usize] = (self.registers[(args & 0x0F) as usize] >> (i * 8)) as u8;
                    }
                },
                0x2C => {
                    let loc = self.registers[(args & 0x0F) as usize] as usize;
                    let dest = self.registers[((args & 0xF0) >> 4) as usize] as usize;
                    let mut i: usize = 0;

                    while rom[loc + i] != 0x00 {
                        self.memory[dest + i] = rom[loc + i];
                        i += 1;
                    }
                },
                0x2D => {
                    let loc = self.registers[(args & 0x0F) as usize] as usize;
                    let mloc = self.registers[((args & 0xF0) >> 4) as usize] as usize;
                    if self.read_buffered_NTString(loc) == self.read_buffered_NTString(mloc) {
                        self.flags = self.flags | 0b00000100;
                        self.stack.push(1);
                    }
                }
                0x2E => {
                    self.flags = 0x00;
                    let loc = self.registers[(args & 0x0F) as usize] as usize;
                    let mloc = self.registers[((args & 0xF0) >> 4) as usize] as usize;
                    if self.read_NTString(loc, rom) == self.read_NTString(mloc, rom) {
                        self.flags = self.flags | 0b00000100;
                    }
                }
                0x2F => {
                    let op1 = to_f64(self.registers[(args & 0x0F) as usize]);
                    let op2 = to_f64(self.registers[((args & 0xF0) >> 4) as usize]);
                    self.registers[(args & 0x0F) as usize] = to_u64(op1.powf(op2));
                }
                0x30 => {
                    let op1 = to_f64(self.registers[(args & 0x0F) as usize]);
                    let op2 = to_f64(self.registers[((args & 0xF0) >> 4) as usize]);
                    self.registers[(args & 0x0F) as usize] = to_u64(op1.powf(1.0 / op2));
                },
                0x31 => {
                    self.handle_syscalls(self.registers[(args & 0x0F) as usize] as usize, rom);
                },
                _ => eprintln!("Unrecognized opcode at {:#018x} (ERR)", self.pc)
            }
            self.pc += jump_amnt;
            if (self.flags & 0b10000000) != 0 {
                loop {}
            }
        }
/*               
        println!("== Register dump: ==");
        for i in 0..16 {
            println!("r{} = {:#018x}", i, self.registers[i]);
        }
        println!("== First 200 bytes of memory ==");
        for i in 0..100 {
            print!("{:#02x} ", self.memory[i]);
            if i % 5 == 0 && i != 0 {
                println!();
            }
        }
        println!();
*/        
    }

    fn handle_syscalls(self: &mut VMLCpu, syscall: usize, rom: &Vec<u8>) {
        match &syscall {
            0x00 => print!("{}", self.stack.pop().unwrap()),
            0x01 => {
                let addr: usize = self.stack.pop().unwrap() as usize;
                print!("{}", self.read_NTString(addr, rom));
            },
            0x02 => print!("{:#064b}", self.stack.pop().unwrap()),
            0x03 => print!("{:#018x}", self.stack.pop().unwrap()),
            0x04 => {
                let addr: usize = self.stack.pop().unwrap() as usize;
                print!("{}", self.read_buffered_NTString(addr));
            },
            0x05 => {
                let buffer: usize = self.stack.pop().unwrap() as usize;
                let mut input: String = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                for i in 0..input.len()-1 {
                    self.memory[buffer + i] = input.as_bytes()[i];
                }
                self.memory[buffer + input.len()] = 0x00;
            },
            0x06 => print!("{}", to_f64(self.stack.pop().unwrap())),
            0x07 => print!("{}", self.stack.pop().unwrap() as i64),
            0x08 => {
                let file_addr = self.stack.pop();
                let buffer = self.stack.pop();
                let context = self.stack.pop();

                let mut filename: String = String::new();
                match &context.unwrap() {
                    0 => filename = self.read_NTString(file_addr.unwrap() as usize, rom),
                    1 => filename = self.read_buffered_NTString(file_addr.unwrap() as usize),
                    _ => eprintln!("Unknown FileBuffer read type: {}.", context.unwrap()),
                }

                let filecontents = fs::read_to_string(filename).expect("Failed to read file.");
                let filec_buf = filecontents.as_bytes();
                for i in 0..filec_buf.len() {
                    self.memory[(buffer.unwrap() as usize) + i] = filec_buf[i];
                }
            },
            0x09 => {
                let file_addr = self.stack.pop();
                let buffer = self.stack.pop();
                let context = self.stack.pop();
                
                let mut filename: String = String::new();
                match &context.unwrap() {
                    0 => filename = self.read_NTString(file_addr.unwrap() as usize, rom),
                    1 => filename = self.read_buffered_NTString(file_addr.unwrap() as usize),
                    _ => eprintln!("Unknown FileBuffer write type: {}.", context.unwrap()),
                }

                fs::write(filename, &*self.read_buffered_NTString(buffer.unwrap() as usize))
                    .expect("Unable to write to file!");
            },
            _    => {
                format_errora("Error - unrecognized SYSCALL. Perhaps you're missing an extension?".to_string());
                process::exit(1);
            }
        }
    }
}
