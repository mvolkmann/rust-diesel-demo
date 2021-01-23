use crate::schema::dogs;

#[derive(Queryable)]
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
