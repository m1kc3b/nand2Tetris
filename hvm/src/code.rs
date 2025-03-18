use crate::asm;
use crate::parser::{CommandType, MemorySegment};
use rand;
use std::io::Result;
// use crate::errors::HVMError;

static STATIC_COUNT: u16 = 16;

// Writes to the output file the assembly code that implements the given arithmetic-logical command.
pub fn write_arithmetic(command: &str) -> Result<String> {
    match command {
        "add" => Ok(format!("{}", asm::ADD)),
        "sub" => Ok(format!("{}", asm::SUB)),
        "neg" => Ok(format!("{}", asm::NEG)),
        "and" => Ok(format!("{}", asm::AND)),
        "or" => Ok(format!("{}", asm::OR)),
        "not" => Ok(format!("{}", asm::NOT)),
        "eq" => {
            let count1 = rand::random_range(1000..=99999);
            let count2 = rand::random_range(1000..=99999);
            Ok(format!(
                "// eq\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@COUNT_{count1}\nD;JEQ\nD=0\n@COUNT_{count2}\n0;JMP\n(COUNT_{count1})\nD=-1\n(COUNT_{count2})\n@SP\nA=M-1\nM=D\n\n"
            ))
        }
        "gt" => {
            let count1 = rand::random_range(1000..=99999);
            let count2 = rand::random_range(1000..=99999);
            Ok(format!(
                "// eq\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@COUNT_{count1}\nD;JGT\nD=0\n@COUNT_{count2}\n0;JMP\n(COUNT_{count1})\nD=-1\n(COUNT_{count2})\n@SP\nA=M-1\nM=D\n\n"
            ))
        }
        "lt" => {
            let count1 = rand::random_range(1000..=99999);
            let count2 = rand::random_range(1000..=99999);
            Ok(format!(
                "// eq\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@COUNT_{count1}\nD;JLT\nD=0\n@COUNT_{count2}\n0;JMP\n(COUNT_{count1})\nD=-1\n(COUNT_{count2})\n@SP\nA=M-1\nM=D\n\n"
            ))
        }
        _ => panic!("Unknow arithmetic command"),
    }
}

// Write to the output file the assembly code that implements the given push or pop command.
pub fn write_push_pop(command: &CommandType, filename: &str) -> Result<String> {
    match command {
        CommandType::Push(segment, index) => {
            let code = match segment {
                MemorySegment::Argument => write_push(index, "ARG", "argument"),
                MemorySegment::Local => write_push(index, "LCL", "local"),
                MemorySegment::Temp => write_push(index, "TEMP", "temp"),
                MemorySegment::That => write_push(index, "THAT", "that"),
                MemorySegment::This => write_push(index, "THIS", "this"),
                MemorySegment::Static => write_push_static(filename, index),
                MemorySegment::Constant => write_push_constant(index),
                MemorySegment::Pointer => write_push_pointer(index),
            };
            return Ok(code);
        }
        CommandType::Pop(segment, index) => {
            let code = match segment {
                MemorySegment::Argument => write_pop(index, "ARG", "argument"),
                MemorySegment::Local => write_pop(index, "LCL", "local"),
                MemorySegment::Temp => write_pop(index, "TEMP", "temp"),
                MemorySegment::That => write_pop(index, "THAT", "that"),
                MemorySegment::This => write_pop(index, "THIS", "this"),
                MemorySegment::Static => write_pop_static(filename, index),
                MemorySegment::Pointer => write_pop_pointer(index),
                _ => panic!("Segment unknow!"),
            };
            return Ok(code);
        }
        // _ => Err(HVMError::UnknownCommand("Unknow push/pop command")),
        _ => panic!("Unknow push/pop command"),
    }
}

// Writes assembly code that effects the label command
pub fn write_label(label: &str) -> Result<String> {
    Ok(format!("({})\n", label))
}

// Writes assembly code that effects the goto command
pub fn write_goto(label: &str) -> Result<String> {
    Ok(format!("@{}\n0;JMP\n\n", label))
}

// Writes assembly code that effects the if-goto command
pub fn write_if(label: &str) -> Result<String> {
    Ok(format!("// if-goto{}\n@{}\nD;JGT\n\n", asm::POP_Y, label))
}

// Writes assembly code that effects the function command
pub fn write_function(function_name: &str, nvars: u8) -> Result<String> {
    // add label
    let label = write_label(function_name)?;
    // init nvars local at 0
    let mut local_vars_init = String::new();
    local_vars_init.push_str("@1\nA=M\nM=0\n");
    for _ in 1..nvars.clone() {
        let init_local = format!("@1\nA=M+1\nM=0\n");
        local_vars_init.push_str(&init_local);
    }
    Ok(format!("{}{}\n", label, local_vars_init))
}

// TODO: test it
// Writes assembly code that effects the call command
pub fn write_call(function_name: &str, nargs: u8) -> Result<String> {
    let return_label = format!("{}$ret{}", function_name, rand::random::<u8>());

    let save_return_address = format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", return_label);

    let save_lcl = "@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    let save_arg = "@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    let save_this = "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    let save_that = "@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";

    let reposition_arg = format!("@SP\nD=M\n@{}\nD=D-A\n@ARG\nM=D\n", nargs as i16 + 5);

    let reposition_lcl = "@SP\nD=M\n@LCL\nM=D\n";

    let goto_function = format!("@{}\n0;JMP\n", function_name);

    let define_return_label = format!("({})\n", return_label);

    Ok(format!(
        "// call {}\n{}{}{}{}{}{}{}{}{}\n\n",
        function_name,
        save_return_address,
        save_lcl,
        save_arg,
        save_this,
        save_that,
        reposition_arg,
        reposition_lcl,
        goto_function,
        define_return_label
    ))
}

