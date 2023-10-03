pub struct Config {
    pub file_path: String,
}

impl Config { 
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        Ok(Config { file_path: args[1].clone() } )
    }
}

pub fn split_slides(file_content: String) -> Vec<String> {
    return Vec::new();
}