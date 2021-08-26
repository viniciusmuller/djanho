use std::fs::File;
use std::io::Write;

use clap::{App, Arg};

use djanho::{decoder, generators};

fn main() {
    let matches = App::new("Djanho")
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
        .arg(
            Arg::with_name("LUA_CONFIG")
                .short("l")
                .long("lua")
                .required(false)
                .help("Whether to output the file in lua"),
        )
        .get_matches();

    let has_lua = matches.is_present("LUA_CONFIG");
    let default_extension = if has_lua { "lua" } else { "vim" };

    let filepath = matches.value_of("FILENAME").unwrap();
    let filename = format!("generated.{}", default_extension);
    let output_path = matches
        .value_of("OUTPUT")
        .unwrap_or_else(|| filename.as_str());

    let theme = decoder::parse_file(filepath.to_string());
    let generator = if has_lua {
        generators::generate_lua_config
    } else {
        generators::generate_vimscript_config
    };

    let mut buffer = String::new();
    generator(&mut buffer, theme);
    let mut f = File::create(output_path).expect("Unable to create file");
    f.write_all(buffer.as_bytes())
        .expect("Unable to write the generated config")
}
