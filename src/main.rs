use std::{fs, path::Path};
use clap::{CommandFactory, Parser, error::ErrorKind};
use serde::Deserialize;

#[derive(Parser)]
struct Cli {
    command: String,
    service_name: Option<String>
}

#[derive(Deserialize)]
struct Config {
    client_service: String,
    client_path: String,

    server_service: String,
    server_path: String
}

const CONFIG_FILE: &str = "feature.toml";
const DEFAULT_CLIENT_PATH: &str = "sync/StarterPlayer/StarterPlayerScripts/GameClient/";
const DEFAULT_CLIENT_SERVICE: &str = "StarterPlayer";
const DEFAULT_SERVER_PATH: &str = "sync/ServerScriptService/GameServer/";
const DEFAULT_SERVER_SERVICE: &str = "ServerScriptService";

fn main() {
    let args = Cli::parse();

    // main arg
    let command = args.command;

    // mutables
    let file_path: String;
    let luau_path: String;
    let handler_suffix: String;
    let handler_content: String;
    let tag: String;

    if command == "init" {
        // does not exist yet
        if !Path::new(CONFIG_FILE).exists() {
            fs::write(
                CONFIG_FILE, 
                format!(
                    "client_path = \"{DEFAULT_CLIENT_PATH}\"\n\
                    client_service = \"{DEFAULT_CLIENT_SERVICE}\"\n\
                    \n\
                    server_path = \"{DEFAULT_SERVER_PATH}\"\n\
                    server_service = \"{DEFAULT_SERVER_SERVICE}\"\n\
                    "
                )
            ).ok();
        } else {
            Cli::command().error(
                ErrorKind::Io, 
                format!("Attempted to run init command with already existing {CONFIG_FILE}!")
            ).exit()
        }
    } else if command == "client" || command == "server" {
        // reading CONFIG_FILE
        let config: Config; {
            let content = fs::read_to_string(CONFIG_FILE)
                .map_err(|_| {
                    Cli::command().error(
                        ErrorKind::Io, 
                        format!("Attempted to create feature files without a {CONFIG_FILE}!")
                    ).exit()
                })
                .unwrap();
            config = toml::from_str(&content)
                .map_err(|_| {
                    Cli::command().error(
                        ErrorKind::Io, 
                        format!("Failed to read/deserialize {CONFIG_FILE}, double-check the format?")
                    ).exit()
                })
                .unwrap();
        }
        let client_path = config.client_path;
        let client_service = config.client_service;
        let server_path = config.server_path;
        let server_service = config.server_service;

        let service_name = args.service_name.ok_or_else(|| {
            Cli::command().error(
                ErrorKind::ArgumentConflict, 
                "Attempted to run a create (client/server) command without service_name argument!"
            ).exit()
        })
        .unwrap();

        if command == "client" {
            tag = String::from("Client");
            file_path = client_path;
            handler_suffix = String::from("client");
            {
                let index = file_path
                    .find(&client_service)
                    .ok_or_else(|| {
                        Cli::command().error(
                            ErrorKind::Io, 
                            format!("Invalid client_service from {CONFIG_FILE}, could not match to client!")
                        ).exit()
                    })
                    .unwrap();
    
                luau_path = file_path[index..].replace("/", ".");
            }
            handler_content = format!(
                "local StarterPlayer = game:GetService(\"StarterPlayer\")\n\
                \n\
                local {service_name}ClientController = require({luau_path}{service_name}.{service_name}ClientController)\n\
                \n\
                {service_name}ClientController.Init()\n\
                "
            )
        } else if command == "server" {
            tag = String::from("Server");
            file_path = server_path;
            handler_suffix = String::from("server");
            {
                let index = file_path
                    .find(&server_service)
                    .ok_or_else(|| {
                        Cli::command().error(
                            ErrorKind::Io, 
                            format!("Invalid server_service from {CONFIG_FILE}, could not match to server!")
                        ).exit()
                    })
                    .unwrap();
    
                luau_path = file_path[index..].replace("/", ".");
            }
            handler_content = format!(
                "local ServerScriptService = game:GetService(\"ServerScriptService\")\n\
                \n\
                local {service_name}ServerController = require({luau_path}{service_name}.{service_name}ServerController)\n\
                \n\
                {service_name}ServerController.Init()\n\
                "
            )
        } else {
            panic!("Attempted to create feature without command!");
        }

        {
            // create dir
            fs::create_dir(format!("{file_path}{service_name}")).ok();

            // create ..Controller.luau
            {
                let controller_name = format!("{service_name}{tag}Controller");
                fs::write(
                    format!(
                        "{file_path}{service_name}/{controller_name}.luau"
                    ), 
                    format!(
                        "local {controller_name} = {{}}\n\
                        \n\
                        function {controller_name}.Init()\n\
                        \n\
                        end\n\
                        \n\
                        return {controller_name}"
                    )
                ).ok();
            }

            // create ..Handler.luau
            {
                let handler_name = format!("{service_name}{tag}Handler");
                fs::write(
                    format!(
                        "{file_path}{service_name}/{handler_name}.{handler_suffix}.luau"
                    ), 
                    handler_content
                ).ok();
            }
        }
    }
}