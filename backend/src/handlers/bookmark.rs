use crate::db::DbPool;
use crate::models::{Bookmark, Category};
use crate::result::ApiResult;
use axum::{
    extract::{Multipart, Query, State},
    http::header,
    response::IntoResponse,
    Json,
};
use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ListParams {
    #[serde(rename = "userId")]
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct ExportParams {
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_format() -> String {
    "json".to_string()
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i64,
}

pub async fn list(
    State(db): State<DbPool>,
    Query(params): Query<ListParams>,
) -> ApiResult<Vec<Bookmark>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, title, url, category_id, user_id FROM bookmark WHERE user_id = ?1")
        .unwrap();
    let bookmarks: Vec<Bookmark> = stmt
        .query_map(rusqlite::params![params.user_id], |row| {
            Ok(Bookmark {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                url: row.get(2)?,
                category_id: row.get(3)?,
                user_id: row.get(4)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    ApiResult::success(bookmarks)
}

pub async fn add(State(db): State<DbPool>, Json(bookmark): Json<Bookmark>) -> ApiResult<Bookmark> {
    let conn = db.lock().unwrap();

    if let Some(id) = bookmark.id {
        conn.execute(
            "UPDATE bookmark SET title=?1, url=?2, category_id=?3, user_id=?4 WHERE id=?5",
            rusqlite::params![
                bookmark.title,
                bookmark.url,
                bookmark.category_id,
                bookmark.user_id,
                id,
            ],
        )
        .unwrap();
        ApiResult::success(bookmark)
    } else {
        conn.execute(
            "INSERT INTO bookmark (title, url, category_id, user_id) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                bookmark.title,
                bookmark.url,
                bookmark.category_id,
                bookmark.user_id,
            ],
        )
        .unwrap();
        let id = conn.last_insert_rowid();
        ApiResult::success(Bookmark {
            id: Some(id),
            ..bookmark
        })
    }
}

pub async fn delete(State(db): State<DbPool>, Json(req): Json<DeleteRequest>) -> ApiResult<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "DELETE FROM bookmark WHERE id = ?1",
        rusqlite::params![req.id],
    )
    .unwrap();
    ApiResult::success_empty()
}

pub async fn export_bookmarks(
    State(db): State<DbPool>,
    Query(params): Query<ExportParams>,
) -> impl IntoResponse {
    let conn = db.lock().unwrap();
    let (content, filename, content_type) = if params.format.eq_ignore_ascii_case("html") {
        let html = export_as_html(&conn, params.user_id);
        (
            html,
            "bookmarks.html".to_string(),
            "text/html; charset=UTF-8".to_string(),
        )
    } else {
        let json = export_as_json(&conn, params.user_id);
        (
            json,
            "bookmarks.json".to_string(),
            "application/json; charset=UTF-8".to_string(),
        )
    };

    let bytes = content.into_bytes();
    (
        [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        bytes,
    )
        .into_response()
}

pub async fn import_bookmarks(
    State(db): State<DbPool>,
    mut multipart: Multipart,
) -> ApiResult<i64> {
    let mut user_id = None;
    let mut format = "json".to_string();
    let mut file_content = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("userId") => {
                if let Ok(text) = field.text().await {
                    user_id = text.parse::<i64>().ok();
                }
            }
            Some("format") => {
                if let Ok(text) = field.text().await {
                    format = text;
                }
            }
            Some("file") => {
                file_content = field.text().await.ok();
            }
            _ => {}
        }
    }

    let user_id = match user_id {
        Some(id) => id,
        None => return ApiResult::error("缺少 userId 参数"),
    };

    let content = match file_content {
        Some(c) => c,
        None => return ApiResult::error("未上传文件"),
    };

    let conn = db.lock().unwrap();

    if format.eq_ignore_ascii_case("html") {
        match import_from_html(&conn, &content, user_id) {
            Ok(count) => ApiResult::success(count),
            Err(e) => ApiResult::error(format!("导入失败: {}", e)),
        }
    } else {
        match import_from_json(&conn, &content, user_id) {
            Ok(count) => ApiResult::success(count),
            Err(e) => ApiResult::error(format!("导入失败: {}", e)),
        }
    }
}

fn export_as_json(conn: &rusqlite::Connection, user_id: i64) -> String {
    let categories = list_categories(conn, user_id);
    let bookmarks = list_bookmarks_raw(conn, user_id);

    let json = serde_json::json!({
        "categories": categories,
        "bookmarks": bookmarks,
    });

    serde_json::to_string(&json).unwrap_or_else(|_| "{}".to_string())
}

fn list_categories(conn: &rusqlite::Connection, user_id: i64) -> Vec<Category> {
    let mut stmt = conn
        .prepare("SELECT id, name, user_id, parent_id, sort_order FROM category WHERE user_id = ?1")
        .unwrap();
    stmt.query_map(rusqlite::params![user_id], |row| {
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
    .collect()
}

fn list_bookmarks_raw(conn: &rusqlite::Connection, user_id: i64) -> Vec<Bookmark> {
    let mut stmt = conn
        .prepare("SELECT id, title, url, category_id, user_id FROM bookmark WHERE user_id = ?1")
        .unwrap();
    stmt.query_map(rusqlite::params![user_id], |row| {
        Ok(Bookmark {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            url: row.get(2)?,
            category_id: row.get(3)?,
            user_id: row.get(4)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

fn export_as_html(conn: &rusqlite::Connection, user_id: i64) -> String {
    let categories = list_categories(conn, user_id);
    let bookmarks = list_bookmarks_raw(conn, user_id);

    let mut children_map: HashMap<i64, Vec<&Category>> = HashMap::new();
    let mut bookmark_map: HashMap<i64, Vec<&Bookmark>> = HashMap::new();

    for cat in &categories {
        if let Some(parent_id) = cat.parent_id {
            children_map.entry(parent_id).or_default().push(cat);
        }
    }
    for bm in &bookmarks {
        if let Some(cat_id) = bm.category_id {
            bookmark_map.entry(cat_id).or_default().push(bm);
        }
    }

    let mut sb = String::new();
    sb.push_str("<!DOCTYPE NETSCAPE-Bookmark-file-1>\n");
    sb.push_str(
        "<!-- This is an automatically generated file.\n     It will be read and overwritten.\n     DO NOT EDIT! -->\n",
    );
    sb.push_str("<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n");
    sb.push_str("<TITLE>Bookmarks</TITLE>\n");
    sb.push_str("<H1>书签菜单</H1>\n");
    sb.push_str("<DL><p>\n");

    for cat in &categories {
        if cat.parent_id.is_none() {
            build_category_html(&mut sb, cat, &children_map, &bookmark_map, 1);
        }
    }

    for bm in &bookmarks {
        if bm.category_id.is_none() {
            append_indent(&mut sb, 1);
            sb.push_str(&format!(
                "<DT><A HREF=\"{}\">{}</A>\n",
                escape_html(&bm.url),
                escape_html(&bm.title)
            ));
        }
    }

    sb.push_str("</DL><p>\n");
    sb
}

fn build_category_html(
    sb: &mut String,
    category: &Category,
    children_map: &HashMap<i64, Vec<&Category>>,
    bookmark_map: &HashMap<i64, Vec<&Bookmark>>,
    depth: i32,
) {
    append_indent(sb, depth);
    sb.push_str(&format!("<DT><H3>{}</H3>\n", escape_html(&category.name)));
    append_indent(sb, depth);
    sb.push_str("<DL><p>\n");

    if let Some(cat_bookmarks) = bookmark_map.get(&category.id.unwrap()) {
        for bm in cat_bookmarks {
            append_indent(sb, depth + 1);
            sb.push_str(&format!(
                "<DT><A HREF=\"{}\">{}</A>\n",
                escape_html(&bm.url),
                escape_html(&bm.title)
            ));
        }
    }

    if let Some(children) = children_map.get(&category.id.unwrap()) {
        for child in children {
            build_category_html(sb, child, children_map, bookmark_map, depth + 1);
        }
    }

    append_indent(sb, depth);
    sb.push_str("</DL><p>\n");
}

fn append_indent(sb: &mut String, depth: i32) {
    for _ in 0..depth {
        sb.push_str("    ");
    }
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn import_from_json(conn: &rusqlite::Connection, json: &str, user_id: i64) -> Result<i64, String> {
    let data: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("JSON 格式错误: {}", e))?;

    struct CatInfo {
        old_id: i64,
        name: String,
        parent_id: Option<i64>,
        sort_order: i32,
    }
    let mut cat_infos: Vec<CatInfo> = Vec::new();

    if let Some(categories) = data["categories"].as_array() {
        for cm in categories {
            cat_infos.push(CatInfo {
                old_id: cm["id"].as_i64().unwrap_or(0),
                name: cm["name"].as_str().unwrap_or("").to_string(),
                parent_id: cm["parentId"].as_i64(),
                sort_order: cm["sortOrder"].as_i64().unwrap_or(0) as i32,
            });
        }
    }

    let mut category_id_map: HashMap<i64, i64> = HashMap::new();

    for info in &cat_infos {
        let existing = get_category_by_name(conn, &info.name, user_id);
        let new_id = if let Some(cat) = existing {
            cat.id.unwrap()
        } else {
            conn.execute(
                "INSERT INTO category (name, user_id, sort_order) VALUES (?1, ?2, ?3)",
                rusqlite::params![info.name, user_id, info.sort_order],
            )
            .map_err(|e| e.to_string())?;
            conn.last_insert_rowid()
        };
        category_id_map.insert(info.old_id, new_id);
    }

    for info in &cat_infos {
        if let Some(pid) = info.parent_id {
            if let Some(&new_pid) = category_id_map.get(&pid) {
                if let Some(&new_id) = category_id_map.get(&info.old_id) {
                    conn.execute(
                        "UPDATE category SET parent_id = ?1 WHERE id = ?2",
                        rusqlite::params![new_pid, new_id],
                    )
                    .map_err(|e| e.to_string())?;
                }
            }
        }
    }

    let mut count = 0i64;
    if let Some(bookmarks) = data["bookmarks"].as_array() {
        for bm in bookmarks {
            let url = bm["url"].as_str().unwrap_or("").to_string();
            let title = bm["title"].as_str().unwrap_or("").to_string();
            let category_id = bm["categoryId"].as_i64();
            let mapped_cat = category_id.and_then(|cid| category_id_map.get(&cid).copied());

            let bookmark = Bookmark {
                id: None,
                title,
                url,
                category_id: mapped_cat,
                user_id,
            };
            if let Some(c) = insert_bookmark_if_new(conn, &bookmark, user_id)? {
                count += c;
            }
        }
    }

    Ok(count)
}

fn insert_bookmark_if_new(
    conn: &rusqlite::Connection,
    bookmark: &Bookmark,
    user_id: i64,
) -> Result<Option<i64>, String> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM bookmark WHERE url = ?1 AND user_id = ?2",
            rusqlite::params![bookmark.url, user_id],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);

    if !exists {
        conn.execute(
            "INSERT INTO bookmark (title, url, category_id, user_id) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                bookmark.title,
                bookmark.url,
                bookmark.category_id,
                user_id
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(Some(1))
    } else {
        Ok(None)
    }
}

fn get_category_by_name(conn: &rusqlite::Connection, name: &str, user_id: i64) -> Option<Category> {
    conn.query_row(
        "SELECT id, name, user_id, parent_id, sort_order FROM category WHERE name = ?1 AND user_id = ?2",
        rusqlite::params![name, user_id],
        |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                user_id: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
            })
        },
    )
    .ok()
}

fn import_from_html(conn: &rusqlite::Connection, html: &str, user_id: i64) -> Result<i64, String> {
    let document = Html::parse_document(html);
    let dl_selector = Selector::parse("dl").map_err(|e| e.to_string())?;

    let mut imported = Vec::new();
    for dl in document.select(&dl_selector) {
        parse_dl_element(&dl, None, user_id, conn, &mut imported)?;
    }

    let mut count = 0i64;
    for bookmark in &imported {
        if let Some(c) = insert_bookmark_if_new(conn, bookmark, user_id)? {
            count += c;
        }
    }

    Ok(count)
}

fn parse_dl_element(
    dl: &scraper::ElementRef<'_>,
    parent_category_id: Option<i64>,
    user_id: i64,
    conn: &rusqlite::Connection,
    result: &mut Vec<Bookmark>,
) -> Result<(), String> {
    let dt_selector = Selector::parse("dt").map_err(|e| e.to_string())?;
    let h3_selector = Selector::parse("h3").map_err(|e| e.to_string())?;
    let a_selector = Selector::parse("a").map_err(|e| e.to_string())?;
    let dl_selector = Selector::parse("dl").map_err(|e| e.to_string())?;

    for dt in dl.select(&dt_selector) {
        if let Some(h3) = dt.select(&h3_selector).next() {
            let folder_name: String = h3.text().collect();
            let folder_name = folder_name.trim().to_string();

            let cat = get_category_by_name(conn, &folder_name, user_id);
            let cat_id = if let Some(c) = cat {
                c.id.unwrap()
            } else {
                conn.execute(
                    "INSERT INTO category (name, user_id, parent_id, sort_order) VALUES (?1, ?2, ?3, 0)",
                    rusqlite::params![folder_name, user_id, parent_category_id],
                )
                .map_err(|e| e.to_string())?;
                conn.last_insert_rowid()
            };

            if let Some(sub_dl) = dt.select(&dl_selector).next() {
                parse_dl_element(&sub_dl, Some(cat_id), user_id, conn, result)?;
            }
        } else if let Some(a) = dt.select(&a_selector).next() {
            let url = a.value().attr("href").unwrap_or("").to_string();
            let title: String = a.text().collect();
            let title = title.trim().to_string();

            if !url.is_empty() {
                result.push(Bookmark {
                    id: None,
                    title,
                    url,
                    category_id: parent_category_id,
                    user_id,
                });
            }
        }
    }

    Ok(())
}
