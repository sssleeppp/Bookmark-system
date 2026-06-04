# 项目结构

```
Bookmark-system/
├── backend/               # Rust 后端
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs        # 入口：路由注册、CORS、启动服务器
│       ├── db.rs          # SQLite 建库、建表、种子数据
│       ├── result.rs      # ApiResult<T> 统一返回体 {code, msg, data}
│       ├── models.rs      # User / Category / Bookmark 结构体
│       └── handlers/
│           ├── mod.rs
│           ├── user.rs        # /user/login, /user/register
│           ├── category.rs    # /category/* CRUD + 树形递归删除
│           └── bookmark.rs    # /bookmark/* CRUD + JSON/HTML 导入导出
├── frontend/              # Vue 3 前端
│   ├── src/
│   │   ├── main.js        # Vue 应用入口
│   │   ├── App.vue
│   │   ├── router/        # Vue Router（登录守卫）
│   │   ├── utils/         # Axios 实例
│   │   ├── components/    # 登录页组件
│   │   └── views/         # HomeView, LoginView
│   ├── vite.config.js
│   └── package.json
├── docs/                  # 文档
│   ├── api.md
│   ├── dev-guide.md
│   └── structure.md
├── flake.nix              # Nix 开发环境定义
├── justfile               # 任务运行器
├── AGENTS.md              # AI 编码助手说明书
└── README.md
```

## 核心模块说明

### backend/src/handlers/bookmark.rs

最大模块（~500 行）。包含书签 CRUD 以及 JSON/HTML 两种格式的导入导出逻辑：
- **JSON 导出**：将 categories + bookmarks 序列化为 `{"categories":[...], "bookmarks":[...]}`
- **HTML 导出**：生成 Netscape Bookmark HTML 格式（浏览器兼容）
- **JSON 导入**：二遍扫描解析 categories（解决父子顺序问题），去重插入 bookmarks
- **HTML 导入**：用 `scraper` crate 解析 `<DL><DT><H3><A>` 结构，递归建分类树

### backend/src/handlers/category.rs

分类 CRUD + 递归删除。删除父分类时递归收集所有子孙分类 ID，先删关联书签，再删分类。

### backend/src/db.rs

每次服务启动时执行 `DROP TABLE IF EXISTS` → `CREATE TABLE` → 插入种子用户 `admin/123456`。数据库总是从干净状态开始。
