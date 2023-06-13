mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    let text = matches.get_many("text")
        .expect("Some text is required")
        .collect::<Vec<&String>>()
        .into_iter()
        .map(|a| a.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    let omit_newline = matches.get_flag("Omit_newline");

    match omit_newline {
        true => print!("{}", text),
        false => println!("{}", text)
    }
}
