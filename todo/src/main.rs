use std::env;

enum CmdName {
    Help,
    List,
}

enum Arg {
    List,
    None,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd: Command = Command::new(&args);
}

struct Command{
    name: CmdName,
    arg: Arg,
}

impl Command{
    fn new(args: &[String]) -> Command{
        if args.len() < 1{
            panic!("todo: Not enough arguments! Find available commands: todo help");
        }
        else if args.len() > 3{
            panic!("todo: Too many arguments! Find available commands: todo help");
        }

        let name: CmdName;
        let arg: Arg;

        match args[1].as_str(){
            "help" => name = CmdName::Help,
            _ => panic!("todo: {} is not a command", args[1].as_str()),
        }
        if args.len() == 3{
            match args[2].as_str(){
                "list" => arg = Arg::List,
                _ => panic!("todo: {} is not an option for any command", args[2]),
            }
        }
        else{
            arg = Arg::None;
        }

        Command {name, arg}
    }
}
