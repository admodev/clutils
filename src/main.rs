use std::env;

fn echo_input(mut input: Vec<String>) -> String {
    input[1] = "".to_string();

    let joined_string = input
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    println!("{}", joined_string);

    return joined_string;
}

fn main() {
    let args: Vec<String> = env::args().map(|s| s.to_string()).collect();

    match args.get(1) {
        Some(arg) if arg == "echo" => {
            let output = echo_input(args);
            println!("{}", output);
        }
        Some(arg) if arg == "nothing" => println!("Nada."),
        _ => println!("Helper function!"),
    }
}
