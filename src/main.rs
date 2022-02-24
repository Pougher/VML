use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Read;

pub mod errors;
pub mod vml_cpu;
pub mod assembler;
pub mod token;
pub mod variable;

use crate::assembler::*;

static VERSION: &str = "0.0.0a *ALPHA BUILD*";

#[derive(PartialEq)]
enum RunType {
    COMPILE,
    RUN,
    ASSEMBLE,
    NONE
}

fn load_binary_file(filename: &String) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let meta = fs::metadata(&filename).expect(&*format!("Unable to read metadata of file '{}'.", filename));
    let mut buff = vec![0; meta.len() as usize];
    file.read(&mut buff).expect(&*format!("Buffer overflow on file {}.", filename));

    return buff;
}

fn load_text_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename).expect(&*format!("Unable to read the file '{}'.", filename)); 
    return contents;
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut runtype: RunType = RunType::NONE;
    let mut filename: String = String::new();

    if args.len() == 1 {
        eprintln!("{}", errors::err_no_args());
        process::exit(1);
    }
    
    args.remove(0);
    for i in args {
        if runtype == RunType::NONE {
            match &*i {
                "-c" => runtype = RunType::COMPILE,
                "-r" => runtype = RunType::RUN,
                "-a" => runtype = RunType:: ASSEMBLE,
                _ => {
                    eprintln!("{}", errors::err_arg_not_found());
                    process::exit(1);
                },
            }
        } else {
            filename = i;
        }
    }

    match &runtype {
        RunType::COMPILE => { 
            let contents: String = load_text_file(&filename);
            let mut lexer: Lexer = Lexer::new();
            lexer.lex_vml(contents);
            let assembly: String = lexer.tokens_to_assembly();
            lexer.lex_asm(assembly);
            let filesize: usize = lexer.assemble_asm();
            println!("Finished compilation: {:.2}KB (ALL OK).", (filesize as f64) / 1024.0);
        },
        RunType::RUN => {
            let file_data: Vec<u8> = load_binary_file(&filename);
            let mut vm_cpu: vml_cpu::VMLCpu = vml_cpu::VMLCpu::new();
            vm_cpu.exec(&file_data, &file_data.len());
        },
        RunType::ASSEMBLE => {
            println!("VML Global Assembler (C) AxolotifiedC");
            println!("ver {} [+0 commits]\n", VERSION);
            let contents: String = load_text_file(&filename);

            let mut lexer: Lexer = Lexer::new();
            lexer.lex_asm(contents);
            let filesize: usize = lexer.assemble_asm();
            println!("Finished compilation: {:.2}KB (ALL OK).", (filesize as f64) / 1024.0);
        },
        _ => {},
    }
}
