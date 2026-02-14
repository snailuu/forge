# Forge

Rust 编写的服务器初始化和部署工具。

## 功能

- 服务器初始化（nginx + deploy 用户）
- 自动配置 nginx
- SSH 密钥管理
- 交互式配置（用户名、项目目录）
- 支持自定义模板

## 系统要求

- Linux (Ubuntu/Debian)
- 需要 root 权限

## 快速开始

```bash
# 1. 下载并安装
wget https://github.com/snailuu/forge/releases/latest/download/forge-x86_64-unknown-linux-gnu.tar.gz
tar xzf forge-x86_64-unknown-linux-gnu.tar.gz
sudo mv forge /usr/local/bin/

# 2. 初始化服务器
sudo forge init

# 3. 按提示配置
# - 是否创建部署用户？
# - 是否创建项目目录？
# - 是否添加 SSH 公钥？

# 4. 完成！访问服务器 IP 查看默认页面
```

## 安装

### 方式 1：从 GitHub Release 下载（推荐）

```bash
# 下载最新版本（Linux x86_64）
wget https://github.com/snailuu/forge/releases/latest/download/forge-x86_64-unknown-linux-gnu.tar.gz

# 解压
tar xzf forge-x86_64-unknown-linux-gnu.tar.gz

# 安装
sudo mv forge /usr/local/bin/
sudo chmod +x /usr/local/bin/forge
```

**支持的平台**：
- `forge-x86_64-unknown-linux-gnu.tar.gz` - Linux x86_64
- `forge-aarch64-unknown-linux-gnu.tar.gz` - Linux ARM64
- `forge-x86_64-apple-darwin.tar.gz` - macOS Intel
- `forge-aarch64-apple-darwin.tar.gz` - macOS ARM (M1/M2)

### 方式 2：从源码编译

```bash
cargo build --release
sudo cp target/release/forge /usr/local/bin/
```

## 发布新版本

项目使用 GitHub Actions 自动构建和发布：

### 开发分支构建（dev）

```bash
# 推送到 dev 分支会自动构建并发布
git checkout dev
git push origin dev

# 下载最新 dev 构建
wget https://github.com/snailuu/forge/releases/download/dev-latest/forge-x86_64-unknown-linux-gnu.tar.gz
```

### 主分支构建（main）

```bash
# 推送到 main 分支会自动构建并发布
git checkout main
git push origin main

# 下载最新 main 构建
wget https://github.com/snailuu/forge/releases/download/main-latest/forge-x86_64-unknown-linux-gnu.tar.gz
```

### 正式版本发布（tag）

```bash
# 打标签触发正式发布
git tag v0.1.0
git push origin v0.1.0

# 产物：forge-x86_64-unknown-linux-gnu.tar.gz
# 发布：GitHub Releases（公开下载）
```

### 构建产物说明

| 触发方式     | Release Tag  | 下载位置                     | 用途       |
| ------------ | ------------ | ---------------------------- | ---------- |
| push to dev  | `dev-latest` | GitHub Releases (prerelease) | 开发测试   |
| push to main | `main-latest`| GitHub Releases (prerelease) | 预发布测试 |
| push tag     | tag 名称     | GitHub Releases              | 正式发布   |

所有构建都会生成 4 个平台的二进制文件：
- Linux x86_64
- Linux ARM64
- macOS Intel
- macOS ARM (M1/M2)

## 使用

### 命令参数

```bash
forge init [OPTIONS]

Options:
  --user <USER>          部署用户名（可选，不提供则交互式询问）
  --project <PROJECT>    项目名称（可选，不提供则交互式询问）
  --domain <DOMAIN>      站点域名或 IP 地址（默认：localhost）
  --ssh-key <SSH_KEY>    SSH 公钥（可选，不提供则交互式询问）
  -h, --help             显示帮助信息
```

### 初始化服务器

```bash
# 交互式初始化（推荐）
sudo forge init

# 完全自动化初始化
sudo forge init \
  --user deploy \
  --project myapp \
  --domain example.com \
  --ssh-key "ssh-ed25519 AAAA..."

# 仅创建用户和配置 SSH
sudo forge init --user deploy --ssh-key "ssh-ed25519 AAAA..."

# 创建项目目录并配置 nginx
sudo forge init --project myapp --domain example.com
```

初始化过程（未提供参数时）会询问：
1. **是否创建部署用户**（如果未提供 --user）
2. **是否创建项目目录**（如果未提供 --project）
3. **项目名称**（如果选择创建项目）
4. **是否添加 SSH 公钥**（如果未提供 --ssh-key）

**注意**：提供命令行参数时会跳过对应的交互式询问。

初始化会执行：
1. 检查系统平台（仅支持 Linux）
2. 安装 nginx
3. 创建部署用户（密码：`<username>123`）
4. 配置 SSH 密钥（如果提供）
5. 设置 /var/www 目录权限
6. 创建项目目录和默认页面（如果选择）
7. 配置 nginx 主配置文件
8. 启动 nginx 服务

### 自定义模板

通过环境变量 `FORGE_TEMPLATE_DIR` 指定自定义模板目录：

```bash
# 创建自定义模板目录
mkdir -p /etc/forge/templates

# 复制默认模板进行修改
cp templates/* /etc/forge/templates/

# 编辑模板
vim /etc/forge/templates/index.html.template

# 使用自定义模板运行
sudo FORGE_TEMPLATE_DIR=/etc/forge/templates forge init
```

**支持的模板文件**：
- `nginx.conf` - Nginx 主配置
- `app.conf.template` - 应用配置模板
- `index.html.template` - 默认首页模板（占位符：`{{PROJECT_NAME}}`）

**模板查找顺序**：
1. 环境变量 `FORGE_TEMPLATE_DIR` 指定的目录
2. 嵌入到二进制文件中的默认模板

## 示例

```bash
$ sudo forge init
==========================================
  Server Initialization
==========================================

Create deploy user? [y/N]: y
User name [deploy]: myapp
Create project directory in /var/www? [y/N]: y
Project name: myapp
Add SSH public key for passwordless login? [y/N]: n

📦 Updating package list...
📦 Installing nginx...
✓ Nginx installed
✓ Created user 'myapp'
⚠ No SSH key provided - you can add it later
✓ Web directory configured
✓ Project directory created: /var/www/myapp
✓ Nginx main config installed
✓ Nginx enabled and started

==========================================
  ✅ Initialization Complete!
==========================================

Deploy user: myapp
Web root: /var/www
Project directory: /var/www/myapp
```

## 开发

```bash
# 编译
cargo build

# 运行
cargo run -- init --ssh-key "..."
```

## 交叉编译（macOS -> Linux）

```bash
# 安装 cross
cargo install cross

# 编译 Linux 版本
cross build --release --target x86_64-unknown-linux-gnu
```

## License

MIT
