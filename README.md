# filetas

基于 Python + Flask 的文件传输加速服务 (File transfer acceleration service)

- https://git.jetsung.com/idev/filetas
- https://framagit.org/idev/filetas
- https://github.com/idev-sig/filetas

## Python

1. 下载源码

```bash
git clone https://github.com/idev-sig/filetas.git
```

2. 安装依赖

```bash
pip install -r requirements.txt
```

3. 运行

**开发环境**

```bash
python app.py
```

```
 * Serving Flask app 'app'
 * Debug mode: off
WARNING: This is a development server. Do not use it in a production deployment. Use a production WSGI server instead.
 * Running on all addresses (0.0.0.0)
 * Running on http://127.0.0.1:5000
```

**生产环境**

```bash
gunicorn app:app -D
```

```
[2023-03-26 22:00:43 +0800] [68613] [INFO] Starting gunicorn 20.1.0
[2023-03-26 22:00:43 +0800] [68613] [INFO] Listening at: http://127.0.0.1:8000 (68613)
[2023-03-26 22:00:43 +0800] [68613] [INFO] Using worker: sync
[2023-03-26 22:00:43 +0800] [68614] [INFO] Booting worker with pid: 68614
```

## Docker

使用本项目提供的镜像

- ghcr.io: https://github.com/devdoz/filetas/packages
- Docker Hub: https://hub.docker.com/r/jetsung/filetas

```bash
# docker.io
docker run -p 8000:8000 -d jetsung/filetas:latest

# ghcr.io
docker run -p 8000:8000 -d ghcr.io/devdoz/filetas:latest
```

### 构建

```bash
# 使用镜像源
docker build --build-arg PIP_MIRROR_URL=https://pypi.tuna.tsinghua.edu.cn/simple -t filetas .

# 不使用镜像
docker build --build-arg -t filetas .
```

### 使用

```bash
docker run -p 8000:8000 -d filetas
```

或者参考 `gunicorn`

```bash
docker run -p 8000:8000 -d filetas gunicorn -w 4 -b 0.0.0.0:8000 app:app
```

**docker-compose.yml**  
参考：https://jihulab.com/jetsung/docker-compose/-/tree/main/filetas

```yml
version: "3"
services:
  filetas:
    image: jetsung/filetas:latest
    container_name: filetas
    restart: unless-stopped
    environment:
      - TITLE=文件加速下载 # 网站标题
      - USERNAME= # BasicAuth 用户名
      - PASSWORD= # BasicAuth 密码
    ports:
      - 8000:8000
    command: ["gunicorn", "-w", "4", "-b", "0.0.0.0:8000", "app:app"]
```

    environment:
      - TITLE=文件传输加速服务 # 网站标题
      - USERNAME= # BasicAuth 用户名
      - PASSWORD= # BasicAuth 密码
