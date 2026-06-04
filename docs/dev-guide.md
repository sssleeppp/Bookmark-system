# 开发指南

## 环境要求

- [Nix](https://nixos.org/download.html)（包管理 + 隔离环境）
- Git

所有工具（cargo, rustc, rustfmt, clippy, bun, just）由 `flake.nix` 提供，无需手动安装。

```bash
# 进入开发环境
nix-shell
```

## 常用命令

```bash
just -l              # 列出所有命令
just build           # 编译后端 + 安装前端依赖
just start-backend   # 启动后端（localhost:8989）
just start-frontend  # 启动前端（localhost:5173）
just start-all       # 启动全栈
just stop            # 停止所有服务
just fmt             # cargo fmt + prettier + alejandra
just check           # clippy + prettier --check
just test-all        # 运行全部 API 测试
```

## 启动顺序

后端启动时自动初始化 SQLite 数据库（DROP + CREATE + 种子数据），无需外部数据库进程。

```
just start-all → 后端 :8989 + 前端 :5173
```

## 测试

所有测试在 `justfile` 中以 curl 脚本形式定义。

```bash
# 运行全部测试（需要先启动后端）
just test-all

# 单独运行
just test-login
just test-category-crud
just test-bookmark-import
just test-category-tree-delete
```

测试覆盖：登录/注册、分类 CRUD + 批量更新 + 递归删除、书签 CRUD、JSON/HTML 导出、JSON 导入。

## 代码风格

- **Rust**: `cargo fmt` (rustfmt 默认配置)
- **前端**: Prettier
- **Nix**: Alejandra

提交前运行 `just check` 确保通过。

## 数据库

SQLite 文件在 `.local/bookmark.db`。每次后端重启清空重建。如需持久化测试数据，注释 `db.rs` 中的 `DROP TABLE IF EXISTS` 语句。

## 架构要点

- Handlers 直连数据库（无 Service 层），通过 `std::sync::Mutex<Connection>` 共享
- 所有 unwrap 在原型阶段保持简洁，生产环境应改为 Result 传播
- 密码明文存储、无 JWT、CORS 全开放——仅开发用途
