pub mod subcmds {
    use std::error;
    //use std::fs;
    use std::fs::File;
    use std::io::{Error, ErrorKind};
    extern crate rusqlite;
    use rusqlite::{Connection, Result};

    pub fn create(list_name: &str) -> Result<(), Box<dyn error::Error>> {
        match dirs::home_dir() {
            Some(home_path) => {
                let db_path = format!(".rusty_todo/{}.db", list_name);
                let list_path = home_path.join(db_path);

                if list_path.exists() {
                    let error = Error::new(ErrorKind::AlreadyExists, "Todo list already exists!");
                    return Err(Box::new(error));
                }

                let list_path_str: &str;
                match list_path.to_str() {
                    Some(file) => {
                        File::create(file)?;
                        list_path_str = file;
                    }
                    None => {
                        let error = Error::new(
                            ErrorKind::AddrNotAvailable,
                            "Could not create path to todo list db file!",
                        );
                        return Err(Box::new(error));
                    }
                }

                let mut sql_cmd = String::from("create table if not exists ");
                sql_cmd.push_str(list_name);
                sql_cmd.push_str(
                    " (
                    task_id integer,
                    task text
                )",
                );

                println!("{}", sql_cmd);

                let conn = Connection::open(list_path_str)?;
                conn.execute(sql_cmd.as_str(), [])?;
            }
            None => {
                let error =
                    Error::new(ErrorKind::AddrNotAvailable, "Could not get home directory!");
                return Err(Box::new(error));
            }
        }

        Ok(())
    }

    /*pub fn read_cur_origin() -> Result<(&str), Box<dyn error::Error>> {
        let config_path: std::path::PathBuf;
        let config_path_str: &str;
        match dirs::home_dir() {
            Some(home_path) => {
                config_path = home_path.join(".rusty_todo/config.json");
            }
            None => {
                let error =
                    Error::new(ErrorKind::AddrNotAvailable, "Could not get home directory!");
                return Err(Box::new(error));
            }
        }

        if !(config_path.exists()) {
            let error = Error::new(ErrorKind::AddrNotAvailable, "Config file does not exist!");
            return Err(Box::new(error));
        } else {
            match config_path.to_str() {
                Some(path) => {
                    config_path_str = path;
                }
                None => {
                    let error = Error::new(
                        ErrorKind::AddrNotAvailable,
                        "Error parsing file path to &str",
                    );
                    return Err(Box::new(error));
                }
            }
        }

        let json_data = fs::read(config_path_str).expect("Unable to read config file!");
        Ok(())
    }*/
}
