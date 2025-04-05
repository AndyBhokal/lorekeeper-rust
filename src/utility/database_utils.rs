
// use std::time::{SystemTime,UNIX_EPOCH};

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn connect_to_db(address : &str, user : &str, pass : &str) -> Result<Surreal<Client>, surrealdb::Error>{
    let db = Surreal::new::<Ws>(address).await?;
    
    db.signin(Root {
        username: user,
        password: pass,
    })
    .await?;
    Ok(db)
}

#[allow(dead_code, unused_variables)]
pub async fn create_document_entry(database : Surreal<Client>, content : &str){
    // database.create().content(content);
}