# filetas

基于 Rust + axum 的文件传输加速服务 (File transfer acceleration service)

1. 下载源码

```bash
git clone https://github.com/idevsig/filetas.git
```

2. 运行

**开发环境**

```bash
cargo run
```

```
Listening on 0.0.0.0:8000
```

**生产环境**

```bash
cargo build --release

./target/release/filetas

```

```
Listening on 0.0.0.0:8000
```

## Docker

使用本项目提供的镜像

- ghcr.io: https://github.com/idevsig/filetas/packages
- Docker Hub: https://hub.docker.com/r/idevsig/filetas

```bash
# docker.io
docker run -p 8000:8000 -d idevsig/filetas:python

# ghcr.io
docker run -p 8000:8000 -d ghcr.io/idevsig/filetas:python
```

### 构建

```bash
docker build --build-arg -t filetas .
```

### 使用

```bash
docker run -p 8000:8000 -d filetas
```

**docker-compose.yml**  
参考：https://git.jetsung.com/jetsung/docker-compose/-/tree/main/filetas

```yml
services:
  filetas:
    image: idevsig/filetas:latest
    container_name: filetas
    restart: unless-stopped
    ports:
      - 8000:8000
    command: ["filetas"]
```

---
## 仓库镜像
- https://git.jetsung.com/idev/filetas
- https://framagit.org/idev/filetas
- https://gitcode.com/idev/filetas
- https://github.com/idevsig/filetas