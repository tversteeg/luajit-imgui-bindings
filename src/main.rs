mod r#enum;
mod json;
mod parser;
mod r#struct;
mod r#type;

use anyhow::Result;
use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "output")]
    output_directory: String,
    #[clap(short, long, default_value = "cimgui")]
    cimgui_directory: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let parser = parser::Parser::new();

    Ok(())
}
