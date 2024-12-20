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
