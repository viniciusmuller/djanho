use clap::{clap_app, App, Arg};
use std::{fs::File, io::Write};

use djanho::{decoder, generators};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Vinícius Müller <vinigm.nho@gmail.com>")
        (about: "Convert VSCode's JSON themes to Vimscript/Lua themes")
        (@arg FILENAME: +required "Sets the input file to use")
        (@arg OUTPUT: -o --output "Sets the output file to use")
        (@arg LUA_CONFIG: -l --lua "Whether to output the file in Lua")
    ).get_matches();

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

    let config = generator(theme);
    let mut f = File::create(output_path).expect("Unable to create file");
    f.write_all(config.as_bytes())
        .expect("Unable to write the generated config")
}
