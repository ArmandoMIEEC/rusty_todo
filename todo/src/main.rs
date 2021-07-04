mod todo;
use todo::subcommands::create;
use clap::{App, load_yaml};
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error>{
    let yaml = load_yaml!("cli.yaml");
    let app_m = App::from(yaml).get_matches();

    match app_m.subcommand(){
        ("create", Some(sub_m)) =>{
            let list_name = match sub_m.value_of("list_name"){
                Some(name)  => name,
                None =>{
                    let error = Error::new(ErrorKind::Other, "Missing Arguments!");
                    return Err(error);
                },
            };

            create(list_name)?;
        },
        _=> println!("Default"),
    }

    Ok(())
}
