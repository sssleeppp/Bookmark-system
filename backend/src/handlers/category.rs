use crate::db::DbPool;
use crate::models::Category;
use crate::result::ApiResult;
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListParams {
    #[serde(rename = "userId")]
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i64,
}

pub async fn list(
    State(db): State<DbPool>,
    Query(params): Query<ListParams>,
) -> ApiResult<Vec<Category>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, name, user_id, parent_id, sort_order
             FROM category WHERE user_id = ?1 ORDER BY sort_order ASC",
        )
        .unwrap();

    let categories: Vec<Category> = stmt
        .query_map(rusqlite::params![params.user_id], |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                user_id: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    ApiResult::success(categories)
}

pub async fn add(State(db): State<DbPool>, Json(category): Json<Category>) -> ApiResult<Category> {
    let conn = db.lock().unwrap();

    if let Some(id) = category.id {
        conn.execute(
            "UPDATE category SET name=?1, user_id=?2, parent_id=?3, sort_order=?4 WHERE id=?5",
            rusqlite::params![
                category.name,
                category.user_id,
                category.parent_id,
                category.sort_order,
                id
            ],
        )
        .unwrap();
        ApiResult::success(category)
    } else {
        conn.execute(
            "INSERT INTO category (name, user_id, parent_id, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                category.name,
                category.user_id,
                category.parent_id,
                category.sort_order
            ],
        )
        .unwrap();
        let id = conn.last_insert_rowid();
        ApiResult::success(Category {
            id: Some(id),
            ..category
        })
    }
}

pub async fn batch_update(
    State(db): State<DbPool>,
    Json(categories): Json<Vec<Category>>,
) -> ApiResult<()> {
    let conn = db.lock().unwrap();
    conn.execute("BEGIN", []).unwrap();
    for cat in &categories {
        if let Some(id) = cat.id {
            conn.execute(
                "UPDATE category SET name=?1, user_id=?2, parent_id=?3, sort_order=?4 WHERE id=?5",
                rusqlite::params![cat.name, cat.user_id, cat.parent_id, cat.sort_order, id,],
            )
            .unwrap();
        }
    }
    conn.execute("COMMIT", []).unwrap();
    ApiResult::success_empty()
}

pub async fn delete(State(db): State<DbPool>, Json(req): Json<DeleteRequest>) -> ApiResult<()> {
    let conn = db.lock().unwrap();
    let mut ids = Vec::new();
    collect_category_ids(&conn, req.id, &mut ids);

    for id in &ids {
        conn.execute(
            "DELETE FROM bookmark WHERE category_id = ?1",
            rusqlite::params![id],
        )
        .unwrap();
        conn.execute("DELETE FROM category WHERE id = ?1", rusqlite::params![id])
            .unwrap();
    }

    ApiResult::success_empty()
}

fn collect_category_ids(conn: &rusqlite::Connection, parent_id: i64, ids: &mut Vec<i64>) {
    ids.push(parent_id);
    let mut stmt = conn
        .prepare("SELECT id FROM category WHERE parent_id = ?1")
        .unwrap();
    let children: Vec<i64> = stmt
        .query_map(rusqlite::params![parent_id], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for child_id in children {
        collect_category_ids(conn, child_id, ids);
    }
}
