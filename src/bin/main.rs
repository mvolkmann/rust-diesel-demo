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

fn insert_dog(conn: &PgConnection, name: &str, breed: &str) {
    let dog = Dog {
        id: 0,
        name: name.to_string(),
        breed: breed.to_string(),
    };
    diesel::insert_into(dogs::table)
        .values(&dog)
        //.get_result(conn)
        .execute(conn)
        .expect("error inserting dog")
}

/*
fn insert_dogs(conn: &PgConnection) {
    let dogs = [
        ("Maisey", "Treeing Walker Coonhound"),
        ("Ramsay", "Native American Indian Dog"),
        ("Comet", "Whippet"),
    ];
    for dog in &dogs {
        insert_dog(conn, dog.0, dog.1)?;
    }
    Ok(dogs.len() as u64) // # of inserted rows
}

fn report_dogs(conn: &Connection) {
    let results = dogs.load::<Dog>(&conn).expect("error loading dogs");

    println!("Displaying {} dogs", results.len());
    for dog in results {
        println!("{} is a {}.", dog.name, dog.breed);
    }
}

fn update_dog(conn: &Connection, id: i32, name: String, breed: String) {
    diesel::update(dogs.find(id))
        .set(breed.eq(breed))
        .set(name.eq(name))
        .get_result::<Dog>(&conn)
        .expect(&format!("unable to find dog {}", id));
}
*/

fn main() {
    use self::schema::dogs::dsl::*;

    let conn = establish_connection();
    delete_dogs(&conn);
    insert_dog(&conn, "Comet", "Whippet");

    /*
    insert_dogs(&conn);

    let id = insert_dog("Oscar", "German Shorthaired Pointer");
    update_dog(&conn, id, "Oscar Wilde", "German Shorthaired Pointer");

    report_dogs(&conn);
    */
}
