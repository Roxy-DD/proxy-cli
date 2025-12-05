# proxy-cli

一个简单的命令行代理管理工具，用于快速切换 Shell 会话级的 HTTP/HTTPS 代理。

## 功能特点

- ✨ **交互式菜单**：提供直观的图形化界面，支持键盘导航
- 🚀 **快速切换**：一键启用/禁用代理设置
- 📋 **端口管理**：轻松设置和管理代理端口
- 📊 **状态显示**：实时查看当前代理状态
- 💾 **配置持久化**：保存代理设置以便后续使用
- 🔒 **会话级代理**：使用环境变量设置代理，不影响系统设置

## 支持平台

- **Windows**: Windows 10/11 (x86_64)
- **Linux**: x86_64 (GNU)
- **macOS**: Intel (x86_64) 和 Apple Silicon (aarch64)

## 代理类型说明

本工具设置的是**会话级代理**（Session-level Proxy），通过环境变量 `http_proxy` 和 `https_proxy` 实现：

- ✅ **仅对当前进程及其子进程有效**
- ✅ **不影响系统全局代理设置**
- ✅ **关闭终端后自动失效**
- ✅ **适合临时使用代理的场景**

如果需要系统级代理，请使用 Windows 系统设置或其他工具。

## 安装方法

### 方法一：从 GitHub Releases 下载

1. 访问项目的 [GitHub Releases 页面](https://github.com/Roxy-DD/proxy-cli/releases)

2. 根据您的平台下载对应的文件：
   - **Windows**: `proxy-windows-x86_64.zip`
   - **Linux**: `proxy-linux-x86_64.tar.gz`
   - **macOS (Intel)**: `proxy-macos-x86_64.tar.gz`
   - **macOS (Apple Silicon)**: `proxy-macos-aarch64.tar.gz`

3. 解压文件，将可执行文件保存到任意目录

4. 可选：将该目录添加到系统 PATH 环境变量，即可在任意位置使用 `proxy` 命令

### 方法二：从源码编译

1. 确保已安装 Rust 环境（[安装指南](https://www.rust-lang.org/tools/install)）

2. 克隆仓库并进入目录：
   ```powershell
   git clone https://github.com/Roxy-DD/proxy-cli.git
   cd proxy-cli
   ```

3. 编译程序：
   ```powershell
   cargo build --release
   ```

4. 编译完成后，可执行文件位于：
   - **Windows**: `target/release/proxy.exe`
   - **Linux/macOS**: `target/release/proxy`

### 添加到 PATH（可选）

#### Windows (PowerShell)

```powershell
# 临时添加到当前会话（重启 PowerShell 后失效）
$env:PATH += ";路径\到\proxy.exe所在目录"

# 永久添加到用户环境变量
[Environment]::SetEnvironmentVariable("PATH", "$env:PATH;路径\到\proxy.exe所在目录", [EnvironmentVariableTarget]::User)
```

#### Linux/macOS

```bash
# 添加到 ~/.bashrc 或 ~/.zshrc
export PATH="$PATH:/path/to/proxy"

# 或者创建符号链接
sudo ln -s /path/to/proxy /usr/local/bin/proxy
```

## 使用说明

### 交互式模式

直接运行程序即可启动交互式菜单：

```powershell
proxy
```

使用上下箭头导航菜单，回车键选择功能，Q/Esc 键退出。

**注意**：代理设置仅在当前 PowerShell/终端会话中有效。关闭终端后，代理设置会自动失效。

### 命令行模式

#### 启用代理

```powershell
proxy enable
```

#### 禁用代理

```powershell
proxy disable
```

#### 设置代理端口

```powershell
proxy set-port 8080
```

#### 查看当前状态

```powershell
proxy status
```

## 配置文件

配置文件用于保存端口设置，位于：

- **Windows**: `%APPDATA%\proxy-cli\config.json`
- **Linux/macOS**: `~/.config/proxy-cli/config.json`

配置文件仅保存端口设置，代理的启用/禁用状态不会持久化（因为使用的是会话级环境变量）。

## 许可证

本项目采用 GNU General Public License v3.0 许可证开源。详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 问题反馈

如有任何问题或建议，请通过 GitHub Issues 反馈。
