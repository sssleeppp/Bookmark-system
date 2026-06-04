# API 参考

Base URL: `http://localhost:8989`

所有接口返回统一格式 `{code, msg, data}`。成功 `code: 200`，失败 `code: 500`。

## 接口总览

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/user/login` | 登录 |
| POST | `/user/register` | 注册 |
| GET | `/category/list?userId=` | 分类列表 |
| POST | `/category/add` | 新增/更新分类 |
| POST | `/category/batchUpdate` | 批量更新排序 |
| POST | `/category/delete` | 删除分类（级联子孙 + 书签） |
| GET | `/bookmark/list?userId=` | 书签列表 |
| POST | `/bookmark/add` | 新增/更新书签 |
| POST | `/bookmark/delete` | 删除书签 |
| GET | `/bookmark/export?userId=&format=` | 导出文件下载 |
| POST | `/bookmark/import` | multipart 上传导入 |

## 数据库设计

3 张表，`user` 1:N `category`，`user` 1:N `bookmark`，`category` N:1 `bookmark`。

```sql
CREATE TABLE user (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE category (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL,
    user_id    INTEGER NOT NULL,
    parent_id  INTEGER,          -- NULL = 根节点
    sort_order INTEGER DEFAULT 0 -- 排序字段
);

CREATE TABLE bookmark (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    url         TEXT NOT NULL,
    category_id INTEGER,
    user_id     INTEGER NOT NULL
);
```

建库行为：每次后端重启执行 `DROP TABLE IF EXISTS` → `CREATE TABLE` → 种子用户 `admin / 123456`。

---

## 用户

### POST /user/login

**Request:**
```json
{ "username": "admin", "password": "123456" }
```

**Response (200):**
```json
{
  "code": 200,
  "msg": "操作成功",
  "data": { "id": 1, "username": "admin", "password": "123456" }
}
```

**Response (500):**
```json
{ "code": 500, "msg": "账号或密码错误", "data": null }
```

### POST /user/register

**Request:**
```json
{ "username": "test", "password": "123456" }
```

**Response:**
```json
{ "code": 200, "msg": "操作成功", "data": null }
```

---

## 分类

### GET /category/list?userId={id}

返回该用户的分类列表，按 `sortOrder` 升序。

**Response:**
```json
{
  "code": 200,
  "msg": "操作成功",
  "data": [
    { "id": 1, "name": "开发工具", "userId": 1, "parentId": null, "sortOrder": 0 },
    { "id": 2, "name": "前端", "userId": 1, "parentId": 1, "sortOrder": 1 }
  ]
}
```

### POST /category/add

若 `id` 非空则为更新，否则为新建。

**Request (新增):**
```json
{ "name": "新分类", "userId": 1, "parentId": null, "sortOrder": 0 }
```

**Response:**
```json
{
  "code": 200,
  "msg": "操作成功",
  "data": { "id": 3, "name": "新分类", "userId": 1, "parentId": null, "sortOrder": 0 }
}
```

### POST /category/batchUpdate

批量更新分类（拖拽排序场景）。

**Request:**
```json
[
  { "id": 1, "name": "开发工具", "userId": 1, "parentId": null, "sortOrder": 99 }
]
```

**Response:**
```json
{ "code": 200, "msg": "操作成功", "data": null }
```

### POST /category/delete

删除分类及其所有子孙分类，级联删除关联书签。

**Request:**
```json
{ "id": 1 }
```

**Response:**
```json
{ "code": 200, "msg": "操作成功", "data": null }
```

---

## 书签

### GET /bookmark/list?userId={id}

**Response:**
```json
{
  "code": 200,
  "msg": "操作成功",
  "data": [
    { "id": 1, "title": "GitHub", "url": "https://github.com", "categoryId": 1, "userId": 1 }
  ]
}
```

### POST /bookmark/add

**Request:**
```json
{ "title": "GitHub", "url": "https://github.com", "categoryId": 1, "userId": 1 }
```

**Response:**
```json
{ "code": 200, "msg": "操作成功", "data": { "id": 1, ... } }
```

### POST /bookmark/delete

**Request:**
```json
{ "id": 1 }
```

### GET /bookmark/export?userId={id}&format={json|html}

文件下载。

| 参数 | 默认值 | 说明 |
|------|--------|------|
| format | json | `json` 返回 JSON 文件，`html` 返回浏览器兼容的书签 HTML |

### POST /bookmark/import

multipart/form-data。

| 字段 | 说明 |
|------|------|
| userId | 用户 ID |
| format | `json` 或 `html`（默认 json） |
| file | 上传文件 |

**Response:**
```json
{ "code": 200, "msg": "操作成功", "data": 5 }
```

导入行为：
- JSON 格式：先导入分类（重名复用已有 ID），再导入书签（同 URL 去重）
- HTML 格式：递归解析 `<DL><DT><H3>`（分类）和 `<DT><A>`（书签），同名分类复用
- 返回值为实际新增书签数
