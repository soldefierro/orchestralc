extern crate clap;
use clap::{Arg, App, SubCommand};

mod compile;
mod parameters;

fn main() {
    let matches = App::new("orchestralc")
        .version("0.1.0")
        .author("Sol de Fierro <undisclosed>")
        .about("Runtime free template-to-rust compiler")
        .subcommand(SubCommand::with_name("compile")
            .about("compile an orchestralc template into a rust file")
            .version("0.1.0")
            .author("Sol de Fierro <undisclosed>")
            .arg(Arg::with_name("INPUT")
                .help("location of input file")
                .required(true)
                .index(1))
            .arg(Arg::with_name("OUTPUT")
                .help("location of output file. will create; overwrite")
                .required(true)
                .index(2))
            ).get_matches();


}
