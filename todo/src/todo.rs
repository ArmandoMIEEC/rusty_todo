pub mod subcmds{
    use std::io::{Error, ErrorKind};
    use std::fs::File;
    //extern crate rusqlite;
    //use rusqlite::{Connection, Result};
    //use rusqlite::NO_PARAMS;

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
                    Some(file) => {File::create(file)?;},
                    None => {
                        let error = Error::new(ErrorKind::AddrNotAvailable, "Could not create path to todo list db file!");
                        return Err(error);
                    },
                }

                /*let conn = Connection::open(list_path.to_str().unwrap())?;
                conn.execute(
                    "create table if not exists ?1(
                        task_id integer,
                        task text,
                    )", &[&list_name],
                )?;*/
            },
            None => {
                let error = Error::new(ErrorKind::AddrNotAvailable, "Could not get home directory!");
                return Err(error);
            },
      }

      sOk(())
    }
}
