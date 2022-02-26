use crate::token::*;
use crate::errors::*;
use crate::variable::*;
use crate::util::*;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::u64;
use std::u32;
use std::u8;
use std::process;

use std::collections::HashMap;

pub struct Lexer {
    tokens: Vec<Token>,
    toks: String,
    expr: String,
    lexer_state: u16,
    expected: i8
}

impl Lexer {
    pub fn new() -> Self {
        return Lexer {
            tokens: Vec::new(),
            toks: String::new(),
            expr: String::new(),
            lexer_state: 0,
            expected: 0,
        }
    }

    pub fn clear_state(self: &mut Lexer) { self.lexer_state = 0; }
    pub fn set_string_bit(self: &mut Lexer) { self.lexer_state = self.lexer_state | 1; }
    pub fn set_int_bit(self: &mut Lexer) { self.lexer_state = self.lexer_state | (1 << 1); }
    pub fn set_label_bit(self: &mut Lexer) { self.lexer_state = self.lexer_state | (1 << 2); }
    pub fn set_prereg_bit(self: &mut Lexer) { self.lexer_state = self.lexer_state | (1 << 3); }
    pub fn set_comment_bit(self: &mut Lexer) { self.lexer_state = self.lexer_state | (1 << 4); }
    pub fn set_method_bit(self: &mut Lexer) { self.lexer_state = self.lexer_state | (1 << 5); }
    pub fn set_variable_bit(self: &mut Lexer) { self.lexer_state = self.lexer_state | (1 << 6); }

    pub fn add_token(self: &mut Lexer, t: TokenType, d: &str) {
        self.tokens.push(Token::new(t, String::from(d)));
        self.toks = String::from("");
    }

    pub fn read_line_num(self: &Lexer, file_data: &String, line: usize) -> String {
        let mut index: usize = 0;
        let mut filedescriptor: usize = 0;
        let bytes = file_data.as_bytes();
        let mut line_string: String = String::new();
        while index < line {
            if bytes[filedescriptor] == 0x0a {
                index += 1;
            }
            filedescriptor += 1;
        }
        while bytes[filedescriptor] != 0x0a {
            line_string += &*format!("{}", bytes[filedescriptor] as char);
            filedescriptor += 1;
        }

        return line_string;
    }

    pub fn read_chars_upto(self: &Lexer, file_data: &String, line: usize) -> String {
        let mut index: usize = 0;
        let mut filedescriptor: usize = 0;
        let bytes = file_data.as_bytes();
        let mut line_string: String = String::new();
        while index < line {
            line_string += &*format!("{}", bytes[filedescriptor] as char);
            if bytes[filedescriptor] == 0x0a {
                line_string += "\n";
                index += 1;
            }
            filedescriptor += 1;
        }
        while bytes[filedescriptor] != 0x0a {
            line_string += &*format!("{}", bytes[filedescriptor] as char);
            filedescriptor += 1;
        }

        line_string += "\n";

        return line_string;
    }

    fn manage_includes(self: &Lexer, data_i: String) -> String {
        let mut changes: usize = 1;
        let mut data = format!("{}", data_i);
        let mut new: String = String::new();
        let mut include: bool = false;
        let mut modified: bool = false;
        let mut files: Vec<String> = Vec::new();

        while changes != 0 {
            changes = 0;
            if data.contains("include") {
                new = String::new();
                for c in data.replace("\n", "\n ").split(" ") {
                    if include {
                        let modified_c = c.replace("\n", "");
                        let replaced_sm = c.replace("\"", "");
                        if !modified_c.contains("\"") || modified_c.matches("\"").count() < 2{
                            format_errora("Filename must be surrounded in a pair of \"\" for include.".to_string());
                            process::exit(1);
                        }
                        if !files.contains(&format!("{}", modified_c)) {
                            let contents = fs::read_to_string(replaced_sm.clone()).expect(&*format!("Unable to read file '{}'", replaced_sm));
                            new += &*contents;
                            files.push(format!("{}", modified_c));
                        }
                        include = !include;
                        continue;
                    }
                    if c == "include" {
                        include = true;
                        modified = true;
                        changes = 1;
                    } else if c.contains("include") {
                        format_errora("Include must take the following form: `include \"<filename>\"`.".to_string());
                        process::exit(1);
                    }
                    else {
                        new += &*format!("{} ", c);
                    }
                }
                data = new.clone();
            }
        }
        if !modified { new = data_i.clone(); }
        return new;
    }


