pub fn err_arg_not_found() -> &'static str {
    return "ERROR::CMD_ARG_NOT_FOUND:\n\tUSAGE: vml [-C/-R/-A] [FILENAME]";
}

pub fn err_no_args() -> &'static str {
    return "ERROR::NO_ARGS:\n\tUSAGE: vml [-C/-R/-A] [FILENAME]";
}

pub fn format_errorl(error: String, line: usize, error_block: String) {
    let mut tildes: String = String::new();
    for _ in 0..error_block.len()-1 {
        tildes += "~";
    }
    tildes += "^";
    println!("\x1b[31m\x1b[4mError on line {}: {}\x1b[0m", line, error);
    println!("\t{}", error_block);
    println!("\x1b[93m\x1b[1mHere ->\t\x1b[21m\x1b[94m{}\x1b[0m", tildes);
}

pub fn format_errora(error: String) {
    println!("\x1b[31m\x1b[4mError while parsing: {}\x1b[0m", error);
}

pub fn warninga(error: &str) {
    println!("\x1b[33m\x1b[4mWarning: {}\x1b[0m", error);
}
