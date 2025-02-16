use std::{fs::{self, File}, hash::{DefaultHasher, Hash, Hasher}, io::Write};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io;
use std::process;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug,Hash)]
struct User {
    id:Option<u64>,
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
         "find"=>find(),
         "delete"=>delete(),
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


fn get_content()->String{
    let path = "./json_database/db.json";
    let content = fs::read_to_string(path);
    match  content {
            Ok(content)=> {return content}
            Err(err)=>panic!("{}",err)
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

fn add_user(path: &str){
    let name = get_user_input(String::from("Enter user name"));
    let surname = get_user_input(String::from("Enter user surname"));
    let city = get_user_input(String::from("Enter user city"));
    let state = get_user_input(String::from("Enter user state"));
    let country = get_user_input(String::from("Enter user country"));
    let user = User{
        id:None,
        name:name.clone(),
        surname:surname.clone(),
        city:city.clone(),
        state:state.clone(),
        country:country.clone()
    };
    let id = generate_id(&user);
    let user = User{
        id:Some(id),
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


fn generate_id<T:Hash>(t:&T)->u64{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


fn find(){
    print!("> Enter id : ");
    io::stdout().flush().unwrap();
    let mut id =  String::new();
    io::stdin().read_line(& mut id).unwrap();
    let user = find_user_by_id(String::from(id.trim()));
    match user {
        Ok(user)=>{
            println!("---------------------------------------------------------");
            println!("{}", serde_json::to_string_pretty(&user).unwrap());
            println!("---------------------------------------------------------");
        }
        Err(err) =>println!("> Error occured {}",err)
    }

}

fn find_user_by_id(id:String)->Result<User,Box<dyn Error>>{
    let content = get_content();
    let users = fetch_user(&content);
    if  users.len()==0{
        return Err("Database is empty".into())
    }
    let id_u64 =  match id.as_str().parse::<u64>(){
        Ok(id)=>id,
        Err(_)=> {return  Err("Invalid id".into());}
    };

    for user in users {
        let user_id = user.id.unwrap();
        if user_id==id_u64{
            return Ok(user);
        }
    }
   Err("User not found".into())
}


fn delete(){
    print!("> Enter id : ");
    io::stdout().flush().unwrap();
    let mut id =  String::new();
    io::stdin().read_line(& mut id).unwrap();  
    let result  = delete_user_by_id(String::from(id.trim()));
    match result {
        Ok(_)=>{
            println!("> User deleted successfully")
        }
        Err(err)=>{
            println!("> Error occured : {}",err)
        }
    }
   

}


fn delete_user_by_id(id:String)->Result<(),Box<dyn Error>>{
    let path = "./json_database/db.json";
    let id_u64 = id.as_str().parse::<u64>().unwrap();
    let content = get_content();
    let mut users = fetch_user(&content);
    let previous_length = users.len();
    if previous_length==0 {
        return Err("Database is empty".into());
    }
    users.retain(|u| u.id.unwrap()!=id_u64);
    if users.len()==previous_length{
        return Err("User ID not found".into());
    }
    let json_data= serde_json::to_string_pretty(&users).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();   
    Ok(()) 

}

