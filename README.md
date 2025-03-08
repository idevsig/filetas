# Filetas

基于 Rust + Axum 的高性能文件加速下载服务 (High-performance file acceleration download service)

## 功能特性

- **高性能**: 基于 Rust 和 Axum 框架，提供极致性能
- **GitHub 加速**: 自动识别并加速 GitHub 文件下载
- **智能重定向**: 自动处理 HTTP 重定向和 URL 转换
- **CORS 支持**: 完整的跨域资源共享支持
- **现代界面**: 蓝色渐变设计的现代化 Web 界面
- **详细日志**: 结构化日志记录，支持多级别日志输出
- **灵活配置**: 支持命令行参数和环境变量配置
- **容器化**: 提供 Docker 镜像，支持容器化部署

## 快速开始

### 先决条件

```bash
# Ubuntu/Debian
sudo apt update -y
sudo apt install -y pkg-config libssl-dev

# CentOS/RHEL
sudo yum install -y pkgconfig openssl-devel

# macOS
brew install pkg-config openssl
```

### 安装运行

1. **下载源码**

```bash
git clone https://github.com/idev-sig/filetas.git
cd filetas
```

2. **开发环境运行**

```bash
cargo run
```

3. **生产环境构建**

```bash
cargo build --release
./target/release/filetas
```

## 使用方法

### 命令行参数

```bash
filetas [OPTIONS]

选项:
  -H, --host <HOST>              服务器监听地址 [默认: 0.0.0.0] [环境变量: HOST]
  -p, --port <PORT>              服务器端口 [默认: 8000] [环境变量: PORT]
  -t, --title <TITLE>            页面标题 [默认: 文件加速下载] [环境变量: TITLE]
      --template-dir <DIR>       模板目录路径 [默认: templates] [环境变量: TEMPLATE_DIR]
      --user-agent <USER_AGENT>  请求用户代理 [环境变量: USER_AGENT]
  -v, --verbose                  启用详细日志 (等同于 RUST_LOG=debug)
  -q, --quiet                    启用安静模式 (等同于 RUST_LOG=warn)
  -h, --help                     显示帮助信息
  -V, --version                  显示版本信息
```

### 使用示例

```bash
# 基本使用
filetas

# 自定义端口和主机
filetas --host 127.0.0.1 --port 3000

# 自定义页面标题
filetas --title "我的文件服务器"

# 启用详细日志
filetas --verbose

# 使用环境变量
HOST=0.0.0.0 PORT=8080 TITLE="File Server" filetas

# 组合使用
RUST_LOG=debug filetas --port 8080 --title "开发服务器"
```

### 环境变量

| 变量名         | 描述           | 默认值                         |
| -------------- | -------------- | ------------------------------ |
| `HOST`         | 服务器监听地址 | `0.0.0.0`                      |
| `PORT`         | 服务器端口     | `8000`                         |
| `TITLE`        | 页面标题       | `文件加速下载`                 |
| `TEMPLATE_DIR` | 模板目录路径   | `templates`                    |
| `USER_AGENT`   | 请求用户代理   | `Mozilla/5.0 ...`              |
| `RUST_LOG`     | 日志级别       | `filetas=info,tower_http=info` |

### 日志配置

```bash
# 默认日志级别 (INFO)
filetas

# 详细调试日志
filetas --verbose
# 或
RUST_LOG=debug filetas

# 只显示警告和错误
filetas --quiet
# 或
RUST_LOG=warn filetas

# 自定义日志级别
RUST_LOG=filetas=trace,tower_http=debug filetas
```

## 支持的 URL 格式

### GitHub 文件加速

- **Releases**: `https://github.com/user/repo/releases/download/v1.0.0/file.zip`
- **Archive**: `https://github.com/user/repo/archive/refs/heads/main.zip`
- **Raw 文件**: `https://github.com/user/repo/raw/main/file.txt`
- **Blob 文件**: `https://github.com/user/repo/blob/main/file.txt` (自动转换为 raw)
- **Gist**: `https://gist.github.com/user/gist-id/raw/file.txt`
- **Tags**: `https://github.com/user/repo/tags`

