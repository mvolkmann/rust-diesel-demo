#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::result::Error;
use models::*;

// Deletes all rows from the dogs table.
fn delete_dogs(conn: &PgConnection) -> Result<usize, Error> {
    diesel::delete(schema::dogs::table).execute(conn)
}

// Gets a connection to the "animals" database that contains a "dogs" table.
fn get_connection() -> ConnectionResult<PgConnection> {
    use dotenv::dotenv;
    use std::env;
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url)
}

// Inserts a row in the "dogs" table and returns its id.
fn insert_dog(conn: &PgConnection, name: &str, breed: &str) -> Result<i32, Error> {
    let dog = NewDog {
        name: name.to_string(),
        breed: breed.to_string(),
    };
    let id: i32 = diesel::insert_into(schema::dogs::table)
        .values(&dog)
        .returning(schema::dogs::id)
        .get_result(conn)?;
    Ok(id)
}

// Outputs information about each row in the "dogs" table.
fn report_dogs(conn: &PgConnection) {
    let results = schema::dogs::dsl::dogs
        .load::<Dog>(conn)
        .expect("error loading dogs");

    for dog in results {
        println!("{} is a {} (id {}).", dog.name, dog.breed, dog.id);
    }
}

// Updates the "dogs" table row with a given id.
fn update_dog(conn: &PgConnection, id: i32, name: &str, breed: &str) -> Result<usize, Error> {
    let dog = Dog {
        id,
        name: name.to_string(),
        breed: breed.to_string(),
    };
    diesel::update(&dog).set(&dog).execute(conn)
}

// The return type specified here allows using "?" for error handling
// regardless of the specific kinds of errors that occur.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;

    delete_dogs(&conn)?;

    let dogs = [
        ("Maisey", "Treeing Walker Coonhound"),
        ("Ramsay", "Native American Indian Dog"),
        ("Comet", "Whippet"),
    ];
    for dog in &dogs {
        insert_dog(&conn, dog.0, dog.1)?;
    }

    let id = insert_dog(&conn, "Oscar", "German Shorthaired Pointer")?;

    update_dog(&conn, id, "Oscar Wilde", "German Shorthaired Pointer")?;

    report_dogs(&conn);

    Ok(())
}
