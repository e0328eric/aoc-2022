// Copyright (c) 2022 Sungbae Jeong
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR must be set"));
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));
    let nasm = get_program_location("nasm")?;
    let gcc = get_program_location("gcc")?;

    let nasm_output = Command::new(nasm)
        .arg(manifest_dir.join("src/main.asm"))
        .arg("-fmacho64")
        .arg("-o")
        .arg(output_dir.join("main.o"))
        .output()?;
    if !nasm_output.status.success() {
        panic!("{:?}", String::from_utf8(nasm_output.stderr));
    }

    if let Ok(_) = fs::File::open(output_dir.join("../../../libaoc_2022.a")) {
        let gcc_output = Command::new(gcc)
            .arg(output_dir.join("main.o"))
            .arg(output_dir.join("../../../libaoc_2022.a"))
            .arg("-o")
            .arg(manifest_dir.join("solution"))
            .output()?;
        if !gcc_output.status.success() {
            panic!("{:?}", String::from_utf8(gcc_output.stderr));
        }
    }

    Ok(())
}

fn get_program_location(prog_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("which").arg(prog_name).output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
