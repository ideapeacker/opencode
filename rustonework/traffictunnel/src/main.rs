use clap::Parser;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

/// The network traffic forwarder tunnel.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Local address for listen.
    #[arg(short, long, value_name = "IP", default_value_t=String::from("127.0.0.1"))]
    local: String,

    /// Local port for listen and receive incoming connection.
    #[arg(short, long = "lport", default_value_t = 1080)]
    port: u16,

    /// Remote address for connecting to.
    #[arg(short, long, value_name = "IP")]
    remote: String,

    /// Remote port.
    #[arg(
        short = 'd',
        long = "dport",
        default_value_t = 1081,
        value_name = "PORT"
    )]
    tport: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let args = Args::parse();

    "".to_string();
    let local = format!("{}:{}", &args.local, args.port);

    let destp = format!("{}:{}", &args.remote, args.tport);

    let listener = TcpListener::bind(&local).await?;
    info!("Listen on {:?}", &local);
    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Accept a connection from {:?}", addr.to_string());
        let dest = destp.clone();
        tokio::spawn(async move {
            match TcpStream::connect(&dest).await {
                Ok(mut remote) => {
                    info!("Okay to connect {:?}", &dest);

                    if let Err(e) = tokio::io::copy_bidirectional(&mut socket, &mut remote).await {
                        error!("[+] copy_bidirectional:{:?}", e);
                    }
                }
                Err(e) => {
                    error!("[+] Failed to connect {:?}: {:?}", &dest, e);
                }
            }
        });
    }
}

fn init_logger() {
    use chrono::Local;
    use env_logger::fmt::Color;
    use env_logger::Env;
    use log::LevelFilter;
    use std::io::Write;

    //set_var("RUST_LOG", "debug");
    let env = Env::default().filter_or("MY_LOG_LEVEL", "debug");

    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug | log::Level::Trace => Color::Cyan,
            };

            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);
            let mut style = buf.style();
            style.set_color(Color::White).set_dimmed(true);

            // writeln!(buf,"[{} {} {}] {}",
            // Local::now().format("%m/%d/%Y %H:%M:%S%.3f"),
            // level_style.value(record.level()),
            // style.value(record.module_path().unwrap_or("")),
            // record.args())
            writeln!(
                buf,
                "[{} {}] {}",
                Local::now().format("%m/%d/%Y %H:%M:%S%.3f"),
                level_style.value(record.level()),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    // " date 精确到毫秒 ({:?})",  local.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    // " date 精确到微秒 ({:?})",  local.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
    // " date 精确到纳秒 ({:?})",  local.format("%Y-%m-%d %H:%M:%S%.9f").to_string()
}
