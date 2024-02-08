use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[test]
fn find_a_matches() {
    let mut result = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn main() -> Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // //nth(1) means get the first argument
    // //expect means nth() is an option. If its Some then output value, otherwise output message
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", args.path.display()))?;

    // println!("pattern: {:?},path: {:?}", args.pattern, args.path)

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())

    /*
    Nice error reporting:
    let result = std::fs::read_to_string("test.txt");
    match result {
        Ok(content) => {println!("File content: {}", content); }
        Err(error) => { println!("Oh noes: {}", error); }
        //Err(error) => {panic!("Can't deal with {}, just exit here", error); }
    }
    //let content = std::fs::read_to_string("test.txt"),unwrap();


    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let result = std::fs::read_to_string("test.txt");
        let content = match result {
            Ok(content) => { content },
            Err(error) => { return Err(error.into()); }
        };
        // let content = std::fs::read_to_string("test.txt")?;


        println!("file content: {}", content);
        Ok(())
    }
     */
}