// Writes assembly code that effects the return command
pub fn write_return() -> Result<String> {
    // Save LCL as `frame`
    let save_frame = "@LCL\nD=M\n@R13\nM=D\n";

    // Save retAddr (frame - 5) dans R14
    let save_ret_addr = "@5\nA=D-A\nD=M\n@R14\nM=D\n";
    
    // ARG[0] = return value
    let get_last_values = "@SP\nA=M-1\nD=M\n";
    let updates_arg0 = "@ARG\nA=M\nM=D\n";

    // SP = ARG + 1
    let updates_sp = "@ARG\nD=M+1\n@SP\nM=D\n";

    // THAT = *(frame - 1)
    let updates_that = "@R13\nD=M-1\nA=D\nD=M\n@THAT\nM=D\n";

    // THIS = *(frame - 2)
    let updates_this = "@R13\nD=M\n@2\nA=D-A\nD=M\n@THIS\nM=D\n";

    // ARG = *(frame - 3)
    let updates_arg = "@R13\nD=M\n@3\nA=D-A\nD=M\n@ARG\nM=D\n";

    // LCL = *(frame - 4)
    let updates_lcl = "@R13\nD=M\n@4\nA=D-A\nD=M\n@LCL\nM=D\n";

    // Jump to retAdr
    let jump_to_ret = "@R14\nA=M\n0;JMP\n";

    Ok(format!(
        "// return\n{}{}{}{}{}{}{}{}{}{}",
        save_frame,
        save_ret_addr,
        get_last_values,
        updates_arg0,
        updates_sp,
        updates_that,
        updates_this,
        updates_arg,
        updates_lcl,
        jump_to_ret
    ))
}

// Generates bootstarp code & call Sys.init
pub fn write_bootstrap() -> Result<String> {
    let init_sp = "@256\nD=A\n@SP\nM=D\n"; // SP = 256
    let call_sys_init = write_call("Sys.init", 0)?; // Call Sys.init()

    Ok(format!(
        "// Bootstrap Code\n{}{}",
        init_sp, call_sys_init
    ))
}

/*

HELPERS

*/
// Generic function to write push ARG, LCL, TEMP, THIS, THAT command
fn write_push(index: &u16, label: &str, segment: &str) -> String {
    if label == "TEMP" {
        let count = index + 5;
        format!(
            "// push {segment} {index}\n@{index}\nD=A\n@{count}\nD=M{}\n\n",
            asm::PUSH_X
        )
    } else {
        format!(
            "// push {segment} {index}\n@{index}\nD=A\n@{label}\nA=D+M\nD=M{}\n\n",
            asm::PUSH_X
        )
    }
}

// Writes push STATIC command
fn write_push_static(filename: &str, index: &u16) -> String {
    let count = STATIC_COUNT + index;
    format!(
        "// push static {index}\n@{index}\nD=A\n@{filename}.{index}\n@{count}\nD=M{}\n\n",
        asm::PUSH_X
    )
}

// Writes push CONSTANT command
fn write_push_constant(index: &u16) -> String {
    format!("// push constant {index}\n@{index}\nD=A{}\n\n", asm::PUSH_X)
}

// Writes push POINTER command
fn write_push_pointer(index: &u16) -> String {
    if *index == 0 {
        format!("// push pointer {index}\n@THIS\nD=M{}\n\n", asm::PUSH_X)
    } else {
        format!("// push pointer {index}\n@THAT\nD=M{}\n\n", asm::PUSH_X)
    }
}

// Generic function to write pop ARG, LCL, TEMP, THIS, THAT command
fn write_pop(index: &u16, label: &str, segment: &str) -> String {
    let increment = incremente_m(*index);
    if label == "TEMP" {
        let count = index + 5;
        return format!("// pop {segment} {index}{}\n@{count}\nM=D\n\n", asm::POP_Y);
    } else {
        return format!(
            "// pop {segment} {index}{}\n@{label}\n{}M=D\n\n",
            asm::POP_Y,
            increment
        );
    }
}

// Writes pop STATIC command
fn write_pop_static(filename: &str, index: &u16) -> String {
    let count = STATIC_COUNT + index;
    format!(
        "// pop static {index}{}\n@{filename}.{index}\n@{count}\nM=D\n\n",
        asm::POP_Y
    )
}

// Writes pop POINTER command
fn write_pop_pointer(index: &u16) -> String {
    if *index == 0 {
        return format!("// pop pointer {index}{}\n@THIS\nM=D\n\n", asm::POP_Y);
    } else {
        return format!("// pop pointer {index}{}\n@THAT\nM=D\n\n", asm::POP_Y);
    }
}

// Get the memory segment
fn incremente_m(index: u16) -> String {
    let mut increment = String::new();

    match index {
        0 => increment.push_str("A=M\n"),
        1 => increment.push_str("A=M+1\n"),
        2..=u16::MAX => {
            increment.push_str("A=M+1\n");
            for _ in 1..index {
                increment.push_str("A=A+1\n")
            }
        }
    }

    increment
}
