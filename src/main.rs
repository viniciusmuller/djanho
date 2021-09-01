use clap::clap_app;
use std::{fs::File, io::Write};

use djanho::{
    decoder, generator::ConfigGenerator, generators, lua::LuaGenerator,
    vimscript::VimscriptGenerator,
};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Vinícius Müller <vinigm.nho@gmail.com>")
        (about: "Convert VSCode's JSON themes to Vimscript/Lua themes")
        (@arg FILENAME: +required "Sets the input file to use")
        (@arg OUTPUT: -o --output "Sets the output file to use")
        (@arg LUA_CONFIG: -l --lua "Whether to output the file in Lua")
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

    // Select generator and generate config
    let mut generator: Box<dyn ConfigGenerator> = if has_lua {
        Box::new(LuaGenerator::default())
    } else {
        Box::new(VimscriptGenerator::default())
    };
    generators::generate_config(theme, &mut generator);
    let config = generator.collect();

    let mut f = File::create(output_path).expect("Unable to create file");
    f.write_all(config.as_bytes())
        .expect("Unable to write the generated config")
}
