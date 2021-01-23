extern crate diesel;
extern crate diesel_demo;

use self::models::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_demo::*;
use schema::dogs;

fn delete_dogs(conn: &PgConnection) -> Result<usize, Error> {
    diesel::delete(dogs::table).execute(conn)
}

// This returns the id of the inserted row.
fn insert_dog(conn: &PgConnection, name: &str, breed: &str) -> Result<i32, Error> {
    let dog = NewDog {
        name: name.to_string(),
        breed: breed.to_string(),
    };
    let id: i32 = diesel::insert_into(dogs::table)
        .values(&dog)
        .returning(dogs::id)
        .get_result(conn)?;
    Ok(id)
}

fn insert_dogs(conn: &PgConnection) -> Result<usize, Error> {
    let dogs = [
        ("Maisey", "Treeing Walker Coonhound"),
        ("Ramsay", "Native American Indian Dog"),
        ("Comet", "Whippet"),
    ];
    for dog in &dogs {
        insert_dog(conn, dog.0, dog.1)?;
    }
    Ok(dogs.len()) // # of inserted rows
}

fn report_dogs(conn: &PgConnection) {
    let results = dogs::dsl::dogs
        .load::<Dog>(conn)
        .expect("error loading dogs");

    println!("Displaying {} dogs", results.len());
    for dog in results {
        println!("{} is a {}.", dog.name, dog.breed);
    }
}

fn update_dog(conn: &PgConnection, id: i32, name: &str, breed: &str) -> Result<usize, Error> {
    let dog = Dog {
        id,
        name: name.to_string(),
        breed: breed.to_string(),
    };
    diesel::update(&dog).set(&dog).execute(conn)
}

fn main() {
    let conn = establish_connection();
    delete_dogs(&conn).unwrap();
    insert_dogs(&conn).unwrap();

    if let Ok(id) = insert_dog(&conn, "Oscar", "German Shorthaired Pointer") {
        update_dog(&conn, id, "Oscar Wilde", "German Shorthaired Pointer").unwrap();
    } else {
        eprintln!("error inserting dog");
    }

    report_dogs(&conn);
}
