pub mod liberrors {
    use std::fmt;

    #[derive(Debug)]
    pub enum TodoError {
        ListExists(String),
        CannotCreateList(String),
        CannotCreateDir(String),
        CannotCreateFile(String),
        AddrNotAvailable(String),
        SQLError(String),
    }

    impl std::error::Error for TodoError {}

    impl fmt::Display for TodoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TodoError::ListExists(list) => {
                    write!(f, "A list named {} already exists!", &list[..])
                }
                TodoError::CannotCreateList(list) => {
                    write!(f, "Could not create db file for list {}!", &list[..])
                }
                TodoError::AddrNotAvailable(addr) => {
                    write!(f, "Could not find addressk {}!", &addr[..])
                }
                TodoError::CannotCreateDir(dir) => {
                    write!(f, "Could not create directory {}!", &dir[..])
                }
                TodoError::CannotCreateFile(file) => {
                    write!(f, "Could not create file {}!", &file[..])
                }
                TodoError::SQLError(error) => {
                    write!(f, "SQL error: {}!", &error[..])
                }
            }
        }
    }
}

pub mod subcmds {
    use crate::liberrors::TodoError;
    use std::error;
    use std::fs;
    extern crate rusqlite;
    use crate::config;
    use rusqlite::{Connection, Result};

    pub fn create(list_name: &str) -> Result<(), TodoError> {
        let lists_dir = config::find_listsdir().unwrap();
        let list_path = lists_dir.join(list_name);
        let list_path_str: &str;

        if list_path.exists() {
            return Err(TodoError::ListExists(String::from(list_name)));
        }

        match list_path.to_str() {
            Some(file) => {
                fs::File::create(file).expect("Error creating todo list db file!");
                list_path_str = file;
            }
            _ => {
                panic!("Error creating path str for list db file!");
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

        let conn = Connection::open(list_path_str).expect("Error opening sql connection!");
        match conn.execute(sql_cmd.as_str(), []) {
            Err(e) => return Err(TodoError::SQLError(String::from(format!("{}", e)))),
            _ => {}
        }

        Ok(())
    }

    pub fn list(list_group: &str) -> Result<(), Box<dyn error::Error>> {
        match list_group {
            "all" => {
                let files = fs::read_dir("/Users/armando/.rusty_todo/todo_lists").unwrap();
                for file in files {
                    println!("Name: {}", file.unwrap().path().display())
                }
            }
            _ => {
                println!("Vazio!");
            }
        }
        Ok(())
    }
}

pub mod config {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::error;
    use std::fs;
    use std::io::{Error, ErrorKind};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize)]
    struct Config {
        lists_dirpath: String,
    }

    pub fn find_listsdir() -> Result<PathBuf, Box<dyn error::Error>> {
        match dirs::home_dir() {
            Some(home_path) => {
                let config_str = ".rusty_todo/config";
                let config_path = home_path.join(config_str);
                let config_path_str: &str;
                if config_path.exists() {}
                match config_path.to_str() {
                    Some(file) => {
                        fs::File::create(file)?;
                        config_path_str = file;
                    }
                    None => {
                        let error = Error::new(
                            ErrorKind::AddrNotAvailable,
                            "Could not create config file!",
                        );
                        return Err(Box::new(error));
                    }
                }

                let dir_str = ".rusty_todo/todo_lists/";
                let lists_dirpath = home_path.join(dir_str);
                if !lists_dirpath.exists() {
                    match lists_dirpath.to_str() {
                        Some(dir) => {
                            fs::create_dir(dir)?;
                            let data = json!({ "lists_dirpath": dir });
                            fs::write(config_path_str, data.to_string())?;
                        }
                        None => {
                            let error = Error::new(
                                ErrorKind::AddrNotAvailable,
                                "Could not create todo lists directory!",
                            );
                            return Err(Box::new(error));
                        }
                    };
                }

                Ok(lists_dirpath)
            }
            None => {
                let error =
                    Error::new(ErrorKind::AddrNotAvailable, "Could not get home directory!");
                return Err(Box::new(error));
            }
        }
    }
}
