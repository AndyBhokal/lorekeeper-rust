mod utility;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use utility::config_utils::load_config;
use utility::config_utils::get_config_value;
use utility::database_utils::connect_to_db;
use utility::discord_message_utils;


struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx : Context, msg: Message){
        let prefix: char = ';';
        if msg.content == ";test"{
            if let Err(why) = msg.channel_id.say(&ctx.http, "I'm alive again!").await {
                println!("Error Sending Message: {why:?}");
            }
        }
        if msg.content.starts_with(prefix){
            let mut content  = msg.content.splitn(2, " ");
            let command = content.next().unwrap().to_string().replace(prefix,"");
            let args = content.next().unwrap().to_string();
            
            match command.as_str() {
                "echo" => discord_message_utils::send_message(args, msg, ctx).await,
                "add" => todo!(" db create"),
                "update" => todo!("  db update"),
                "end" => todo!("  db read/ discord aggregate message/ publish"),
                "start" => todo!(" db init/ create table"),
                "help" => todo!("  discord send help message/ discord -> db command lookup"),
                &_ => discord_message_utils::send_message("Not a real command".to_string(), msg, ctx).await, //idk send error or smth not a real command \(-_-)/
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
#[allow(unused_mut)]
#[allow(unused_variables)]
async fn main(){

    let config_list = load_config();
    let token: String = get_config_value(&config_list, "DiscordBotToken");

    let db_addr: String = get_config_value(&config_list, "DatabaseAddress");
    let db_user: String = get_config_value(&config_list, "DatabaseUser");
    let db_pass: String = get_config_value(&config_list, "DatabasePassword");

    let db = connect_to_db(&db_addr, &db_user, &db_pass);

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }    
   
    
}
