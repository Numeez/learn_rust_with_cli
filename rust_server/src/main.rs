use std::{env,  io::Write, num::ParseIntError};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::io;
use std::process;
use std::error::Error;
use dotenv::dotenv;

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
       
    // let path = "./json_database/db.json";
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut  input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input  = input.trim().to_lowercase();

        match  input.as_str() {
         "write"=>{
            let result  = add_user(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while adding user: {}",err)
            }
        },
         "read"=>{
            let result  = get_all_users(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while fetching users: {}",err)
            }
        },
         "" => println!("{}",""),
         "clear"=>clear(),

         "find"=>{
            let result  = fetch_user_by_id(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while fetching user: {}",err)
            }
        },
         "delete"=>{
            let result  = delete_user_by_id(&pool).await;
            match result {
                Ok(_)=>{}
                Err(err)=>println!("Error occured while fetching user: {}",err)
            }
        },
         "exit"=>std::process::exit(0),
         _ => println!("Invalid command")
        };
    
        
       

    // }
}
}


async  fn add_user(pool:&sqlx::PgPool)->Result<(),Box<dyn Error>>{
    let name = get_user_input(String::from("Enter user name"));
    let surname = get_user_input(String::from("Enter user surname"));
    let city = get_user_input(String::from("Enter user city"));
    let state = get_user_input(String::from("Enter user state"));
    let country = get_user_input(String::from("Enter user country"));
    let user = User{
        name:name.clone(),
        surname:surname.clone(),
        city:city.clone(),
        state:state.clone(),
        country:country.clone()
    };
    let query = "INSERT INTO user_information (name,surname,city,state,country) VALUES ($1,$2,$3,$4,$5)";
    sqlx::query(&query)
    .bind(&user.name) 
    .bind(&user.surname) 
    .bind(&user.city) 
    .bind(&user.state) 
    .bind(&user.country)
    .execute(pool)
    .await?;
    println!("> User added successfully");
    Ok(())
}


async fn get_all_users(pool:&sqlx::PgPool)->Result<(),Box<dyn Error>>{
    let q = "SELECT * FROM user_information";
   let query = sqlx::query(q);
    let rows = query.fetch_all(pool).await?;
    let data:Vec<User> = rows.iter().map(|row|User{
        name: row.get("name"),
        surname: row.get("surname"),
        city: row.get("city"),
        state: row.get("state"),
        country: row.get("country"),

    }).collect();
    for user in data {
        pretty_print(&user);
    }
    Ok(())
}
async fn delete_user_by_id(pool: &sqlx::PgPool)-> Result<(),Box<dyn Error>>{
    let id = get_user_input(String::from("Enter user id"));
    let user_id_val:Result<i32, ParseIntError> = id.as_str().parse();
    let user_id  = match user_id_val {
        Ok(id)=>id,
        Err(err)=> {return  Err(err.into());}
    };
    let q =  "DELETE FROM user_information WHERE id=$1";
    sqlx::query(q)
    .bind(user_id)
    .execute(pool).await?;
    println!("> User deleted successfully");
    Ok(())
}

async  fn fetch_user_by_id(pool :&sqlx::PgPool)->Result<(),Box<dyn Error>>{
    let id = get_user_input(String::from("Enter user id"));
    let user_id_val:Result<i32, ParseIntError> = id.as_str().parse();
    let user_id  = match user_id_val {
        Ok(id)=>id,
        Err(err)=> {return  Err(err.into());}
    };
    let q = "SELECT * FROM user_information WHERE id = $1";
    let query = sqlx::query(q);
    let row  = query.bind(user_id).fetch_one(pool).await?;
    let user = User{
        name: row.get("name"),
        surname: row.get("surname"),
        city: row.get("city"),
        state: row.get("state"),
        country: row.get("country"),
    };
    pretty_print(&user);
    Ok(())
}


fn pretty_print(user:&User){
    println!("----------------------------------------------------");
    println!("");
    println!("Name : {}",user.name);
    println!("Surname : {}",user.surname);
    println!("City : {}",user.city);
    println!("State : {}",user.state);
    println!("Country : {}",user.country);
    println!("");
    println!("----------------------------------------------------")
}

fn clear(){
    process::Command::new("clear").status().unwrap();
}


fn get_user_input(question:String)->String{
    print!("> {}: ",question);
    io::stdout().flush().unwrap();
    let mut user_data = String::new();
    io::stdin().read_line(&mut user_data).unwrap();
    String::from(user_data.trim())
}
