use std::env;

const COMMANDS: [&'static str; 1] = ["help"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd: Command = Command::new(&args);

    println!("Command was {} {}", cmd.name, cmd.option.unwrap());
}

struct Command{
    name: String,
    option: Option<String>,
}

impl Command{
    fn new(args: &[String]) -> Command{
        if args.len() < 1{
            panic!("todo: Not enough arguments! Find available commands: todo help");
        }
        else if args.len() > 3{
            panic!("todo: Too many arguments! Find available commands: todo help");
        }

        for cmd in

        let name: String = args[1].clone();
        let option: Option<String> = Some(args[2].clone());

        Command {name, option}
    }
}