    pub fn lex_asm(self: &mut Lexer, file_data: String) {
        let mut default_bitlen: bool = false;
        let chars = file_data.chars();
        let mut line: usize = 0;
        let mut mul_reg: bool = false;

        for i in chars {
            if i != '\n' && i != '\t' && i != ' ' { self.toks += &String::from(i); }
            if i == '\n' && (self.lexer_state == 0) {
                if &*self.toks != "" {
                    format_errorl("Syntax error".to_string(),
                                    line, 
                                    self.read_line_num(&file_data, line),
                    );
                    process::exit(1);
                }
            }
            if self.lexer_state == 0 {
                match &*self.toks {
                    "mov" => { self.add_token(TokenType::INSTRUCTION, "mov"); self.expected = 9; },
                    "ldr" => { self.add_token(TokenType::INSTRUCTION, "ldr"); self.expected = 5; },
                    "indl" => { self.add_token(TokenType::INSTRUCTION, "indl"); self.expected = 9; }, 
                    "cpy" => { self.add_token(TokenType::INSTRUCTION, "cpy"); self.expected = 1; },
                    "str" => { self.add_token(TokenType::INSTRUCTION, "str"); self.expected = 5; },
                    "inds" => { self.add_token(TokenType::INSTRUCTION, "inds"); self.expected = 5; },
                    "push" => { self.add_token(TokenType::INSTRUCTION, "push"); self.expected = 1; },
                    "pop" => { self.add_token(TokenType::INSTRUCTION, "pop"); self.expected = 1; },
                    "iadd" => { self.add_token(TokenType::INSTRUCTION, "iadd"); self.expected = 1; },
                    "isub" => { self.add_token(TokenType::INSTRUCTION, "isub"); self.expected = 1; },
                    "imul" => { self.add_token(TokenType::INSTRUCTION, "imul"); self.expected = 1; },
                    "idiv" => { self.add_token(TokenType::INSTRUCTION, "idiv"); self.expected = 1; },
                    "dadd" => { self.add_token(TokenType::INSTRUCTION, "dadd"); self.expected = 1; },
                    "dsub" => { self.add_token(TokenType::INSTRUCTION, "dsub"); self.expected = 1; },
                    "dmul" => { self.add_token(TokenType::INSTRUCTION, "dmul"); self.expected = 1; },
                    "ddiv" => { self.add_token(TokenType::INSTRUCTION, "ddiv"); self.expected = 1; },
                    "icst" => { self.add_token(TokenType::INSTRUCTION, "icst"); self.expected = 1; },
                    "dcst" => { self.add_token(TokenType::INSTRUCTION, "dcst"); self.expected = 1; },
                    "shl" => { self.add_token(TokenType::INSTRUCTION, "shl"); self.expected = 1; },
                    "shr" => { self.add_token(TokenType::INSTRUCTION, "shr"); self.expected = 1; },
                    "and" => { self.add_token(TokenType::INSTRUCTION, "and"); self.expected = 1; },
                    "or" => { self.add_token(TokenType::INSTRUCTION, "or"); self.expected = 1; },
                    "neg" => { self.add_token(TokenType::INSTRUCTION, "neg"); self.expected = 1; },
                    "icmp" => { self.add_token(TokenType::INSTRUCTION, "icmp"); self.expected = 1; },
                    "dcmp" => { self.add_token(TokenType::INSTRUCTION, "dcmp"); self.expected = 1; },
                    "jmp" => { self.add_token(TokenType::INSTRUCTION, "jmp"); self.expected = 4; },
                    "beq" => { self.add_token(TokenType::INSTRUCTION, "beq"); self.expected = 4; },
                    "bne" => { self.add_token(TokenType::INSTRUCTION, "bne"); self.expected = 4; },
                    "bgt" => { self.add_token(TokenType::INSTRUCTION, "bgt"); self.expected = 4; },
                    "blt" => { self.add_token(TokenType::INSTRUCTION, "blt"); self.expected = 4; },
                    "jsr" => { self.add_token(TokenType::INSTRUCTION, "jsr"); self.expected = 4; },
                    "ret" => { self.add_token(TokenType::INSTRUCTION, "ret"); self.expected = 0; },
                    "sys" => { self.add_token(TokenType::INSTRUCTION, "sys"); self.expected = 4; },
                    "halt" => { self.add_token(TokenType::INSTRUCTION, "halt"); self.expected = 0; },
                    "adr" => { self.add_token(TokenType::INSTRUCTION, "adr"); self.expected = 5; },
                    "lei" => { self.add_token(TokenType::INSTRUCTION, "lei"); self.expected = 1; },
                    "lst" => { self.add_token(TokenType::INSTRUCTION, "lst"); self.expected = 1; },
                    "ltt" => { self.add_token(TokenType::INSTRUCTION, "ltt"); self.expected = 1; },
                    "lsf" => { self.add_token(TokenType::INSTRUCTION, "lsf"); self.expected = 1; },
                    "sei" => { self.add_token(TokenType::INSTRUCTION, "sei"); self.expected = 1; },
                    "sst" => { self.add_token(TokenType::INSTRUCTION, "sst"); self.expected = 1; },
                    "stt" => { self.add_token(TokenType::INSTRUCTION, "stt"); self.expected = 1; },
                    "ssf" => { self.add_token(TokenType::INSTRUCTION, "ssf"); self.expected = 1; },
                    "bufc" => { self.add_token(TokenType::INSTRUCTION, "bufc"); self.expected = 1; },
                    "bseq" => { self.add_token(TokenType::INSTRUCTION, "bseq"); self.expected = 1; },
                    "lseq" => { self.add_token(TokenType::INSTRUCTION, "lseq"); self.expected = 1; },
                    "pow" => { self.add_token(TokenType::INSTRUCTION, "pow"); self.expected = 1; },
                    "root" => { self.add_token(TokenType::INSTRUCTION, "root"); self.expected = 1; },
                    "call" => { self.add_token(TokenType::INSTRUCTION, "call"); self.expected = 1; },
                    //"externo" => self.add_token(TokenType::INSTRUCTION, "pop"),
                    "$" => { default_bitlen = true; self.toks = String::from(""); }
                    "U$" => { default_bitlen = false; self.toks = String::from(""); }
                    "0x" => { self.set_int_bit(); self.expr = String::from(""); self.toks = String::from(""); },
                    "r" => { self.set_prereg_bit(); self.toks = String::from(""); self.expr = String::from(""); }
                    "." => { self.set_label_bit(); self.toks = String::from(""); self.expr = String::from(""); }
                    "\"" => { self.set_string_bit(); self.toks = String::from(""); self.expr = String::from(""); }
                    ";" => { self.set_comment_bit(); }
                    _ => ()
                }
            } else {
                if (self.lexer_state & 0x02 != 0) && (i != '\n' && i != ' ' && i != '\t') {
                    self.expr += &String::from(i);
                } else if self.lexer_state & 0x02 != 0 {
                    self.add_token(TokenType::INTEGER, &*format!("{}{}", if default_bitlen { "L" } else { "U" }, self.expr));
                    self.expected -= if default_bitlen { 8 } else { 4 };
                    self.clear_state();
                    self.toks = String::from("");
                    default_bitlen = false;
                }

                if (self.lexer_state & 0x08) != 0 {
                    if self.toks == "e" || self.toks == "o" {
                        self.clear_state();
                        if self.toks == "e" { self.toks = String::from("re"); }
                        else { self.toks = String::from("ro"); }
                    }
                    else {
                        if i != '\n' && i != ',' && i != '\t' && i != ' ' {
                            self.expr += &String::from(i);
                        } else {
                            self.add_token(TokenType::REGISTER, &*self.expr.clone());
                            self.toks = String::from("");
                            self.clear_state();
                            if !mul_reg {
                                self.expected -= 1;
                                mul_reg = !mul_reg;
                            } else {
                                mul_reg = !mul_reg;
                            }
                        }
                    }
                }

                if (self.lexer_state & 0x04) != 0 {
                    if i != ':' && i != ' ' && i != '\t' && i != ',' && i != '\n' {
                        self.expr += &String::from(i);
                    } else {
                        if i == ':' {
                            self.add_token(TokenType::LABEL, &*format!("D{}", self.expr));
                        }
                        else {
                            self.add_token(TokenType::LABEL, &*format!("U{}", self.expr));
                            self.expected -= 4;
                        }
                        self.toks = String::from("");
                        self.clear_state();
                    }
                }

                if (self.lexer_state & (1 << 4)) != 0 {
                    if i == '\n' {
                        self.toks = String::from("");
                        self.clear_state();
                    }
                }

                if (self.lexer_state & 0x01) != 0 {
                    if i != '"' {
                        self.expr += &String::from(i);
                    } else {
                        self.add_token(TokenType::STRING, &*self.expr.clone());
                        self.toks = String::from("");
                        self.clear_state();
                    }
                }
            }
            if i == '\n' {
                mul_reg = false;
                if self.expected != 0 {
                    format_errorl("Invalid operands for instruction".to_string(),
                                    line, 
                                    self.read_line_num(&file_data, line),
                    );
                    println!("{}", self.expected);
                    process::exit(1);
                }
                if self.lexer_state == 1 {
                    format_errorl("Unclosed delimiter (did you forget a \"?)".to_string(),
                                    line, 
                                    self.read_line_num(&file_data, line),
                    );
                    process::exit(1);
                }
                line += 1;
            }
        }
    }

