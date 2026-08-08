use std::fs;
use clap::Parser;
use serde::Deserialize;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    service_name: String
}

#[derive(Deserialize)]
struct Config {
    client: String,
    server: String
}

fn main() {
    let args = Cli::parse();

    // args
    let pattern = args.pattern;
    let service_name = args.service_name;

    // mutables
    let file_path: String;
    let handler_suffix: String;
    let tag: String;

    // reading feature.toml
    let config: Config; {
        let content = fs::read_to_string("feature.toml").expect("msg");
        config = toml::from_str(&content).expect("msg");
    }

    if pattern == "client" {
        tag = String::from("Client");
        file_path = config.client;
        handler_suffix = String::from("client");
    } else if pattern == "server" {
        tag = String::from("Server");
        file_path = config.server;
        handler_suffix = String::from("server");
    } else {
        panic!("Attempted to create feature without `pattern`!");
    }

    fs::create_dir(format!("{file_path}/{service_name}")).expect("hi");
        {
            let controller_name = format!("{service_name}{tag}Controller");
            fs::write(
                format!(
                    "{file_path}/{service_name}/{controller_name}.luau"
                ), 
                format!(
                    "local {controller_name} = {{}}\n\nfunction {controller_name}.Init()\n\nend\n\nreturn {controller_name}"
                )
            ).expect("");

            let handler_name = format!("{service_name}ClientHandler");
            fs::write(
                format!(
                    "{file_path}/{service_name}/{handler_name}.{handler_suffix}.luau"
                ), 
                ""
            ).expect("");
        }
}