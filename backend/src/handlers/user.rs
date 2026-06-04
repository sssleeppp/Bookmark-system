use crate::db::DbPool;
use crate::models::User;
use crate::result::ApiResult;
use axum::extract::State;
use axum::Json;

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(State(db): State<DbPool>, Json(req): Json<LoginRequest>) -> ApiResult<User> {
    let conn = db.lock().unwrap();
    let result = conn.query_row(
        "SELECT id, username, password FROM user WHERE username = ?1 AND password = ?2",
        rusqlite::params![req.username, req.password],
        |row| {
            Ok(User {
                id: Some(row.get(0)?),
                username: row.get(1)?,
                password: row.get(2)?,
            })
        },
    );
    match result {
        Ok(user) => ApiResult::success(user),
        Err(_) => ApiResult::error("账号或密码错误"),
    }
}

pub async fn register(State(db): State<DbPool>, Json(user): Json<User>) -> ApiResult<()> {
    let conn = db.lock().unwrap();
    match conn.execute(
        "INSERT INTO user (username, password) VALUES (?1, ?2)",
        rusqlite::params![user.username, user.password],
    ) {
        Ok(_) => ApiResult::success_empty(),
        Err(e) => ApiResult::error(format!("注册失败: {}", e)),
    }
}
