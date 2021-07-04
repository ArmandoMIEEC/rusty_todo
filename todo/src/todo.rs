pub mod subcommands{
    use std::io::{Error, ErrorKind};
    use std::fs::File;

    pub fn create(list_name: &str) -> Result<(), Error>{
        match dirs::home_dir() {
            Some(home_path) =>{
                let db_path = format!(".rusty_todo/{}.db", list_name);
                let list_path =  home_path.join(db_path);

                if list_path.exists(){
                    let error = Error::new(ErrorKind::AlreadyExists, "Todo list already exists!");
                    return Err(error);
                }

                match list_path.to_str(){
                    Some(path) => {File::create(path)?;},
                    None => {
                        let error = Error::new(ErrorKind::AddrNotAvailable, "Could not create path to todo list db file!");
                        return Err(error);
                    },
                }
            },
            None => {
                let error = Error::new(ErrorKind::AddrNotAvailable, "Could not get home directory!");
                return Err(error);
            },
        }

        Ok(())
    }
}
