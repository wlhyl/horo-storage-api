# Horo Storage API

## 项目简介

**Horo Storage API** 为 [horo-ui](https://github.com/wlhyl/horo-ui.git) 提供案例存放服务，并配有独立的 [Web UI](https://github.com/wlhyl/horo-storage-ui.git)。

## API 文档

完整接口文档请访问 Swagger UI 查看：
`http://localhost:8000/swagger-ui`

## 快速开始

### 依赖要求

- Rust 1.87+
- MySQL 5.7+/8.0+ （需设置如下参数）
  - CHARACTER SET：utf8mb4，指定数据库字符集为 utf8mb4，支持完整的 Unicode 字符（如 Emoji 等）。
  - COLLATE：utf8mb4_general_ci，指定排序规则为通用不区分大小写，适合大多数应用场景。
  - 创建数据库示例：
    ```sql
    CREATE DATABASE horo_storage DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
    ```
- Docker（可选）

### 本地运行

#### 开发环境（含 Swagger/CORS）

```bash
cp .env.example .env          # 编辑数据库和加密密钥配置
cp log4rs.yaml.example log4rs.yaml # 根据需要调整日志配置
# 2. 数据库迁移
cargo run --release --package migration

# 3. 启动开发服务
cargo run --features swagger,cors --bin storage_api -- -p 8081
```

#### 生产环境

```bash
# 1. 配置环境变量和日志（同上）
# 2. 数据库迁移（同上）

# 3. 启动服务（不启用 swagger/cors 特性）
cargo run --bin storage_api -- -p 8081
```

### Docker 镜像构建

```bash
docker build -t horo/storage-api .
```

## 配置说明

### 环境变量

- **DATABASE_URL**: MySQL 数据库连接 URL，示例值：mysql://user:password@localhost:3306/horo_storage
- **LOG4RS_CONFIG**: 日志配置文件路径，示例值：log4rs.yaml
- **TOKEN_EXPIRE_SECONDS**: JWT 令牌过期时间（秒），示例值：86400
- **USERNAME**: API 认证用户名，示例值：your_username
- **PASSWORD**: API 认证密码，示例值：your_password

### 可选 features

- `swagger`：API 文档
- `cors`：跨域支持

## 许可协议

本项目采用 GPL-3.0 协议开源，详见 [LICENSE](LICENSE)。
