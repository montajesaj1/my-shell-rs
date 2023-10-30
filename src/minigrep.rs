use std::fs;
use std::error::Error;

pub struct Grep {
    pub query: String,
    pub file_path: String,
}

pub fn run(grep: Grep) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(grep.file_path)?;
        println!("With text: \n {contents}");
        Ok(())
 }

impl Grep {
    pub fn build(args: &[String]) -> Result<Grep, &'static str> {
        // --error handling--
        if args.len() < 2 {
          return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Grep { query, file_path })
    }
}
