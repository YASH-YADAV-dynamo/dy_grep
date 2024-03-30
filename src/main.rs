use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(op: |err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(code: 1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename); 

    let contents: String = fs::read_to_string(path:config.filename):Result<String, Error>
        .expect(msg: "Something went wrong reading the file");
    println!("With text:\n{}",contents);


    struct Config {
        query: String,
        filename: String,
    }
     impl Config {
        fn new(args: &[String]) -> Result<Config, &str>{
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query:String = args[1].clone();
            let filename:String = args[2].clone();

            OK(Config{ query, filename })
        }
     }

    
}
