use std::{env,  io::Write};
use serde::{Deserialize, Serialize};
use std::io;
use std::error::Error;
use dotenv::dotenv;
mod utils;

#[derive(Serialize, Deserialize, Debug,Hash)]
struct User {
    name : String,
    surname:String,
    city:String,
    state:String,
    country:String,
}

#[tokio::main]
async fn main()->Result<(),Box<dyn Error>> {
     dotenv().ok();
    let database_url = env::var("DB_URL").expect(
        "DATABASE_URL must be set");
    let  pool = sqlx::postgres::PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut  input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input  = input.trim().to_lowercase();

        match  input.as_str() {
         "write"=>{
            let result  = utils::add_user(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while adding user: {}",err)
            }
        },
         "read"=>{
            let result  = utils::get_all_users(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while fetching users: {}",err)
            }
        },
         "" => println!("{}",""),
         "clear"=>utils::clear(),

         "find"=>{
            let result  = utils::fetch_user_by_id(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while fetching user: {}",err)
            }
        },
         "delete"=>{
            let result  = utils::delete_user_by_id(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while fetching user: {}",err)
            }
        },
         "exit"=>std::process::exit(0),
         _ => println!("Invalid command")
        };
    }
}

