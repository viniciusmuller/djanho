use std::fs::File;
use std::io::Write;

use clap::{App, Arg};

use converter::{decoder, generators};

fn main() {
    let matches = App::new("VSCode->Vim Colorscheme Converter")
        .version("0.1")
        .author("Vinícius Müller <vinigm.nho@gmail.com>")
        .about("Convert VSCode's JSON themes to Vimscript/Lua themes")
        .arg(
            Arg::with_name("FILENAME")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .help("Sets the output file to use")
                .takes_value(true),
        )
        .get_matches();

    let filepath = matches.value_of("FILENAME").unwrap();
    let output_path = matches.value_of("OUTPUT").unwrap_or("generated.vim");
    let theme = decoder::parse_file(filepath.to_string());
    let generated = generators::generate_vimscript_config(theme);

    let mut f = File::create(output_path).expect("Unable to create file");
    f.write_all(generated.as_bytes())
        .expect("Unable to write the generated config")
}
