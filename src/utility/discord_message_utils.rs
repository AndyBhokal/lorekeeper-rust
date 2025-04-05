pub async fn send_message(content : String, msg_info : serenity::model::channel::Message, context : serenity::client::Context){
    if let Err(why) = msg_info.channel_id.say( &context.http, content).await {
        println!("Error Sending Message: {why:?}");
    }
}

#[allow(dead_code, unused_variables)]
pub async fn send_aggregate_message(content : String, msg_info : serenity::model::channel::Message, context : serenity::client::Context){
    todo!("finish DB methods")
}