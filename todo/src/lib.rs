#[cfg(test)]
mod tests {}

/// # Error Handling
/// In rusty todo there are 3 categories of errors:
/// - Recoverable errors;
/// - Unrecoverable errors that trigger a user notification;
/// - Unrecoverable erros that cause panic.
/// ### Recoverable Errors
/// **Possible actions:** Propagate (if in not in main) OR Solve error (if in main).
///
/// **Note:** Rusty To-Do's lib crate does not panic, but might return early. It propagates every single error to the binary crate, which then decides if an error is recoverable or not (Rusty To-Do uses the "anyhow" crate for error propagation and "thiserror" to derive handel custom error trait implementations).
/// ### Unrecoverable Errors (user notification)
/// **Possible actions:** Notify user, end program without panicking.
///
/// **Note:** This kind of error should be used when the error is likely caused by the user and there is no relevant information to be displayed from a developer standpoint (ex: user used command line arguments incorrectly). Notificating the user will sufice.
/// ### Unrecoverable errors (panic)
/// **Possible actions:** Notify user and panic OR panic.
///
/// **Note:** This are technical errors that are not likely user triggered and are relevant from a developer standpoint. This should be the most common type of error during development and the least common after release. This errors most likely indicate bugs.
pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ToDoError {
        #[error("Error - could not create file: {0}.")]
        CannotCreateFile(String),
        #[error("Error - not found: {0}.")]
        NotFound(String),
        #[error("Error - unable to: {0}.")]
        Unable(String),
    }
}

/*pub mod data {
    pub struct Item {
        item_id: u8,
        item_name: String,
        item_done: bool,
        item: String,
    }
    pub struct List {
        list_id: u8,
        list_name: String,
        items: Vec<Item>,
    }
}*/

///Support Functions
pub mod utilities {
    use crate::errors::ToDoError;
    use anyhow::bail;
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::fs;
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize, Debug)]
    struct Config {
        db_path: PathBuf,
    }

    fn find_config() -> Result<PathBuf> {
        match dirs::home_dir() {
            Some(home_path) => {
                let config_path = home_path.join(".todo/config.txt");
                if !config_path.exists() {
                    match config_path.to_str() {
                        Some(file) => {
                            fs::File::create(file)?;
                            let data = json!({ "db_path": ""});
                            fs::write(file, data.to_string())?;
                        }
                        None => {
                            bail!(ToDoError::CannotCreateFile(String::from("config.txt",)))
                        }
                    }
                }
                Ok(config_path)
            }
            None => bail!(ToDoError::NotFound(String::from("Home Directory"))),
        }
    }

    pub fn find_db() -> Result<PathBuf> {
        let config_path = find_config()?;
        let config_contents = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_contents[..])?;
        match config.db_path.to_str() {
            Some("") => {
                println!("create database");
            }
            Some(path_str) => println!("open database: {path_str}"),
            None => bail!(ToDoError::Unable(String::from(
                "check if local database exists."
            ))),
        }

        Ok(config.db_path)
        //Verify if path is none -> if it is create db
    }
}

///Command execution
pub mod commands {
    use crate::{errors::ToDoError, utilities::find_db};
    use anyhow::Result;
    //use std::error;
    //use rusqlite::{Connection, Result};

    pub fn create(list_name: String) -> Result<()> {
        //check if local db exists
        //if not, create local db
        //check if list with same name exist
        //if so return error will user notifocation
        //check which id to use
        //create list
        //return
        find_db()?;
        Ok(())
    }
}
