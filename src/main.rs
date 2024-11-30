use std::env;

use dotenvy::dotenv;

fn main() {
    dotenv().ok(); // Loads the .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database path: {}", database_url);
}
