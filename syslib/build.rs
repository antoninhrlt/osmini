// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::fs;
use std::process::Command;

static ASM: &str = "nasm";

fn main() {
    let out_dir = "../target/x64osmini/debug/";
    let asm_files = ["drivers/cursor"];

    for asm_file in asm_files {
        match fs::create_dir(format!("{}syslib.drivers", out_dir)) {
            _ => {}
        }

        // Compile the bootloader to an object file
        call(ASM, 
            &[
                format!("-o {}syslib.{}.o", out_dir, asm_file).as_str(),
                "-f", "elf64",
                "-I", "src/",
                format!("src/{}.asm", asm_file).as_str(),
            ]
        );
    }
}

fn call(program: &str, args: &[&str]) {
    if !Command::new(program).args(args).status().unwrap().success() {
        panic!("Execution of {}", program);
    }
}
