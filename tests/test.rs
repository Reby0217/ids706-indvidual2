#[cfg(test)]
mod tests {
    use ids706_indvidual2::{delete_bob, run};
    use rusqlite::{Connection, Result};

    #[test]
    fn test_create_table() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='wealthiest_people'",
            )
            .unwrap();
        let table_exists: Result<Option<String>> = stmt.query_row([], |row| row.get(0));
        assert!(table_exists.unwrap().is_some());
    }

    #[test]
    fn test_insert_data() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM wealthiest_people")
            .unwrap();
        let count: i32 = stmt.query_row([], |row| row.get(0)).unwrap();

        // Assert that 3 records are inserted
        assert_eq!(count, 3);
    }

    #[test]
    fn test_update_data() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();

        let updated_net_worth: f64 = conn
            .query_row(
                "SELECT net_worth FROM wealthiest_people WHERE name = 'Charlie'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(updated_net_worth, 180.0);
    }

    #[test]
    fn test_delete_data() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();
        delete_bob(&conn).unwrap();

        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM wealthiest_people WHERE name = 'Bob'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        // Assert that Bob's record is deleted
        assert_eq!(count, 0);
    }

    #[test]
    fn test_read_data() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();

        let mut stmt = conn
            .prepare("SELECT id, name, country, industry, net_worth, company FROM wealthiest_people")
            .unwrap();
        let people_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,    // id
                row.get::<_, String>(1)?, // name
                row.get::<_, String>(2)?, // country
                row.get::<_, String>(3)?, // industry
                row.get::<_, f64>(4)?,    // net_worth
                row.get::<_, String>(5)?, // company
            ))
        }).unwrap();

        let mut count = 0;
        for person in people_iter {
            let (id, name, country, industry, net_worth, company) = person.unwrap();
            match id {
                1 => {
                    assert_eq!(name, "Alice");
                    assert_eq!(country, "USA");
                    assert_eq!(industry, "Tech");
                    assert_eq!(net_worth, 100.0);
                    assert_eq!(company, "CompanyA");
                }
                2 => {
                    assert_eq!(name, "Bob");
                    assert_eq!(country, "UK");
                    assert_eq!(industry, "Finance");
                    assert_eq!(net_worth, 200.0);
                    assert_eq!(company, "CompanyB");
                }
                3 => {
                    assert_eq!(name, "Charlie");
                    assert_eq!(country, "Canada");
                    assert_eq!(industry, "Tech");
                    assert_eq!(net_worth, 180.0); // After update
                    assert_eq!(company, "CompanyC");
                }
                _ => panic!("Unexpected record"),
            }
            count += 1;
        }

        // Assert that 3 records are read
        assert_eq!(count, 3);
    }
}
