use crate::minigrep::Grep;
// use minigrep::run;
mod minigrep;
use std::env;
use std::io::*;
use std::process::*;
use std::path::*;
use std::process;
use std::fs;

fn main() {
    println!("> my shell 🐢");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();

        // read standard input
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {

            // remove trailing new line
            let mut parts  = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;

                },

                "minigrep" => {
                    let args: Vec<String> = env::args().collect();
                    let config: Grep = Grep::build(&args).unwrap_or_else(|err| {
                        println!("Error passing args: {err}");
                        process::exit(1);
                    });

                    println!("Searching for {}", config.query);
                    println!("In file {}", config.file_path);

                    if let Err(e) = minigrep::run(config) {
                        println!("App error: {e}");
                        process::exit(1);
                    }

                },

                "ndir" => {
                    let dir_name = args.peekable().peek().map_or_else(|| {
                        eprintln!("Expected directory name");
                        process::exit(1);
                    }, |&x| x);

                    match fs::create_dir(dir_name) {
                        Ok(_) => println!("Directory created successfully"),
                        Err(e) => eprintln!("Error creating directory: {e}"),
                    }
                }

                "nfil" => {
                    let file_name = args.peekable().peek().map_or_else(|| {
                        eprintln!("Expected file name");
                        process::exit(1);
                    }, |&x| x);

                    match fs::File::create(file_name) {
                        Ok(_) => println!("File created successfully"),
                        Err(e) => eprintln!("Error creating file: {e}"),
                    }
                }

                "exit" => return,

                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                                Stdio::from(output.stdout.unwrap())
                            });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()

                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();


                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
