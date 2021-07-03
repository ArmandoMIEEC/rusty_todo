pub mod subcommands{
    use std::error::Error;
    pub fn create(list_name: &str) -> Result<(), Box<dyn Error>>{
        println!("listname: {}", list_name);
        Ok(())
    }
}
