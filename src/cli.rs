extern crate clap;
use clap::{App, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("scan")
                .about("scans a drectory")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .long("debug")
                        .help("print debug information verbosely"),
                )
                .arg(
                    Arg::with_name("directory")
                        .short("d")
                        .long("directory")
                        .value_name("DIR")
                        .help("Directory for configuration fragments")
                        .takes_value(true),
                ),
        )
}
