use clap::Parser;

/// Configuration !
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ConfigArgs {
    /// The address for accept incoming connection or connect to target.
    #[arg(short = 'l', long, value_name = "IP OR Domain")]
    pub addr: String,

    /// Port.
    #[arg(short, long = "PORT", default_value_t = 1080)]
    pub port: u16,
}

pub fn get_config() -> (String, u16) {
    let arguments = ConfigArgs::parse();

    (arguments.addr, arguments.port)
}

pub fn get_config_2() -> (String, u16) {
    use clap::{arg, Arg, ArgAction, Command};

    let matches = Command::new("Server Configuration.")
        .version("1.0")
        .author("Kevin")
        .about("Does connection things")
        .arg(
            Arg::new("server")
                .short('a')
                .long("server")
                .action(ArgAction::Set)
                .value_name("IP OR Domain")
                .help("The address for accept incoming connection or connect to target.")
                .required(true),
        )
        .arg(
            arg!(-p --port <PORT> "Target server port.")
                .required(true)
                .value_parser(clap::value_parser!(u16).range(1024..6000)),
        )
        .arg(arg!([input] "an optional input file to use"))
        .get_matches();

    let s = matches.get_one::<String>("server").expect("required");
    println!("server: {:?}", s);
    let p = matches.get_one::<u16>("port").expect("required");
    println!("port: {}", *p);

    (String::from(s), *p)
}