    pub fn assemble_asm(self: &mut Lexer) -> usize {
        let path = Path::new("out.bin");
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create file {}: {}", "out.bin", why),
            Ok(file) => file
        };
        let mut file_vec: Vec<String> = Vec::new();
        let mut output_vec: Vec<u8> = Vec::new();
        let mut label_table: HashMap::<String, usize> = HashMap::new();

        let mut token_ind: usize;

        token_ind = 0;

        while token_ind < self.tokens.len() {
            match &self.tokens[token_ind].token_t {
                TokenType::INSTRUCTION => match &*self.tokens[token_ind].data {
                    "mov" => file_vec.push(String::from("00")),
                    "ldr" => file_vec.push(String::from("01")),
                    "indl" => file_vec.push(String::from("02")),
                    "cpy" => file_vec.push(String::from("03")),
                    "str" => file_vec.push(String::from("04")),
                    "inds" => file_vec.push(String::from("05")),
                    "push" => file_vec.push(String::from("06")),
                    "pop" => file_vec.push(String::from("07")),
                    "iadd" => file_vec.push(String::from("08")),
                    "isub" => file_vec.push(String::from("09")),
                    "imul" => file_vec.push(String::from("0a")),
                    "idiv" => file_vec.push(String::from("0b")),
                    "dadd" => file_vec.push(String::from("0c")),
                    "dsub" => file_vec.push(String::from("0d")),
                    "dmul" => file_vec.push(String::from("0e")),
                    "ddiv" => file_vec.push(String::from("0f")),
                    "icst" => file_vec.push(String::from("10")),
                    "dcst" => file_vec.push(String::from("11")),
                    "shl" => file_vec.push(String::from("12")),
                    "shr" => file_vec.push(String::from("13")),
                    "and" => file_vec.push(String::from("14")),
                    "or" => file_vec.push(String::from("15")),
                    "neg" => file_vec.push(String::from("16")),
                    "icmp" => file_vec.push(String::from("17")),
                    "dcmp" => file_vec.push(String::from("18")),
                    "jmp" => { file_vec.push(String::from("19")); file_vec.push(String::from("00")); }
                    "beq" => { file_vec.push(String::from("1a")); file_vec.push(String::from("00")); }
                    "bne" => { file_vec.push(String::from("1b")); file_vec.push(String::from("00")); }
                    "bgt" => { file_vec.push(String::from("1c")); file_vec.push(String::from("00")); }
                    "blt" => { file_vec.push(String::from("1d")); file_vec.push(String::from("00")); }
                    "jsr" => { file_vec.push(String::from("1e")); file_vec.push(String::from("00")); }
                    "ret" => { file_vec.push(String::from("1f")); file_vec.push(String::from("00")); }
                    "sys" => { file_vec.push(String::from("20")); file_vec.push(String::from("00")); }
                    "halt" => { file_vec.push(String::from("22")); file_vec.push(String::from("00")); }
                    "adr" => file_vec.push(String::from("23")),
                    "lei" => file_vec.push(String::from("24")),
                    "lst" => file_vec.push(String::from("25")),
                    "ltt" => file_vec.push(String::from("26")),
                    "lsf" => file_vec.push(String::from("27")),
                    "sei" => file_vec.push(String::from("28")),
                    "sst" => file_vec.push(String::from("29")),
                    "stt" => file_vec.push(String::from("2a")),
                    "ssf" => file_vec.push(String::from("2b")),
                    "bufc" => file_vec.push(String::from("2c")),
                    "bseq" => file_vec.push(String::from("2d")),
                    "lseq" => file_vec.push(String::from("2e")),
                    "pow" => file_vec.push(String::from("2f")),
                    "root" => file_vec.push(String::from("30")),
                    "call" => file_vec.push(String::from("31")),
                    _ => println!("Unimplemented instruction!")
                },
                TokenType::REGISTER => {
                    if token_ind != self.tokens.len() -1 {
                        if self.tokens[token_ind + 1].token_t == TokenType::REGISTER {
                            file_vec.push(format!("{:x}", ((self.tokens[token_ind].data.parse::<u8>().unwrap()) + (self.tokens[token_ind + 1].data.parse::<u8>().unwrap() << 4))));
                            token_ind += 1;
                        } else {
                            file_vec.push(format!("{:x}", self.tokens[token_ind].data.parse::<u8>().unwrap()));
                        }
                    } else {
                        file_vec.push(format!("{:x}", self.tokens[token_ind].data.parse::<u8>().unwrap()));
                    }
                },
                TokenType::INTEGER => {
                    let mut chars = self.tokens[token_ind].data.chars();
                    chars.next();
                    let string = chars.as_str();
                    if self.tokens[token_ind].data.chars().next().unwrap() == 'L' {
                        let mut value: u64 = u64::from_str_radix(string, 16).unwrap();
                        for _ in 0..8 {
                            file_vec.push(format!("{:x}", value & 0xFF));
                            value = value >> 8;
                        }
                    } else {
                        let mut value: u32 = u32::from_str_radix(string, 16).unwrap();
                        for _ in 0..4 {
                            file_vec.push(format!("{:x}", value & 0xFF));
                            value = value >> 8;
                        }
                    }
                },
                TokenType::LABEL => {                    
                    file_vec.push(format!("{}", self.tokens[token_ind].data));
                }
                TokenType::STRING => {
                    let mut escape = false;
                    for chars in self.tokens[token_ind].data.chars() {
                        if chars == '\\' {
                            escape = true;
                        }
                        else if !escape { file_vec.push(format!("{:x}", chars as u8)); }
                        else {
                            match chars {
                                'n' => file_vec.push("0a".to_string()),
                                't' => file_vec.push("09".to_string()),
                                'v' => file_vec.push("0b".to_string()),
                                '"' => file_vec.push("22".to_string()),
                                'r' => file_vec.push("0d".to_string()),
                                'f' => file_vec.push("0c".to_string()),
                                'b' => file_vec.push("08".to_string()),
                                'a' => file_vec.push("07".to_string()),
                                'e' => file_vec.push("1b".to_string()),
                                '\'' => file_vec.push("27".to_string()),
                                '\\' => file_vec.push("5c".to_string()),
                                _ => {
                                    format_errora(format!("Unknown escape sequence found! ('\\{}')", chars));
                                    process::exit(1);
                                }
                            }
                            escape = false;
                        }
                    }
                    file_vec.push(String::from("00"));
                },
                _ => warninga("Unimplemented token found!"),
            }
            token_ind += 1;
        }
        // because I can't seem to figure out the lengths of things, I'm going
        // to just use something I like to call a "post-processor".
        
