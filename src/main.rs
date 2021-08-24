use jargon_args::Jargon;
use std::process::exit;
use std::collections::HashMap;

const URL: &str = "https://api.myip.com";

const HELP: &str = "\
myip [OPTIONS]

OPTIONS:
    -a, --all       Display all IP information
    -C, --code      Display two letter country code
    -c, --country   Display only country
    -h, --help      Display help information
    -r, --raw       Display raw API response
    -v, --version   Display version information
";

const VERSION: &str = "\
myip version";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut j: Jargon = Jargon::from_env();

    if j.contains(["-h", "--help"]) {
        print!("{}", HELP);
        exit(0);
    } else if j.contains(["-v", "--version"]) {
        println!("{} {}\nThanks to https://myip.com/ for hosting this API.",
                 VERSION, env!("CARGO_PKG_VERSION")
        );
        exit(0);
    }

    let all: bool = j.contains(["-a", "--all"]);
    let country: bool = j.contains(["-c", "--country"]);
    let cc: bool = j.contains(["-C", "--code"]);
    let raw: bool = j.contains(["-r", "--raw"]);

    let resp = reqwest::get(URL)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    if all {
        println!("IP:\t\t{}\nCountry:\t{}\nCC:\t\t{}",
        resp["ip"], resp["country"], resp["cc"]);
    } else if country {
        println!("{}", resp["country"]);
    } else if cc {
        println!("{}", resp["cc"]);
    } else if raw {
        println!("{:#?}", resp);
    } else {
        println!("{}", resp["ip"])
    }

    Ok(())
}
