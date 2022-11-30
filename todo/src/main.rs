use clap::{load_yaml, App};
use std::error;
use std::io::{Error, ErrorKind};
use todo::subcmds;

fn main() -> Result<(), Box<dyn error::Error>> {
    let yaml = load_yaml!("cli.yaml");
    let app_m = App::from(yaml).get_matches();

    match app_m.subcommand() {
        ("create", Some(sub_m)) => {
            let list_name = match sub_m.value_of("list_name") {
                Some(name) => name,
                None => {
                    let error = Error::new(ErrorKind::Other, "Missing Arguments!");
                    return Err(Box::new(error));
                }
            };

            match subcmds::create(list_name) {
                Err(e) => panic!("{}", e),
                _ => {}
            }
        }
        ("list", Some(sub_m)) => {
            let list_group = match sub_m.value_of("list_group") {
                Some(name) => name,
                None => {
                    let error = Error::new(ErrorKind::Other, "Missing Arguments!");
                    return Err(Box::new(error));
                }
            };

            subcmds::list(list_group)?;
        }
        _ => println!("Default"),
    }

    Ok(())
}
