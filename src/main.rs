use clap::{Arg,App};

struct Config {
    port: String,
    path: String
}

impl Config {

}

fn main() {
    let matches = App::new("echor")
        .version("1.0")
        .author("Jamie Barnett")
        .about("will listen on a supplied path and port for any incoming requests and log them to stdout.")
        .arg(Arg::with_name("path").short("p").long("path").help("set a path to handle request for. defaults to /").takes_value(true));
}
