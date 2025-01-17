use webrtc_mdns as mdns;

use mdns::Error;
use mdns::{config::*, conn::*};

use clap::{App, AppSettings, Arg};
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::sync::mpsc;

// For interop with webrtc-rs/mdns_server
// cargo run --color=always --package webrtc-mdns --example mdns_query

// For interop with pion/mdns_server:
// cargo run --color=always --package webrtc-mdns --example mdns_query -- --local-name pion-test.local

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] {} - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                chrono::Local::now().format("%H:%M:%S.%6f"),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Trace)
        .init();

    let mut app = App::new("mDNS Query")
        .version("0.1.0")
        .author("Rain Liu <yuliu@webrtc.rs>")
        .about("An example of mDNS Query")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::with_name("FULLHELP")
                .help("Prints more detailed help information")
                .long("fullhelp"),
        )
        .arg(
            Arg::with_name("server")
                .required_unless("FULLHELP")
                .takes_value(true)
                .default_value("0.0.0.0:5353")
                .long("server")
                .help("mDNS Server name."),
        )
        .arg(
            Arg::with_name("local-name")
                .long("local-name")
                .takes_value(true)
                .default_value("webrtc-rs-test.local")
                .help("Local name"),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("FULLHELP") {
        app.print_long_help().unwrap();
        std::process::exit(0);
    }

    let server = matches.value_of("server").unwrap();
    let local_name = matches.value_of("local-name").unwrap();

    let server = DnsConn::server(
        SocketAddr::from_str(server)?,
        Config {
            ..Default::default()
        },
    )
    .unwrap();

    log::info!("querying dns");

    let (_a, b) = mpsc::channel(1);

    let (answer, src) = server.query(local_name, b).await.unwrap();
    log::info!("dns queried");
    println!("answer = {answer}, src = {src}");

    server.close().await.unwrap();
    Ok(())
}
