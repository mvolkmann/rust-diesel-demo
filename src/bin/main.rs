extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::*;

fn main() {
    use diesel_demo::schema::dogs::dsl::*;

    let connection = establish_connection();
    let results = dogs
        //.filter(published.eq(true))
        //.limit(5)
        .load::<Dog>(&connection)
        .expect("error loading dogs");

    println!("Displaying {} dogs", results.len());
    for dog in results {
        println!("{} is a {}.", dog.name, dog.breed);
    }
}
