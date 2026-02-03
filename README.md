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
wget https://github.com/你的用户名/forge/releases/latest/download/forge-x86_64-unknown-linux-gnu.tar.gz

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

```bash
# 1. 提交代码
git add .
git commit -m "feat: 新功能"
git push

# 2. 打标签触发发布
git tag v0.1.0
git push origin v0.1.0

# 3. GitHub Actions 会自动：
#    - 编译 4 个平台的二进制文件
#    - 打包成 tar.gz
#    - 创建 GitHub Release
#    - 上传所有平台的安装包
```

发布完成后，用户可以从 `https://github.com/你的用户名/forge/releases` 下载对应平台的安装包。

## 使用

### 初始化服务器

```bash
# 交互式初始化
sudo forge init

# 带 SSH 密钥初始化
sudo forge init --ssh-key "ssh-ed25519 AAAA..."
```

初始化过程会询问：
1. **是否使用默认用户名 deploy**
2. **是否创建项目目录**（默认：否）
3. **项目名称**（如果选择创建）
4. **是否添加 SSH 公钥**（如果命令行未提供）

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

Deploy user name [deploy]: myapp
Create project directory in /var/www? [y/N]: y
Project name: myapp

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

# 测试
cargo test
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
