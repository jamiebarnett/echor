extern crate iron;
extern crate router;
extern crate clap;

use clap::{Arg, App, ArgMatches};

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

struct Config {
    port: u16,
    path: String,
}

fn main() {
    let matches = App::new("echor")
        .version("1.0")
        .author("Jamie Barnett")
        .about("will listen on a supplied path and port for any incoming requests and log them to stdout.")
        .arg(Arg::with_name("path").long("path").help("set a path to handle request for. defaults to /post").takes_value(true))
        .arg(Arg::with_name("port").long("port").help("set a port to handle request for. defaults to 7000").takes_value(true))
        .get_matches();

    let config = get_args(matches);

    println!("listening on port {} for incoming requests to {}", config.port, config.path);

    let mut router = Router::new();
    router.post(config.path, handler, "index");

    Iron::new(router).http(format!("localhost:{}", config.port)).unwrap();

}

fn handler(req: &mut Request) -> IronResult<Response> {

    println!("received {} request to {}", req.method, req.url);

    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();

    match payload.as_str() {
        "" => {
            println!("with no body")
        }
        _=> {
            println!("with body : {}", payload)
        }
    }

    Ok(Response::with(status::Ok))
}

fn get_args(matches: ArgMatches) -> Config {
    let path = match matches.value_of("path").to_owned() {
        Some(path) => path,
        None => "post"
    };

    let port = match matches.value_of("port").to_owned() {
        Some(port) => {
            let port: u16 = port.parse().expect("port must be a valid integer");
            port
        }
        None => 7000
    };

    Config {
        port,
        path: path.to_owned(),
    }
}