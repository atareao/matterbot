mod bot;
mod utils;

use dotenv::dotenv;
use crate::bot::Bot;
use clap::{App, Arg, AppSettings};
use serde_json::Value;
use std::env;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str =env!("CARGO_PKG_VERSION");
const AUTHORS: &str =env!("CARGO_PKG_AUTHORS");


fn main() {
    dotenv().ok();
    let protocol = env::var("PROTOCOL").expect("PROTOCOL not set");
    let base_uri = env::var("BASE_URI").expect("BASE_URI not set");
    let token = env::var("TOKEN").expect("TOKEN not set");

    let bot = Bot::new(&protocol, &base_uri, &token);
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .subcommand(App::new("check")
                    .about("Check")
                    .subcommand(App::new("team")
                                .about("Check if team exists")
                                )
                    )
        .subcommand(App::new("list")
                    .about("List")
                    .subcommand(App::new("users")
                                .about("List users")
                                )
                    .subcommand(App::new("channels")
                                .about("List channels")
                                )
                    .subcommand(App::new("teams")
                                .about("List teams")
                                )
                    .subcommand(App::new("roles")
                                .about("List roles")
                                )
                    .subcommand(App::new("webhooks")
                                .about("List webhooks")
                                .subcommand(App::new("incoming")
                                            .about("List incoming webhooks")
                                            )
                                .subcommand(App::new("outgoing")
                                            .about("List outgoing webhooks")
                                            )
                                )
                    )
        .subcommand(App::new("create")
                    .about("Create")
                    .subcommand(App::new("user")
                                .about("Create a new user")
                                .arg(Arg::new("username")
                                     .short('u')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("email")
                                     .short('e')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("password")
                                     .short('p')
                                     .required(true)
                                     .takes_value(true))
                                )
                    .subcommand(App::new("team")
                                .about("Create a new team")
                                .arg(Arg::new("name")
                                     .short('n')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("display_name")
                                     .short('d')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("private")
                                     .short('p')
                                     .required(false))
                                )
                    .subcommand(App::new("channel")
                                .about("Create a new channel")
                                .arg(Arg::new("team_id")
                                     .short('i')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("name")
                                     .short('n')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("display_name")
                                     .short('d')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("private")
                                     .short('p')
                                     .required(false))
                                )
                    .subcommand(App::new("webhook")
                                .about("Create a new webhook")
                                .subcommand(App::new("incoming")
                                            .about("Create a new incoming webhook")
                                            .arg(Arg::new("channel_id")
                                                 .short('c')
                                                 .required(true)
                                                 .takes_value(true))
                                            .arg(Arg::new("display_name")
                                                 .short('d')
                                                 .required(true)
                                                 .takes_value(true))
                                            )
                                )
                                .subcommand(App::new("outgoing")
                                            .about("Create a new outgoind webhook")
                                            .arg(Arg::new("channel_id")
                                                 .short('c')
                                                 .required(true)
                                                 .takes_value(true))
                                            .arg(Arg::new("display_name")
                                                 .short('d')
                                                 .required(true)
                                                 .takes_value(true))
                                            .arg(Arg::new("words")
                                                 .short('w')
                                                 .required(true)
                                                 .takes_value(true))
                                            )
                                )
        .subcommand(App::new("post")
                    .about("Post")
                    .subcommand(App::new("message")
                                .about("Post a message room")
                                .arg(Arg::new("channel_id")
                                     .short('c')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("message")
                                     .short('m')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("post_id")
                                     .short('p')
                                     .required(false)
                                     .takes_value(true))
                                )
                    )
        .get_matches();
    if let Some(sub) = matches.subcommand_matches("post"){
        if let Some(subsub) = sub.subcommand_matches("message"){
            let channel_id = subsub.value_of("channel_id").unwrap();
            let message = subsub.value_of("message").unwrap();
            let post_id = subsub.value_of("post_id");
            match bot.post_message(channel_id, message, post_id){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("create"){
        if let Some(subsub) = sub.subcommand_matches("user"){
            let username = subsub.value_of("username").unwrap();
            let email = subsub.value_of("email").unwrap();
            let password = subsub.value_of("password").unwrap();
            match bot.create_user(username, email, password){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(subsub) = sub.subcommand_matches("channel"){
            let team_id = subsub.value_of("team_id").unwrap();
            let name = subsub.value_of("name").unwrap();
            let display_name = subsub.value_of("display_name").unwrap();
            let private = subsub.is_present("private");
            match bot.create_channel(team_id, name, display_name, private){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(subsub) = sub.subcommand_matches("team"){
            let name = subsub.value_of("name").unwrap();
            let display_name = subsub.value_of("display_name").unwrap();
            let private = subsub.is_present("private");
            match bot.create_team(name, display_name, private){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(subsub) = sub.subcommand_matches("webhook"){
            if let Some(sub3) = subsub.subcommand_matches("incoming"){
                let team_id = sub3.value_of("team_id").unwrap();
                let display_name = sub3.value_of("display_name").unwrap();
                match bot.create_incoming_webhook(team_id, display_name){
                    Ok(result) => println!("{}", result.text().unwrap()),
                    Err(result) => println!("{}", result.to_string())
                }
            }else if let Some(sub3) = subsub.subcommand_matches("outgoing"){
                let team_id = sub3.value_of("team_id").unwrap();
                let display_name = sub3.value_of("display_name").unwrap();
                let words = sub3.value_of("words").unwrap().split(' ').collect();
                match bot.create_outgoing_webhook(team_id, display_name, words){
                    Ok(result) => println!("{}", result.text().unwrap()),
                    Err(result) => println!("{}", result.to_string())
                }

            }
        }
    }else if let Some(sub) = matches.subcommand_matches("list"){
        if let Some(_subsub) = sub.subcommand_matches("channels"){
            match bot.list_channels(){
                Ok(result) => {
                    let v: Vec<Value> = serde_json::from_str(&result.text().unwrap()).unwrap();
                    for item in &v{
                        println!("{} - {}", item["id"], item["display_name"]);
                    }
                },
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(_subsub) = sub.subcommand_matches("users"){
            match bot.list_users(){
                Ok(result) => {
                    let v: Vec<Value> = serde_json::from_str(&result.text().unwrap()).unwrap();
                    for item in &v{
                        println!("{} - {}", item["id"], item["username"]);
                        //println!("{:?}", item);
                    }
                },
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(_subsub) = sub.subcommand_matches("teams"){
            match bot.list_teams(){
                Ok(result) => {
                    let v: Vec<Value> = serde_json::from_str(&result.text().unwrap()).unwrap();
                    for item in &v{
                        println!("{} - {}", item["id"], item["display_name"]);
                    }
                },
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(subsub) = sub.subcommand_matches("webhooks"){
            if let Some(_sub3) = subsub.subcommand_matches("incoming"){
                match bot.list_incoming_webhooks(){
                    Ok(result) => {
                        let v: Vec<Value> = serde_json::from_str(&result.text().unwrap()).unwrap();
                        for item in &v{
                            println!("{} - {} - {}", item["id"], item["display_name"], item["channel_id"]);
                        }
                    },
                    Err(result) => println!("{}", result.to_string())
                }
            }else if let Some(_sub3) = subsub.subcommand_matches("outgoing"){
                match bot.list_outgoing_webhooks(){
                    Ok(result) => {
                        let v: Vec<Value> = serde_json::from_str(&result.text().unwrap()).unwrap();
                        for item in &v{
                            println!("{} - {} - {}", item["id"], item["display_name"], item["channel_id"]);
                        }
                    },
                    Err(result) => println!("{}", result.to_string())
                }
            }
        }else if let Some(_subsub) = sub.subcommand_matches("roles"){
            match bot.list_roles(){
                Ok(result) => {
                    let v: Vec<Value> = serde_json::from_str(&result.text().unwrap()).unwrap();
                    for item in &v{
                        println!("{} - {}", item["id"], item["display_name"]);
                    }
                },
                Err(result) => println!("{}", result.to_string())
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("chek"){
        if let Some(subsub) = sub.subcommand_matches("team"){
            let name = subsub.value_of("name").unwrap();
            match bot.check_team(name){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }
}