        let mut passed: usize = 0;

        for label in &file_vec {
            let mut chars = label.chars();
            chars.next();
            let reduced = chars.as_str();
            if label.chars().next().unwrap() == 'D' {
                label_table.insert(reduced.to_string(), passed);
            } else if label.chars().next().unwrap() == 'U' {
                passed += 4;
            } else {
                passed += 1;
            }
        }

        for passed_bytes in &file_vec {
            if passed_bytes.chars().next().unwrap() != 'D' && passed_bytes.chars().next().unwrap() != 'U' {
                output_vec.push(u8::from_str_radix(passed_bytes, 16).unwrap());
            } else {
                let mut chars = passed_bytes.chars();
                chars.next();
                let reduced = chars.as_str();
                if passed_bytes.chars().next().unwrap() == 'U' {
                    if label_table.contains_key(&reduced.to_string()) {
                        let mut val: usize = *label_table.get(&reduced.to_string()).unwrap();
                        for _ in 0..4 {
                            output_vec.push((val & 0xff) as u8);
                            val = val >> 8;
                        }
                    } else {
                        format_errora(format!("Label '{}' does not exist.", reduced));
                        process::exit(1);
                    }
                }
            }
        }
        match file.write_all(&output_vec) {
            Err(why) => panic!("Couldn't write to file {}: {}", "out.bin", why),
            Ok(_) => ()
        }
        return output_vec.len();
    }

    pub fn lex_vml(self: &mut Lexer, file_data_pre: String) {
        let mut pass: bool = false;
        let mut variables: Vec<String> = Vec::new();
        let mut methods: Vec<String> = Vec::new();
        let mut line: usize = 0;
        let mut braces: i32 = 0;

        // manage includes
        let file_data: String = self.manage_includes(file_data_pre);

        for i in file_data.chars() {
            if i != '\n' && i != '\t' && i != ' ' { self.toks += &*format!("{}", i); }
            if i == '\n' {
                if self.toks != "".to_string() && self.lexer_state == 0 && !variables.contains(&self.toks) && !methods.contains(&self.toks) {
                    format_errorl("Syntax error".to_string(), line, self.read_line_num(&file_data, line));
                    process::exit(1);
                }
                if self.lexer_state == 1 {
                    format_errorl("Unexpected EOL while parsing string literal.".to_string(),
                                  line + 1,
                                  self.read_line_num(&file_data, line)
                    );
                    process::exit(1);
                }
                else if self.lexer_state == 512 {
                    format_errorl("Unexpected EOL while parsing character.".to_string(),
                                  line + 1,
                                  self.read_line_num(&file_data, line)
                    );
                    process::exit(1);
                }
                line += 1;
            }
            if self.lexer_state == (1 << 6) {
                if !pass {
                    if i != ' ' && i != '\t' && i != '\n' {
                        self.expr += &*format!("{}", i);
                    } else {
                        self.clear_state();
                        self.add_token(TokenType::VARIABLE_DECL, &*format!("{}", self.expr));
                        variables.push(format!("{}", self.expr));
                        if methods.contains(&self.expr) {
                            format_errorl(format!("Multiple definitions of '{}'.", self.expr),
                                line,
                                self.read_line_num(&file_data, line-1)
                            );
                            process::exit(1);
                        }
                    }
                } else {
                    pass = !pass;
                }
            }
            if self.lexer_state == (1 << 5) {
                if !pass {
                    if i != ' ' && i != '\t' && i != '\n' {
                        self.expr += &*format!("{}", i);
                    } else {
                        if methods.contains(&self.expr) {
                            format_errorl(format!("Repeated definition of method `{}`.", self.expr),
                                line, 
                                self.read_line_num(&file_data, line)
                            );
                            process::exit(1);
                        }
                        if self.expr == "".to_string() {
                            format_errorl(format!("`method` without name. (`method`s should be defined with `method` <name> {{...}})"),
                                line,
                                self.read_line_num(&file_data, line)
                            );
                            process::exit(1);
                        } else if self.expr == "{".to_string() { 
                            format_errorl(format!("`method` without name. (`method`s should be defined with `method` <name> {{...}})"),
                                line,
                                self.read_line_num(&file_data, line-1)
                            );
                            process::exit(1);
                        }
                        methods.push(format!("{}", self.expr));
                        self.clear_state();
                        self.add_token(TokenType::METHOD, &*format!("{}", self.expr));
                        if variables.contains(&self.expr) {
                            format_errorl(format!("Multiple definitions of '{}'.", self.expr),
                                line,
                                self.read_line_num(&file_data, line)
                            );
                            process::exit(1);
                        }
                    }
                } else {
                    pass = !pass;
                }
            }
            if self.lexer_state == 128 {
                if i == '/' {
                    self.clear_state();
                    self.set_comment_bit();
                } else {
                    self.add_token(TokenType::INSTRUCTION, "div");
                    if i != ' ' && i != '\n' && i != '\t' {
                        self.toks = String::from(i);
                    }
                    self.clear_state();
                }
            }

            if self.lexer_state == 1024 {
                if i.is_numeric() {
                    self.expr = String::from("-");
                    self.clear_state();
                    self.set_int_bit();
                } else {
                    self.add_token(TokenType::INSTRUCTION, "sub");
                    self.clear_state();
                }
            }

            if self.lexer_state == 0 {
                match &*self.toks {
                    ">" => self.add_token(TokenType::INSTRUCTION, ">"),
                    "<" => self.add_token(TokenType::INSTRUCTION, "<"),
                    "str=" => self.add_token(TokenType::INSTRUCTION, "strequals"),
                    "str!=" => self.add_token(TokenType::INSTRUCTION, "strnequals"),
                    "if" => self.add_token(TokenType::INSTRUCTION, "if"),
                    "and" => self.add_token(TokenType::INSTRUCTION, "and"),
                    "or" => self.add_token(TokenType::INSTRUCTION, "or"),
                    "while" => self.add_token(TokenType::INSTRUCTION, "while"),
                    "not" => self.add_token(TokenType::INSTRUCTION, "not"),
                    "=" => self.add_token(TokenType::INSTRUCTION, "equals"),
                    "!=" => self.add_token(TokenType::INSTRUCTION, "notequals"),
                    "root" => self.add_token(TokenType::INSTRUCTION, "root"),
                    "pow" => self.add_token(TokenType::INSTRUCTION, "pow"),
                    "return" => self.add_token(TokenType::INSTRUCTION, "return"),
                    "*" => self.add_token(TokenType::INSTRUCTION, "mul"),
                    "}" => {
                        braces -= 1;
                        self.add_token(TokenType::INSTRUCTION, "lbrace");
                    }
                    "{" => {
                        braces += 1;
                        self.add_token(TokenType::INSTRUCTION, "rbrace")
                    }
                    "-" => self.lexer_state = 1024,
                    "+" => self.add_token(TokenType::INSTRUCTION, "add"),
                    "/" => self.lexer_state = 128,
                    "d*" => self.add_token(TokenType::INSTRUCTION, "dmul"),
                    "d-" => self.add_token(TokenType::INSTRUCTION, "dsub"),
                    "d+" => self.add_token(TokenType::INSTRUCTION, "dadd"),
                    "d/" => self.add_token(TokenType::INSTRUCTION, "ddiv"),
                    "d>" => self.add_token(TokenType::INSTRUCTION, "d>"),
                    "d<" => self.add_token(TokenType::INSTRUCTION, "d<"),
                    "d=" => self.add_token(TokenType::INSTRUCTION, "d="),
                    "d!=" => self.add_token(TokenType::INSTRUCTION, "d!="),
                    "(int)" => self.add_token(TokenType::INSTRUCTION, "cast_i64"),
                    "(float)" => self.add_token(TokenType::INSTRUCTION, "cast_f64"),
                    "dup" => self.add_token(TokenType::INSTRUCTION, "dup"),
                    "swap" => self.add_token(TokenType::INSTRUCTION, "swap"),
                    "drop" => self.add_token(TokenType::INSTRUCTION, "drop"),
                    "rot" => self.add_token(TokenType::INSTRUCTION, "rot"),
                    "memory" => self.add_token(TokenType::INSTRUCTION, "mem"),
                    "syscall" => self.add_token(TokenType::INSTRUCTION, "syscall"),
                    "let" => self.add_token(TokenType::INSTRUCTION, "let"),
                    "const" => { self.set_variable_bit(); self.expr = String::from(""); pass = true; }
                    "\"" => { self.set_string_bit(); self.expr = String::from(""); }
                    "method" => {
                        self.set_method_bit(); self.expr = String::from(""); pass = true;
                    }
                    "!32" => self.add_token(TokenType::INSTRUCTION, "store32"),
                    "!64" => self.add_token(TokenType::INSTRUCTION, "store64"),
                    "!16" => self.add_token(TokenType::INSTRUCTION, "store16"),
                    "!8" => self.add_token(TokenType::INSTRUCTION, "store8"),
                    "@32" => self.add_token(TokenType::INSTRUCTION, "load32"),
                    "@64" => self.add_token(TokenType::INSTRUCTION, "load64"),
                    "@16" => self.add_token(TokenType::INSTRUCTION, "load16"),
                    "@8" => self.add_token(TokenType::INSTRUCTION, "load8"),
                    //"input" => self.add_token(TokenType::INSTRUCTION, "input"),
                    "copy" => self.add_token(TokenType::INSTRUCTION, "copy"),
                    "'" => { self.lexer_state = 512; self.expr = String::from(""); },
                    _ => {
                        if i == ' ' || i == '\n' {
                            if variables.contains(&self.toks) {
                                self.add_token(TokenType::VARIABLE, &*self.toks.clone());
                                self.toks = String::from("");
                            }
                            if methods.contains(&self.toks) {
                                self.add_token(TokenType::LABEL, &*self.toks.clone());
                                self.toks = String::from("");
                            }
                        }
                    }
                }
            }

            if self.lexer_state == 1 {
                if i != '"' || self.expr == String::from("")  {
                    if i != '"' { self.expr += &*format!("{}", i); }
                } else {
                    self.clear_state();
                    self.add_token(TokenType::STRING, &*self.expr.clone());
                }
            }
            
            if self.lexer_state == 512 {
                if i != '\'' || self.expr == String::from("")  {
                    if i != '\'' { self.expr += &*format!("{}", i); }
                } else {
                    if self.expr.len() > 1 {
                        format_errorl("Type `char` must contain only 1 character within its type.".to_string(),
                            line,
                            self.read_line_num(&file_data, line)
                        );
                        process::exit(1);
                    }
                    self.clear_state();
                    self.add_token(TokenType::CHAR, &*self.expr.clone());
                }
            }

            if self.lexer_state == 8 {
                if i != '"' || self.expr == String::from("")  {
                    if i != '"' { self.expr += &*format!("{}", i); }
                } else {
                    self.clear_state();
                    self.add_token(TokenType::VARIABLE, &*self.expr.clone());
                }
            }

            if self.lexer_state == (1 << 4) {
                if i == '\n' {
                    self.clear_state();
                    self.toks = String::from("");
                }
            }

            if self.toks.parse::<u64>().is_ok() && self.lexer_state == 0 {
                self.expr = String::from(i);
                self.set_int_bit();
            } else if self.lexer_state == 2 && i != '\n' && i != '\t' && i != ' ' {
                self.expr += &*format!("{}", i);
            } else if self.lexer_state == 2 {
                if self.expr.contains(".") {
                    let result = self.expr.parse::<f64>();
                    if !result.is_ok() {
                        format_errorl("Unexpected character while parsing integer.".to_string(),
                            line,
                            self.read_line_num(&file_data, line-1)
                        );
                        process::exit(1);
                    }
                    self.add_token(TokenType::DOUBLE, &*self.expr.clone());
                    self.clear_state();
                } else {
                    let result = self.expr.parse::<u64>();
                    if !result.is_ok() {
                        let result = self.expr.parse::<i64>();
                        if !result.is_ok() {
                            format_errorl("Unexpected character while parsing integer.".to_string(),
                                line,
                                self.read_line_num(&file_data, line-1)
                            );
                            process::exit(1);
                        } else {
                            self.add_token(TokenType::INTEGER, &*i64_bits(result.unwrap()).to_string());
                            self.clear_state();
                        }
                    } else {
                        self.add_token(TokenType::INTEGER, &*self.expr.clone());
                        self.clear_state();
                    }
                }
            }
        }
        if !methods.contains(&("main".to_string())) {
            format_errora("File does not contain `main` method. Exiting.".to_string());
            process::exit(1);
        }
        if braces != 0 {
            format_errora("Imbalanced braces found!".to_string());
            process::exit(1);
        }
    }

    pub fn tokens_to_assembly(self: &mut Lexer) -> String {
        let mut output: String = String::from("; generated by VML compiler v0.0.0a\n\n.start:\n\t\tjmp \t.end\n");
        let mut index: usize = 0;
        let mut stringmap: HashMap::<String, String> = HashMap::new();
        let mut loopmap_str: Vec<String> = Vec::new();
        let mut loopmap_type: Vec<u8> = Vec::new();
        let mut varlist: Vec<Variable> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        let mut stringindex: usize = 0;
        let mut loopindex: usize = 0;
        let mut labelindex: usize = 0;
        let mut memalloc: usize = 0;

        while index < self.tokens.len() {
            match &self.tokens[index].token_t {
                TokenType::INTEGER => {
                    let result = self.tokens[index].data.parse::<u64>();
                    output += &*format!("\t\tmov \tr0, $0x{:x}\n", result.unwrap());
                    output += "\t\tpush\tr0\n";
                },
                TokenType::INSTRUCTION => {
                    match &*self.tokens[index].data {
                        "and" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpop \tr0\n";
                            output += "\t\tand\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "or" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpop \tr0\n";
                            output += "\t\tor\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "not" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tneg\tr0\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "syscall" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tcall\tr0\n";
                        },
                        "return" => output += "\t\tret\n",
                        "pow" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpow \tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "root" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\troot \tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "cast_f64" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\ticst\tr0\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "cast_i64" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tdcst\tr0\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "add" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tiadd\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "sub" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpop \tr0\n";
                            output += "\t\tisub\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "mul" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\timul\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "div" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpop \tr0\n";
                            output += "\t\tidiv\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "dadd" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tdadd\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "dsub" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpop \tr0\n";
                            output += "\t\tdsub\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "dmul" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tdmul\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "ddiv" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpop \tr0\n";
                            output += "\t\tddiv\tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "dup" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpush\tr0\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "swap" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpush\tr0\n";
                            output += "\t\tpush\tr1\n";
                        },
                        ">" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\ticmp\tr0, r1\n";
                            output += &*format!("\t\tblt \t.label{}\n", labelindex);
                            output += "\t\tmov \tr0, $0x00\n";
                            output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                            output += &*format!(".label{}:\n", labelindex);
                            output += "\t\tmov \tr0, $0x01\n";
                            output += &*format!(".label_join{}:\n", labelindex);
                            output += "\t\tpush\tr0\n";
                            labelindex += 1;
                        },
                        "<" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\ticmp\tr0, r1\n";
                            output += &*format!("\t\tbgt \t.label{}\n", labelindex);
                            output += "\t\tmov \tr0, $0x00\n";
                            output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                            output += &*format!(".label{}:\n", labelindex);
                            output += "\t\tmov \tr0, $0x01\n";
                            output += &*format!(".label_join{}:\n", labelindex);
                            output += "\t\tpush\tr0\n";
                            labelindex += 1;
                        },
                        "d>" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tdcmp\tr0, r1\n";
                            output += &*format!("\t\tblt \t.label{}\n", labelindex);
                            output += "\t\tmov \tr0, $0x00\n";
                            output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                            output += &*format!(".label{}:\n", labelindex);
                            output += "\t\tmov \tr0, $0x01\n";
                            output += &*format!(".label_join{}:\n", labelindex);
                            output += "\t\tpush\tr0\n";
                            labelindex += 1;
                        },
                        "d<" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tdcmp\tr0, r1\n";
                            output += &*format!("\t\tbgt \t.label{}\n", labelindex);
                            output += "\t\tmov \tr0, $0x00\n";
                            output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                            output += &*format!(".label{}:\n", labelindex);
                            output += "\t\tmov \tr0, $0x01\n";
                            output += &*format!(".label_join{}:\n", labelindex);
                            output += "\t\tpush\tr0\n";
                            labelindex += 1;
                        },
                        "strequals" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tlseq\tr0, r1\n"; 
                            output += &*format!("\t\tbeq \t.label{}\n", labelindex);
                            output += "\t\tmov \tr0, $0x00\n";
                            output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                            output += &*format!(".label{}:\n", labelindex);
                            output += "\t\tmov \tr0, $0x01\n";
                            output += &*format!(".label_join{}:\n", labelindex);
                            output += "\t\tpush\tr0\n";
                        },
                        "strnequals" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tlseq\tr0, r1\n"; 
                            output += &*format!("\t\tbne \t.label{}\n", labelindex);
                            output += "\t\tmov \tr0, $0x00\n";
                            output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                            output += &*format!(".label{}:\n", labelindex);
                            output += "\t\tmov \tr0, $0x01\n";
                            output += &*format!(".label_join{}:\n", labelindex);
                            output += "\t\tpush\tr0\n";
                        },
                        "equals" => {
                            if index >= 2 {
                                output += "\t\tpop \tr0\n";
                                output += "\t\tpop \tr1\n";
                                output += "\t\ticmp\tr0, r1\n"; 
                                output += &*format!("\t\tbeq \t.label{}\n", labelindex);
                                output += "\t\tmov \tr0, $0x00\n";
                                output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                                output += &*format!(".label{}:\n", labelindex);
                                output += "\t\tmov \tr0, $0x01\n";
                                output += &*format!(".label_join{}:\n", labelindex);
                                output += "\t\tpush\tr0\n";
                                labelindex += 1;
                            } else {
                                format_errora("`=` expects two prior arguments. (eg. `1 1 =`).".to_string());
                                process::exit(1);
                            }
                        }
                        "notequals" => {
                            if index >= 2 {
                                output += "\t\tpop \tr0\n";
                                output += "\t\tpop \tr1\n";
                                output += "\t\ticmp\tr0, r1\n"; 
                                output += &*format!("\t\tbne \t.label{}\n", labelindex);
                                output += "\t\tmov \tr0, $0x00\n";
                                output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                                output += &*format!(".label{}:\n", labelindex);
                                output += "\t\tmov \tr0, $0x01\n";
                                output += &*format!(".label_join{}:\n", labelindex);
                                output += "\t\tpush\tr0\n";
                                labelindex += 1;
                            }
                            else {
                                format_errora("`!=` expects two prior arguments. (eg. `1 2 !=`).".to_string());
                                process::exit(1);
                            }
                            labelindex += 1;
                        }
                        "d=" => {
                            if index >= 2 {
                                output += "\t\tpop \tr0\n";
                                output += "\t\tpop \tr1\n";
                                output += "\t\tdcmp\tr0, r1\n"; 
                                output += &*format!("\t\tbeq \t.label{}\n", labelindex);
                                output += "\t\tmov \tr0, $0x00\n";
                                output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                                output += &*format!(".label{}:\n", labelindex);
                                output += "\t\tmov \tr0, $0x01\n";
                                output += &*format!(".label_join{}:\n", labelindex);
                                output += "\t\tpush\tr0\n";
                                labelindex += 1;
                            } else {
                                format_errora("`=` expects two prior arguments. (eg. `1 1 =`).".to_string());
                                process::exit(1);
                            }
                        }
                        "d!=" => {
                            if index >= 2 {
                                output += "\t\tpop \tr0\n";
                                output += "\t\tpop \tr1\n";
                                output += "\t\tdcmp\tr0, r1\n"; 
                                output += &*format!("\t\tbne \t.label{}\n", labelindex);
                                output += "\t\tmov \tr0, $0x00\n";
                                output += &*format!("\t\tjmp \t.label_join{}\n", labelindex);
                                output += &*format!(".label{}:\n", labelindex);
                                output += "\t\tmov \tr0, $0x01\n";
                                output += &*format!(".label_join{}:\n", labelindex);
                                output += "\t\tpush\tr0\n";
                                labelindex += 1;
                            }
                            else {
                                format_errora("`!=` expects two prior arguments. (eg. `1 2 !=`).".to_string());
                                process::exit(1);
                            }
                            labelindex += 1;
                        }
                        "rot" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tpop \tr2\n";
                            output += "\t\tpush\tr1\n";
                            output += "\t\tpush\tr0\n";
                            output += "\t\tpush\tr2\n";
                        },
                        "while" => {
                            loopmap_str.push(format!(".loop{}", loopindex));
                            loopmap_type.push(0);
                            output += &*format!(".loop{}:", loopindex);
                            loopindex += 1;
                        },
                        "if" => {
                            loopmap_str.push(format!(".loop{}", loopindex));
                            loopmap_type.push(1);
                            loopindex += 1;
                        },
                        "drop" => {
                            output += "\t\tpop \tr0\n";
                        },
                        "rbrace" => {
                            if loopmap_str.len() > 0 {
                                let last_entry: String = format!("{}", loopmap_str[loopmap_str.len()-1]);
                                if loopmap_type[loopmap_type.len()-1] == 0 || loopmap_type[loopmap_type.len()-1] == 1 {
                                    output += "\t\tpop \tr0\n";
                                    output += "\t\tmov \tr1, $0x01\n";
                                    output += "\t\ticmp\tr0, r1\n";
                                    output += &*format!("\t\tbne \t.E_{}\n", last_entry);
                                }
                            }
                        },
                        "lbrace" => {
                            if loopmap_str.len() > 0 {
                                let last_entry: String = format!("{}", loopmap_str[loopmap_str.len()-1]);
                                if loopmap_type[loopmap_type.len()-1] == 0 {
                                    output += &*format!("\t\tjmp \t{}\n", last_entry);
                                    output += &*format!(".E_{}:\n", last_entry);
                                    output += "\t\tpop \tr0\n";
                                    loopmap_str.pop();
                                    loopmap_type.pop();
                                } else {
                                    output += &*format!(".E_{}:\n", last_entry);
                                    loopmap_str.pop();
                                    loopmap_type.pop();
                                }
                            } else {
                                if labels.len() > 1 {
                                    format_errora("Nested methods are not supported; please place different methods in the global scope.".to_string());
                                    process::exit(1);
                                }
                                let x = labels.pop().unwrap();
                                if x != String::from("main\n") { output += "\t\tret\n"; }
                            }
                        },
                        "store64" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tssf \tr0, r1\n";
                        },
                        "store32" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tstt \tr0, r1\n";
                        },
                        "store16" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tsst \tr0, r1\n";
                        },
                        "store8" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tsei \tr0, r1\n";
                        },
                        "load64" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tlsf \tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "load32" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tltt \tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "load16" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tlst \tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "load8" => {
                            output += "\t\tpop \tr1\n";
                            output += "\t\tlei \tr0, r1\n";
                            output += "\t\tpush\tr0\n";
                        },
                        "copy" => {
                            output += "\t\tpop \tr0\n";
                            output += "\t\tpop \tr1\n";
                            output += "\t\tbufc\tr0, r1\n";
                        },
                        "mem" => {
                            if index + 2 < self.tokens.len() {
                                if self.tokens[index + 1].token_t == TokenType::INTEGER && self.tokens[index + 2].token_t == TokenType::VARIABLE_DECL {
                                    varlist.push(Variable::new(
                                            self.tokens[index + 2].data.clone(),
                                            memalloc.to_string(),
                                            0
                                        )
                                    );
                                    memalloc += self.tokens[index + 1].data.parse::<usize>().unwrap();
                                    index += 2;
                                } else {
                                    format_errora("`memory` declaration incomplete/malformed. `memory` declarations must take the form `memory <size> const <name>`.".to_string());
                                    process::exit(1);
                                }
                            } else {
                                format_errora("`memory` must be followed by a declaration (eg `memory 64 const buffer`)".to_string());
                                process::exit(1);
                            }
                        },
                        "let" => {
                            if index + 2 < self.tokens.len() {
                                if self.tokens[index + 2].token_t == TokenType::VARIABLE_DECL {
                                    if self.tokens[index + 1].token_t == TokenType::CHAR { 
                                        varlist.push(Variable::new(
                                            self.tokens[index + 2].data.clone(),
                                            self.tokens[index + 1].data.clone(),
                                            3
                                        ));
                                        index += 2;
                                    }
                                    else if self.tokens[index + 1].token_t == TokenType::STRING {
                                        varlist.push(Variable::new(
                                            self.tokens[index + 2].data.clone(),
                                            self.tokens[index + 1].data.clone(),
                                            1
                                        ));
                                        index += 2;
                                    }
                                    else if self.tokens[index + 1].token_t == TokenType::INTEGER {
                                        varlist.push(Variable::new(
                                            self.tokens[index + 2].data.clone(),
                                            self.tokens[index + 1].data.clone(),
                                            2
                                        ));
                                        index += 2;
                                    }
                                    else if self.tokens[index + 1].token_t == TokenType::DOUBLE {
                                        varlist.push(Variable::new(
                                            self.tokens[index + 2].data.clone(),
                                            self.tokens[index + 1].data.clone(),
                                            4
                                        ));
                                        index += 2;
                                    }
                                    else {
                                        format_errora("Unexpected token whilst parsing `let` binding.".to_string());
                                        process::exit(1);
                                    }
                                }
                            }
                        },
                        _ => warninga("Unimplemented instruction type encountered!")
                    }
                },
                TokenType::STRING => {
                    output += &*format!("\t\tadr \tr0, .str{}\n", stringindex);
                    output += "\t\tpush\tr0\n";
                    stringmap.insert(format!(".str{}", stringindex), self.tokens[index].data.clone());
                    stringindex += 1;
                },
                TokenType::METHOD => {
                    if self.tokens[index].data == "".to_string() {
                        format_errora("Error while parsing - `method` without name found.".to_string());
                        process::exit(1);
                    }
                    output += &*format!(".{}:\n", self.tokens[index].data);
                    labels.push(format!("{}\n", self.tokens[index].data));
                },
                TokenType::LABEL => {
                    output += &*format!("\t\tjsr \t.{}\n", self.tokens[index].data);
                },
                TokenType::CHAR => {
                    output += &*format!("\t\tadr \tr0, 0x{:x}\n", self.tokens[index].data.as_bytes()[0]);
                    output += "\t\tpush\tr0\n";
                },
                TokenType::DOUBLE => {
                    let as_u64_val: u64 = to_u64(self.tokens[index].data.parse::<f64>().unwrap());
                    output += &*format!("\t\tmov \tr0, $0x{:x}\n", as_u64_val);
                    output += "\t\tpush\tr0\n";
                }
                TokenType::VARIABLE => {
                    // variable checking not required as it is done in the
                    // first step

                    for var in varlist.iter() {
                        if var.variable_name == self.tokens[index].data {
                            match &var.variable_type {
                                0 => {
                                    output += &*format!("\t\tadr \tr0, 0x{:x}\n", var.variable_data.parse::<usize>().unwrap());
                                    output += "\t\tpush\tr0\n";
                                },
                                1 => {
                                    output += &*format!("\t\tadr \tr0, .{}\n", var.variable_name);
                                    output += "\t\tpush\tr0\n";
                                },
                                2 => {
                                    output += &*format!("\t\tmov \tr0, $0x{:x}\n", var.variable_data.parse::<u64>().unwrap());
                                    output += "\t\tpush\tr0\n";
                                },
                                3 => {
                                    output += &*format!("\t\tadr \tr0, 0x{:x}\n", var.variable_data.as_bytes()[0]);
                                    output += "\t\tpush\tr0\n";
                                },
                                4 => {
                                    output += &*format!("\t\tmov \tr0, $0x{:x}\n", to_u64(var.variable_data.parse::<f64>().unwrap()));
                                    output += "\t\tpush\tr0\n";
                                }
                                _ => warninga("Unspecified variable type encountered while parsing.")
                            }
                        }
                    }
                },
                _ => warninga("Unknown token found while parsing")
            }
            index += 1;
        }
        // be sure to jump to the end of the file in order to avoid executing the program's
        // data as code.

        output += "\t\tret\n";

        // add strings
        
        for (k, v) in stringmap {
            output += &*format!("{}: \"{}\"\n", k, v);
        }

        for var in varlist.iter() {
            if var.variable_type == 1 {
                output += &*format!(".{}: \"{}\"\n", var.variable_name, var.variable_data);
            }
        }
        output += ".end: jsr .main\n";
        self.tokens = Vec::new();
        return output;
    }

    pub fn print_toks(self: &mut Lexer) {
        for i in &self.tokens {
            println!("{:?}: {}", i.token_t, i.data);
        }
    }
}
