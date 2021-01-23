#[table_name = "dogs"]
#[derive(Insertable, Queryable)]
pub struct Dog {
    pub id: i32,
    pub name: String,
    pub breed: String,
}
