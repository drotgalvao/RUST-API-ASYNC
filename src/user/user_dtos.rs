// user/user_dtos.rs
use serde::{Deserialize, Serialize};

// DTO para requisições de criação de usuário
#[derive(Debug, Deserialize)]
pub struct CreateUserReqDto {
    pub name: String,
    pub age: i32,
}

// DTO para respostas de criação de usuário
#[derive(Debug, Serialize)]
pub struct CreateUserResDto {
    pub id: i32, // Changed from Uuid to i32
    pub name: String,
    pub age: i32,
}

// DTO para respostas de busca de usuário
#[derive(Debug, Serialize)]
pub struct GetUserResDto {
    pub id: i32, // Changed from Uuid to i32
    pub name: String,
    pub age: i32,
}