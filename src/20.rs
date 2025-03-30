fn main() {
    // Example Rust code snippet

    let user_data: Vec<&str> = vec!["name".to_string(), "age".to_string(), "city".to_string()];
    
    println!("User data:");
    for (key, value) in user_data.iter() {
        match value.as_str() {
            name => println!("Name: {}", name),
            age => println!("Age: {}", age),
            city => println!("City: {}", city),
        }
    }
}
