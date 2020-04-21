use bytes::Bytes;
use clap::{Arg, App, ArgMatches};
use warp::{http::StatusCode, http::Response, Filter};

struct Config {
    port: u16,
    path: String
}

#[tokio::main]
async fn main() {
    let matches = App::new("echor")
        .version("1.0")
        .author("Jamie Barnett")
        .about("will listen on a supplied path and port for any incoming requests and log them to stdout.")
        .arg(Arg::with_name("path").long("path").help("set a path to handle request for. defaults to /post").takes_value(true))
        .arg(Arg::with_name("port").long("port").help("set a port to handle request for. defaults to 7000").takes_value(true))
        .get_matches();

    let config = get_args(matches);

    println!("listening on port {} for oncoming requests to {}", config.port, config.path);

    // TODO panics if any /'s are present in path.
    // Parse path passed to app and supply using path macro
    // or use another framework that can handle an exact path as a string.
    let post = warp::post()
        .and(warp::path(config.path))
        .and(warp::body::bytes())
        .and_then(handler);

    warp::serve(post).run(([0, 0, 0, 0], config.port)).await;

}

async fn handler(body: Bytes) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", body);
    Ok(StatusCode::OK)
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

    Config{
        port,
        path: path.to_owned()
    }
}