extern crate diesel;
extern crate diesel_demo;

use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::dogs::dsl::*;

    let connection = establish_connection();
    //TODO: The next line gives the error: the trait
    //TODO: `diesel::deserialize::FromSql<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>`
    //TODO: is not implemented for `*const str`
    let results = dogs.load::<Dog>(&connection).expect("error loading dogs");

    println!("Displaying {} dogs", results.len());
    for dog in results {
        println!("{} is a {}.", dog.name, dog.breed);
    }
}
