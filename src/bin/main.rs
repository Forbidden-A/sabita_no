use clap::{crate_name, crate_version, App, AppSettings, Arg};
use sabita_no_lib::run;
use std::{env, fs::File, io::Read, path::Path};

fn create_app() -> App<'static> {
    let clap_color_setting = if env::var_os("NO_COLOR").is_none() {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let app = App::new(crate_name!())
        .version(crate_version!())
        .global_setting(clap_color_setting)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::HidePossibleValuesInHelp)
        .about("Execute brainf*ck code.")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .required_unless_present("code")
                .about("File to execute.")
                .alias("Path")
                .short('f')
                .short_alias('p'),
        )
        .arg(
            Arg::new("code")
                .value_name("STRING")
                .required_unless_present("file")
                .conflicts_with("file")
                .about("Code to execute.")
                .alias("String")
                .short('c')
                .short_alias('s'),
        )
        .arg(
            Arg::new("input")
                .value_name("STRING")
                .default_value("")
                .about("Input to use.")
                .short('i')
                .short_alias('I'),
        );
    app
}

fn main() {
    let matches = create_app().get_matches();

    let mut source = String::new();

    if matches.is_present("file") {
        let path = Path::new(matches.value_of("file").unwrap());
        if !path.exists() || !path.is_file() {
            return println!("{} is not a valid file.", matches.value_of("file").unwrap());
        }
        let file = File::open(path);
        if file.is_err() {
            panic!("Failed to open file: {}", file.unwrap_err())
        }
        let mut file = file.unwrap();
        let res = file.read_to_string(&mut source);
        if res.is_err() {
            panic!("Failed to read file: {}", res.unwrap_err())
        }
    } else if matches.is_present("code") {
        source = matches.value_of("code").unwrap().to_string();
    }

    if source.is_empty() {
        panic!("Cannot execute empty program.")
    }

    let res = run(&source, matches.value_of("input").unwrap());

    if res.is_err() {
        panic!("Failed to execute code: {}", res.unwrap_err())
    } else {
        println!("{}", res.unwrap())
    }
}
