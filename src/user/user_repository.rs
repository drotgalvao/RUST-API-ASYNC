// user/user_repository.rs
// use crate::db::connect;
use crate::db::PgPool;
use crate::user::user_dtos::CreateUserReqDto;
use crate::user::user_dtos::CreateUserResDto;



pub async fn create_user(
    pool: &PgPool,
    user_dto: CreateUserReqDto,
) -> Result<CreateUserResDto, String> {


    // let status = pool.status();
    // println!("Available connections: {}", status.available);
    // println!("Max connections: {}", status.max_size);
    // println!("Total connections: {}", status.size);
    // println!("Waiting connections: {}", status.waiting);
    
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => return Err(format!("Failed to connect to database: {}", err)),
    };

    let query = "INSERT INTO users (name, age) VALUES ($1, $2) RETURNING id";

    let row = match client.query_one(query, &[&user_dto.name, &user_dto.age]).await {
        Ok(row) => row,
        Err(err) => return Err(format!("Failed to insert user: {}", err)),
    };

    let user_id: i32 = row.get("id");

    let user_res_dto = CreateUserResDto {
        id: user_id,
        name: user_dto.name,
        age: user_dto.age,
    };

    Ok(user_res_dto)
}