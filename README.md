# 项目说明
该项目为[horo-ui](https://github.com/wlhyl/horo-ui.git)提供案例存放功能。并有一个自己的[web ui](https://github.com/wlhyl/horo-storage-ui.git)。

# 运行API服务

## 支持的features
* swagger: 启用swagger文档，访问地址：http://localhost:8081/swagger-ui/
* cors： 启用跨域支持

## 环境配置

### 日志配置
项目使用日志文件进行日志配置。
* 日志文件例子，文件名：log4rs.yaml，可修改为其它文件名。
```yaml
---
# 检查配置文件变动的时间间隔
refresh_rate: 30 seconds
# appender 负责将日志收集到控制台或文件, 可配置多个
appenders:
  stdout:
    kind: console
#  file:
#    kind: file
#    path: "log/log.log"
#    encoder:
#      # log 信息模式
#      pattern: "{d} - {m}{n}"
# 对全局 log 进行配置
root:
  level: info
  appenders:
    - stdout
#    - file # 启用此配置需要将上面的file节配置的注释取消，file与stdout平缓
```

## 准备mysql数据库
* 数据库主机：db_host
* 数据库名：db_name
* 数据库用户名：db_user
* 数据库密码：db_password

## 需要的环境变量
* LOG4RS_CONFIG：日志配置文件的路径
* DATABASE_URL：mysql连接字符串
* TOKEN_EXPIRE_SECONDS：JWT TOKEN过期秒数，如：86400秒（24小时）
* USERNAME：用户名
* PASSWORD：密码

## 迁移数据库
```bash
DATABASE_URL="mysql://db_user:db_password@db_host/db_name" \
  cargo run --bin migration
```


## 运行API Server
```bash
LOG4RS_CONFIG=/path/to/log4rs.yaml \
  DATABASE_URL="mysql://db_user:db_password@db_host/db_name" \
  TOKEN_EXPIRE_SECONDS=86400 \
  USERNAME="your_username" \
  PASSWORD="your_password" \
  cargo run --features swagger,cors \
  --bin storage_api -- -p 8081
```

## API文档
启动服务后，可通过以下地址访问Swagger文档：
```
http://localhost:8081/swagger-ui/
```

# Docker支持
## 构建镜像
```bash
docker build -t horo/storage-api .
```

## 运行容器
```bash
docker run -d -p 8080:8080 \
  -v /path/to/log4rs.yaml:/app/log4rs.yaml \
  -e LOG4RS_CONFIG=/app/log4rs.yaml \
  -e DATABASE_URL="mysql://db_user:db_password@db_host/db_name" \
  -e TOKEN_EXPIRE_SECONDS=86400 \
  -e USERNAME="your_username" \
  -e PASSWORD="your_password" \
  --name horo-storage-api \
  horo/storage-api
```

# 许可证
项目使用GPL-3.0 许可证 ([LICENSE](LICENSE))。