# 智能书签管理系统 (Smart Bookmark System)

## 📌 项目简介
本项目是一个前后端彻底分离的**智能书签管理系统**，项目旨在解决用户在多设备、多平台间难以统一管理和检索书签的痛点。我们不仅提供基础的书签增删改查功能，还将引入**AI 大模型技术**（如自动摘要、智能分类）以及**云原生分布式架构**（缓存、微服务、消息队列等），打造一个高可用、智能化的现代化 Web 应用。

## 🛠️ 技术栈 (Tech Stack)
- **前端 (Frontend)**: Vue 3, Vite, Element Plus, Axios
- **后端 (Backend)**: Java 17, Spring Boot, MyBatis Plus, Maven, JWT, Spring Security
- **数据库 (Database)**: MariaDB (MySQL 兼容)
- **进阶中间件 (Middleware)**: Redis, RabbitMQ (规划中)
- **开发与部署**: Nix 环境隔离, Just (任务运行器), Docker (规划中)

## 🚀 快速启动 (Quick Start - Linux / macOS)

本项目使用 `Nix` 和 `just` 简化了本地开发环境的配置和任务运行。

```bash
# 1. 进入开发环境 (需要安装 Nix，它会自动提供 Java 和 MariaDB 环境)
nix develop

# 2. 启动所有服务 (数据库、后端 Spring Boot、前端 Vite)
just start-all

# 3. 停止所有服务
just stop

# 其他常用命令：
just build         # 编译后端并安装前端 bun 依赖
just start-db      # 仅启动本地数据库 (数据文件储存在当前目录的 .local/mysql 中)
just start-backend # 仅启动后端
just start-frontend# 仅启动前端
just fmt           # 全栈统一格式化代码 (包含后端 Java、前端 Vue/TS 和 Nix 脚本)

#对数据库进行了修改，在新增导入导出功能之前跑起过项目的需要执行一下命令
mariadb --socket .local/mysql/mysql.sock -u root bookmark_system -e "
     ALTER TABLE bookmark MODIFY COLUMN title VARCHAR(500) NOT NULL;
     ALTER TABLE bookmark MODIFY COLUMN url VARCHAR(2000) NOT NULL;
     ALTER TABLE category MODIFY COLUMN name VARCHAR(200) NOT NULL;
     "
```

---

## 🪟 Windows 开发环境搭建指南

由于本项目重度使用了 Nix 隔离本地数据库和 Java 环境，对于 Windows 开发者，我们提供以下两种环境配置方式：

### 方案一：使用 WSL2 (强烈推荐 ⭐)
在 Windows 下获得与 Linux 完美一致的零配置开发体验。
1. 打开 PowerShell 以管理员身份运行：`wsl --install` 安装 Ubuntu。
2. 重启后进入 WSL Ubuntu 终端，执行官方安装脚本安装 Nix：
   `sh <(curl -L https://nixos.org/nix/install) --daemon`
3. 在 WSL 终端中进入项目目录，直接按照上方的 **【快速启动】** 步骤运行 `nix develop` 和 `just start-all` 即可。

### 方案二：Windows 原生环境 (需手动安装依赖)
如果你不想使用 WSL，需要手动在 Windows 系统中准备以下底层环境：
1. **安装 JDK 17**：并配置好 `JAVA_HOME` 环境变量。
2. **安装 Bun**：在 PowerShell 运行 `powershell -c "irm bun.sh/install.ps1 | iex"` 安装前端包管理器。
3. **安装 Just**：建议使用 Scoop (`scoop install just`) 安装任务运行器。
4. **手动配置数据库**：
   - 原生 Windows 下 `just start-db` 将无法运行。
   - 请自行下载安装 [MariaDB Windows 版](https://mariadb.org/download/) 或 MySQL，默认监听 3306 端口。
   - 确保本地数据库的账号为 `root`，密码为 `root`（或者前往 `backend/src/main/resources/application.yml` 中修改为你自己的密码）。
5. **分步启动项目**：
   - 打开一个终端运行：`just start-backend`
   - 打开另一个终端运行：`just start-frontend`

---


