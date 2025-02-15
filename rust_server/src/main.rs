use std::{fs::{self, File}, io::Write};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io;
use std::process;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name : String,
    surname:String,
    city:String,
    state:String,
    country:String,
}


fn main() {
    let path = "./json_database/db.json";
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut  input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input  = input.trim().to_lowercase();

        match  input.as_str() {
         "write"=>add_user(path),
         "read"=>read(path),
         "" => println!("{}",""),
         "clear"=>clear(),
         "exit"=>std::process::exit(0),
         _ => println!("Invalid command")
        };
    
       

    }
}

fn write(path:&str,user:User){
    let content = fs::read_to_string(path);
    match &content {
        Ok(content)=>{
            let mut users = fetch_user(content);
            users.push(user);
            let json_data= serde_json::to_string_pretty(&users).unwrap();
            let mut file = File::create(path).unwrap();
            file.write_all(json_data.as_bytes()).unwrap();
        }
        Err(_)=>{
            let mut file = File::create(path).unwrap();
            let mut users:Vec<User> = vec![];
            users.push(user);
            let json_data= serde_json::to_string_pretty(&users).unwrap();
            file.write_all(json_data.as_bytes()).unwrap();
        }
    }
    println!("> User saved !")
    

}

fn read(path: &str){
    let content = fs::read_to_string(path);
    match &content {
        Ok(content)=>{
            let users = fetch_user(content);
            for user in users {
               pretty_print(&user);
            }
        }
        Err(_)=>{
            println!("Oops error occured while reading the data");
        }
    }
}

fn fetch_user(content : &String)-> Vec<User>{
    let path = "./json_database/db.json";
    let data = fs::read_to_string(path).unwrap();
    if data.len()==0{
       let users:Vec<User>= vec![];
       return users;
    }
    let users= serde_json::from_str(&content).unwrap();
    users
    
}

fn pretty_print(user:&User){
    println!("Name : {}",user.name);
    println!("Surname : {}",user.surname);
    println!("City : {}",user.city);
    println!("State : {}",user.state);
    println!("Country : {}",user.country);
    println!("----------------------------------------------------")
}

fn clear(){
    process::Command::new("clear").status().unwrap();
}

fn add_user(path: &str){
    let name = get_user_input(String::from("Enter user name"));
    let surname = get_user_input(String::from("Enter user surname"));
    let city = get_user_input(String::from("Enter user city"));
    let state = get_user_input(String::from("Enter user state"));
    let country = get_user_input(String::from("Enter user country"));
    let user = User{
        name:name,
        surname:surname,
        city:city,
        state:state,
        country:country
    };
    write(path, user);

}


fn get_user_input(question:String)->String{
    print!("> {}: ",question);
    io::stdout().flush().unwrap();
    let mut user_data = String::new();
    io::stdin().read_line(&mut user_data).unwrap();
    String::from(user_data.trim())


}