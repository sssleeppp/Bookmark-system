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
    if req.username.trim().is_empty() || req.password.is_empty() {
        return ApiResult::error("账号或密码不能为空");
    }
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
    let username = user.username.trim();
    if username.is_empty() || user.password.is_empty() {
        return ApiResult::error("账号或密码不能为空");
    }
    let conn = db.lock().unwrap();
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM user WHERE username = ?1",
            rusqlite::params![username],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);
    if exists {
        return ApiResult::error("用户名已存在");
    }
    match conn.execute(
        "INSERT INTO user (username, password) VALUES (?1, ?2)",
        rusqlite::params![username, user.password],
    ) {
        Ok(_) => ApiResult::success_empty(),
        Err(e) => ApiResult::error(format!("注册失败: {}", e)),
    }
}
