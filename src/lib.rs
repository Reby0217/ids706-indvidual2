extern crate rusqlite;
use rusqlite::{params, Connection, Result};

pub fn run(conn: &Connection) -> Result<()> {
    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wealthiest_people (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            country TEXT NOT NULL,
            industry TEXT NOT NULL,
            net_worth REAL NOT NULL,
            company TEXT NOT NULL
        )",
        params![],
    )?;
    println!("Table 'wealthiest_people' created successfully.\n");

    // Clear the table before inserting new records
    conn.execute("DELETE FROM wealthiest_people", params![])?;
    println!("Table 'wealthiest_people' cleared.\n");

    // Insert records into the table
    let people = vec![
        (1, "Alice", "USA", "Tech", 100.0, "CompanyA"),
        (2, "Bob", "UK", "Finance", 200.0, "CompanyB"),
        (3, "Charlie", "Canada", "Tech", 150.0, "CompanyC"),
    ];

    for person in &people {
        conn.execute(
            "INSERT INTO wealthiest_people (id, name, country, industry, net_worth, company) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![person.0, person.1, person.2, person.3, person.4, person.5],
        )?;
    }
    println!("{} records inserted successfully.\n", people.len());

    // Read Operation:
    let mut stmt = conn.prepare("SELECT id, name, country, industry, net_worth, company FROM wealthiest_people ORDER BY net_worth DESC")?;
    let people_iter = stmt.query_map(params![], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    println!("Wealthiest People Records sorted by net worth:");
    let mut count = 0;
    for person in people_iter {
        let (id, name, country, industry, net_worth, company) = person?;
        println!("{}, {}, {}, {}, {}, {}", id, name, country, industry, net_worth, company);
        count += 1;
    }
    println!("Retrieved {} records sorted by net worth.\n", count);

    // Update Charlie's net worth
    let new_net_worth = 180.0;
    conn.execute(
        "UPDATE wealthiest_people SET net_worth = ?1 WHERE name = ?2",
        params![new_net_worth, "Charlie"],
    )?;
    println!("Updated Charlie's net worth to {}.\n", new_net_worth);

    // Confirm the update by retrieving all records
    let mut stmt = conn.prepare("SELECT id, name, country, industry, net_worth, company FROM wealthiest_people")?;
    let people_iter = stmt.query_map(params![], |row| {
        Ok((
            row.get::<_, i32>(0)?,    // id
            row.get::<_, String>(1)?, // name
            row.get::<_, String>(2)?, // country
            row.get::<_, String>(3)?, // industry
            row.get::<_, f64>(4)?,    // net_worth
            row.get::<_, String>(5)?, // company
        ))
    })?;

    println!("Updated Wealthiest People Records:");
    let mut count = 0;
    for person in people_iter {
        let (id, name, country, industry, net_worth, company) = person?;
        println!("{}, {}, {}, {}, {}, {}", id, name, country, industry, net_worth, company);
        count += 1;
    }
    println!("Retrieved {} records after updating Charlie's net worth.\n", count);


    Ok(())
}

// New function to handle deletion
pub fn delete_bob(conn: &Connection) -> Result<()> {
    // Delete the record for Bob
    conn.execute(
        "DELETE FROM wealthiest_people WHERE name = 'Bob'",
        params![],
    )?;
    println!("Deleted Bob's record.\n");

    // Check records after deletion
    let mut stmt = conn.prepare("SELECT id, name, country, industry, net_worth, company FROM wealthiest_people")?;
    let people_iter = stmt.query_map(params![], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    println!("Wealthiest People Records after Bob's deletion:");
    let mut count = 0;
    for person in people_iter {
        let (id, name, country, industry, net_worth, company) = person?;
        println!("{}, {}, {}, {}, {}, {}", id, name, country, industry, net_worth, company);
        count += 1;
    }
    println!("Retrieved {} records.\n", count);

    Ok(()) 
}
