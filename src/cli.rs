use clap::{Command, Arg, ArgAction};

pub fn build_cli() -> Command {
    Command::new("echo")
        .about("Rust version of the echo command")
        .version("0.1.0")
        .arg(Arg::new("text")
            .action(ArgAction::Append)
            .value_name("TEXT")
            .required(true)
            .help("Input text"))
        .arg(Arg::new("Omit_newline")
            .action(ArgAction::SetTrue)
            .short('n')
            .overrides_with("Omit_newline")
            .help("Omits the final newline"))
}
