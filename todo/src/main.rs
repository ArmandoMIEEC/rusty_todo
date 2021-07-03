mod todo;
use todo::subcommands::create;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let app_m = App::from(yaml).get_matches();

    match app_m.subcommand(){
        ("create", Some(..)) =>{
            if let Some(sub_m) = app_m.subcommand_matches("create"){
                create(sub_m.value_of("list_name").unwrap()).unwrap();}
        }
        _=>{ println!("Default");}
    }

}
