use std::env;

enum CmdName {
    Help,
    None
}

enum Arg {
    See,
    None
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
            _ => panic!("coiso"),
        }
        match args[2].as_str(){
            "see" => arg = Arg::See,
            _ => panic!("coiso"),
        }

        Command {name, arg}
    }
}
