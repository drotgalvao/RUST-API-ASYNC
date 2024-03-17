// user/user_model.rs


use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}