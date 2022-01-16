mod bot;
mod utils;

use crate::utils::read_from_toml;
use crate::bot::Bot;
use clap::{App, Arg, AppSettings};
use dirs::config_dir;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str =env!("CARGO_PKG_VERSION");
const AUTHORS: &str =env!("CARGO_PKG_AUTHORS");


fn main() {
    let config_path = config_dir().unwrap()
        .join("matterbot")
        .join("matterbot.conf");
    if !config_path.exists(){
        println!("Configure MatterMost Bot");
        return;
    }
    let config = read_from_toml(config_path.to_str().unwrap());
    let protocol = config.get("PROTOCOL").unwrap();
    let base_uri = config.get("BASE_URI").unwrap();
    let token = config.get("TOKEN").unwrap();
    let bot = Bot::new(protocol, base_uri, token);
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .subcommand(App::new("list")
                    .about("List")
                    .subcommand(App::new("users")
                                .about("List users")
                                )
                    .subcommand(App::new("channels")
                                .about("List channels")
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
    }else if let Some(sub) = matches.subcommand_matches("list"){
        if let Some(_subsub) = sub.subcommand_matches("channels"){
            match bot.list_channels(){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(_subsub) = sub.subcommand_matches("users"){
            match bot.list_users(){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }
}
