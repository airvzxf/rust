use rusqlite::{Connection, Error, Statement};

fn main() {
    println!("Hello, world!");
    let connection = create_database();
    println!("{:#?}", connection);

    create_table(&connection);
    insert_user(&connection, "Israel Roldan");
    insert_user(&connection, "Selene Rivas");
    insert_user(&connection, "Shavidenka Roldan");
    insert_user(&connection, "Zxf Roldan");

    let user = get_user(&connection, 1);
    match user {
        Ok(user) => print!("user: {}", user),
        Err(_) => (),
    };
}

fn get_user(connection: &Connection, user_id: i32) -> Result<String, Error> {
    let sql_query: &str = "SELECT name FROM user WHERE id=?1";

    let mut sql_statement: Statement = connection.prepare(sql_query)?;
    let users = sql_statement.query_map([user_id], |row| {
        let name: String = row.get(0)?;
        Ok(name)
    })?;

    for user in users {
        return Ok(user.unwrap());
    }

    Ok("User not found.".to_string())
}

fn insert_user(connection: &Connection, user: &str) {
    let sql_query: &str = "INSERT INTO user (name) VALUES (?1)";
    let params: &[&str; 1] = &[user];
    let result: Result<usize, Error> = connection.execute(sql_query, params);
    match result {
        Ok(_) => println!("Inserted user: {}", user),
        Err(error) => panic!("ERROR: Inserting user {}. {}", user, &error),
    }
}

fn create_table(connection: &Connection) {
    let sql = "CREATE TABLE IF NOT EXISTS user (
        id	    INTEGER NOT NULL UNIQUE,
        name	TEXT NOT NULL,
        PRIMARY KEY(id AUTOINCREMENT)
    );";

    let result: Result<usize, Error> = connection.execute(sql, []);

    match result {
        Ok(_) => println!("Table created"),
        Err(error) => panic!("ERROR: Creating the table. {}", &error),
    }
}

fn create_database() -> Connection {
    let database_path: &str = "user.sqlite";
    let result: Result<Connection, Error> = Connection::open(database_path);
    match result {
        Ok(_) => {
            println!("Connected to the database");
            return result.unwrap();
        }
        Err(error) => panic!("ERROR: Connecting the database. {}", &error),
    }
}
