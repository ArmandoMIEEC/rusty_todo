mod todo;
use todo::subcmds;
use clap::{App, load_yaml};
use std::io::{Error, ErrorKind};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>>{
    let yaml = load_yaml!("cli.yaml");          
    let app_m = App::from(yaml).get_matches();

    match app_m.subcommand(){
        ("create", Some(sub_m)) =>{
            let list_name = match sub_m.value_of("list_name"){
                Some(name)  => name,
                None =>{
                    let error = Error::new(ErrorKind::Other, "Missing Arguments!");
                    return Err(Box::new(error));
                },
            };

            subcmds::create(list_name)?;
        },
        _=> println!("Default"),
    }

    Ok(())
}
