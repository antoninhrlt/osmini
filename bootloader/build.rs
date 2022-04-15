// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::env;
use std::process::Command;

static ASM: &str = "nasm";

fn main() {
    let out_dir = "../target/debug";
    let bootloader: &str = "src/boot.asm";
    let bootloader_out: String = format!("{}/bootloader.o", out_dir);

    // Compile the bootloader to an object file
    call(ASM, 
        &[
            "-o", bootloader_out.as_str(),
            "-I", "src/",
            bootloader,
        ]
    );
}

fn call(program: &str, args: &[&str]) {
    if !Command::new(program).args(args).status().unwrap().success() {
        panic!("Execution of {}", program);
    }
}
