use diesel::deserialize::Queryable;

#[derive(Queryable)]
pub struct Dog {
    pub id: i32,
    pub name: String,
    pub breed: String,
}
