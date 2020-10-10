mod r#enum;
mod function;
mod json;
mod name;
mod parser;
mod render;
mod r#struct;
mod r#type;

use anyhow::Result;
use clap::Clap;
use glob::glob;
use std::{
    fs::{self, File},
    io::prelude::*,
};

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "output")]
    output_directory: String,
    #[clap(short, long, default_value = "cimgui")]
    cimgui_directory: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let mut parser = parser::Parser::new();

    // Read the definitions files
    for path in glob(&format!("{}/**/*definitions.json", opts.cimgui_directory))? {
        let mut file = File::open(path?)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        parser.add_json_definitions(&contents)?;
    }

    // Read the structs and enums files
    for path in glob(&format!(
        "{}/**/*structs_and_enums.json",
        opts.cimgui_directory
    ))? {
        let mut file = File::open(path?)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        parser.add_json_structs_and_enums(&contents)?;
    }

    // Read the typedefs files
    for path in glob(&format!("{}/**/*typedefs_dict.json", opts.cimgui_directory))? {
        let mut file = File::open(path?)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        parser.add_json_typedefs(&contents)?;
    }

    // Parse the data
    let data = parser.parse()?;

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&opts.output_directory)?;

    // Write the Lua to a file
    fs::write(format!("{}/gui.lua", &opts.output_directory), data.lua())?;

    // Write the cdefs to a file
    fs::write(
        format!("{}/cdefs.lua", &opts.output_directory),
        data.cdefs(),
    )?;

    Ok(())
}
