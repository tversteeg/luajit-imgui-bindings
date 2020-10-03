mod json;

use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "output")]
    output_directory: String,
    #[clap(short, long, default_value = "cimgui")]
    cimgui_directory: String,
}

fn main() {
    let opts: Opts = Opts::parse();
}
