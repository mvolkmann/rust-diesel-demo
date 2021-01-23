use crate::schema::dogs;

// Implementing the "Identifiable" trait means the struct
// represents a single row in a database table and, by default,
// has an "id" field that is its primary key.
// The associated table name defaults to the lowercase version
// of the struct name with an "s" on the end.
// If this is not the table name, specify the "table_name" attribute
// as shown above the "NewDog" struct that follows.
#[derive(Identifiable, Queryable)]
pub struct Dog {
    pub id: i32,
    pub name: String,
    pub breed: String,
}

#[table_name = "dogs"]
#[derive(Insertable)]
pub struct NewDog {
    pub name: String,
    pub breed: String,
}
