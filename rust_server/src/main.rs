use std::{fs::{self, File}, io::Write};
use serde::{Serialize, Deserialize};
use serde_json;
use std::io;

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
    let users : Vec<User> = serde_json::from_str(&content).unwrap();
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

// Will be using code like this to make our custom command line 

// loop {
//     print!("Enter your command (type 'exit' to quit): ");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let input = input.trim(); // Remove any extra spaces/newlines
//     if input == "exit" {
//         println!("Exiting program...");
//         break;
//     }
//     println!("You entered: {}", input);
// }