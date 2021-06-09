use std::env;

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
            panic!("Not enough arguments! Find available commands: todo help");
        }

        let name: String = args[1].clone();
        let option: Option<String> = Some(args[2].clone());

        Command {name, option}
    }
}
