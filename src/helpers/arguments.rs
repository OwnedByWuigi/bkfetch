use crate::helpers::colors::COLORS;

#[derive(Debug)]
pub struct Arguments {
    pub help: bool,
    pub color: String,
    pub list: bool,
    pub boykisser: String   
}

impl Arguments {
    fn validate_color(color: String) -> String {
        for c in COLORS.iter() {
            if c.0 == color {
                return color.to_string();
            }
        };

        Self::print_err("Invalid color provided.");
        std::process::exit(1);
    }

    fn validate_boykisser(boykisser: String) -> String {
        let boykissers = crate::helpers::paths::get_boykissers();

        for p in boykissers.iter() {
            if p == &boykisser {
                return boykisser.to_string();
            }
        };

        Self::print_err("Invalid boykisser provided.");
        std::process::exit(1);
    }   

    fn print_help() {
        println!("Usage: boykisserfetch [OPTION]...");
        println!("Prints a boykisser with system information.");
        println!("
            -h, --help      Display this help and exit
            -c=<color>, --color=<color>     Set the color of the boykisser
            -l=<color>, --list=<color>      List all available boykissers
            -p=<color>, --boykisser=<color>      Set the boykisser to display
        ");
        
        std::process::exit(0);
    }

    fn print_boykissers() {
        let boykissers = crate::helpers::paths::get_boykissers();

        println!("Available boykissers:");
        for boykisser in boykissers.iter() {
            println!("    {}", boykisser);
        }

        std::process::exit(0);
    }

    fn print_err(err: &str) {
        println!("Error: {}", err);
        println!("Usage: boykisserfetch [OPTION]...");
        println!("Try 'boykisserfetch --help' for more information.");
        std::process::exit(1);
    }

    fn get_args(arg: String) -> String {
        let args = arg.split("=").collect::<Vec<&str>>();

        if args.len() < 2 {
            Self::print_err("Invalid argument provided.");
        };

        args[1].to_string()
    }

    pub fn parse() -> Arguments {
        let mut args = Arguments {
            help: false,
            list: false,
            color: String::from(""),
            boykisser: String::from("")
        };

        let args_vec: Vec<String> = std::env::args().collect();

        args_vec.into_iter().for_each(|arg| {
            match arg {
                arg if arg == "--help" || arg == "-h" => args.help = true,

                arg if arg.contains("--color") || arg.contains("-c") => {
                    args.color = Self::validate_color(
                        Self::get_args(arg)
                    );
                },

                arg if arg.contains("--boykisser") || arg.contains("-p") => {
                    args.boykisser = Self::validate_boykisser(
                        Self::get_args(arg)
                    );
                },

                arg if arg == "--list" || arg == "-l" => args.list = true,
                _ => ()
            }
        });

        if args.color == "" {
            args.color = String::from("white");
        }

        if args.boykisser == "" {
            args.boykisser = String::from("howyoulook");
        }

        if args.help {
            Self::print_help();
        }

        if args.list {
            Self::print_boykissers();
        }

        args
    }
}