### 通用文件下载

- 任何 HTTP/HTTPS 文件 URL
- 自动处理重定向
- 支持大文件流式传输

## Web 界面使用

1. 访问 `http://localhost:8000`
2. 在输入框中粘贴文件 URL
3. 点击下载按钮或按回车键
4. 文件将通过加速服务下载

## Docker 部署

### 使用预构建镜像

```bash
# Docker Hub
docker run -p 8000:8000 -d idevsig/filetas:latest

# GitHub Container Registry
docker run -p 8000:8000 -d ghcr.io/idev-sig/filetas:latest

# 自定义配置
docker run -p 3000:3000 -e PORT=3000 -e TITLE="My File Server" -d idevsig/filetas:latest
```

### Docker Compose

```yaml
services:
  filetas:
    image: idevsig/filetas:latest
    container_name: filetas
    restart: unless-stopped
    ports:
      - "8000:8000"
    environment:
      - HOST=0.0.0.0
      - PORT=8000
      - TITLE=文件加速下载
      - RUST_LOG=filetas=info
    volumes:
      - ./templates:/app/templates # 可选：自定义模板
```

### 构建自定义镜像

```bash
# 克隆仓库
git clone https://github.com/idev-sig/filetas.git
cd filetas

# 构建镜像
docker build -t my-filetas .

# 运行
docker run -p 8000:8000 -d my-filetas
```

## API 使用

### 直接下载

```bash
# 通过服务下载文件
curl -L "http://localhost:8000/https://example.com/file.zip" -o file.zip

# GitHub 文件加速
curl -L "http://localhost:8000/https://github.com/user/repo/releases/download/v1.0.0/file.zip" -o file.zip
```

### CORS 支持

服务支持跨域请求，可以在前端 JavaScript 中直接使用：

```javascript
// 获取文件
fetch("http://localhost:8000/https://example.com/file.json")
  .then((response) => response.json())
  .then((data) => console.log(data));

// 下载文件
const downloadUrl =
  "http://localhost:8000/" + encodeURIComponent("https://example.com/file.zip");
window.open(downloadUrl);
```

## 开发

### 项目结构

```
filetas/
├── src/
│   └── main.rs          # 主程序
├── templates/
│   └── index.html       # Web 界面模板
├── Cargo.toml           # 项目配置
├── Dockerfile           # Docker 构建文件
└── README.md            # 项目文档
```

### 本地开发

```bash
# 克隆项目
git clone https://github.com/idev-sig/filetas.git
cd filetas

# 安装依赖并运行
cargo run

# 开启详细日志的开发模式
RUST_LOG=debug cargo run -- --verbose

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 性能优化

- 使用 Rust 的零成本抽象和内存安全特性
- 基于 Tokio 异步运行时，支持高并发
- 流式传输大文件，减少内存占用
- 智能重定向处理，减少不必要的请求
- 结构化日志记录，便于性能分析

## 故障排除

### 常见问题

1. **端口被占用**

   ```bash
   # 检查端口占用
   lsof -i :8000
   # 使用其他端口
   filetas --port 8080
   ```

2. **模板文件未找到**

   ```bash
   # 确保模板目录存在
   ls -la templates/
   # 或指定自定义模板目录
   filetas --template-dir /path/to/templates
   ```

3. **SSL/TLS 错误**
   ```bash
   # 确保安装了 OpenSSL 开发包
   sudo apt install libssl-dev pkg-config
   ```

### 日志分析

```bash
# 启用详细日志查看请求详情
RUST_LOG=debug filetas

# 只查看错误日志
RUST_LOG=error filetas

# 查看特定模块日志
RUST_LOG=filetas::proxy=debug filetas
```

## 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

本项目采用 Apache-2.0 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 仓库镜像

- [MyCode](https://git.jetsung.com/idev/filetas) (主仓库)
- [GitHub](https://github.com/idev-sig/filetas)
- [Framagit](https://framagit.org/idev/filetas)
- [GitCode](https://gitcode.com/idev/filetas)
