use ids706_indvidual2::{delete_bob, run}; 
use rusqlite::Connection;

fn main() {
    // Create the SQLite connection (persistent database)
    let conn = Connection::open("wealth_db.db").expect("Failed to open database");
    println!("Database connected.");

    // Pass the connection to the run function to handle the initial setup and insertion
    if let Err(err) = run(&conn) {
        eprintln!("Error: {}", err);
    }

    // Optionally call the delete_bob function to handle the deletion of Bob
    if let Err(err) = delete_bob(&conn) {
        eprintln!("Error deleting Bob: {}", err);
    }

    println!("Database connection closed.");
}
