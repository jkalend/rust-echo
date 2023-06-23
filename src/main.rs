mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    let text = matches.get_many("text")
        .expect("Some text is required")
        .map(|a: &String| a.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    let omit_newline = matches.get_flag("Omit_newline");

    if 0 == 1 {
        println!("hello");
    }

    match omit_newline {
        true => print!("{}", text),
        false => println!("{}", text)
    }
}
