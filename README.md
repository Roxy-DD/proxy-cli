# Ultimate CLI Proxy Manager

一个功能强大的命令行代理管理工具，用于快速切换本地 HTTP/HTTPS 代理。

## 功能特点

- ✨ **交互式菜单**：提供直观的图形化界面，支持键盘导航
- 🚀 **快速切换**：一键启用/禁用代理设置
- 📋 **端口管理**：轻松设置和管理代理端口
- 📊 **状态显示**：实时查看当前代理状态
- 💾 **配置持久化**：保存代理设置以便后续使用

## 支持平台

- Windows 10/11
- PowerShell 5.0+
- CMD

## 安装方法

### 方法一：从 GitHub Releases 下载

1. 访问项目的 [GitHub Releases 页面](https://github.com/Roxy-DD/proxy-cli/releases)

2. 下载最新版本的 `proxy.exe` 文件

3. 将下载的文件保存到任意目录

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

4. 编译完成后，可执行文件位于 `target/release/proxy.exe`

### 添加到 PATH（可选）

将 `proxy.exe` 所在目录添加到系统 PATH 环境变量，即可在任意位置使用 `proxy` 命令：

```powershell
# 临时添加到当前会话（重启 PowerShell 后失效）
$env:PATH += ";路径\到\proxy.exe所在目录"

# 永久添加到系统环境变量
[Environment]::SetEnvironmentVariable("PATH", "$env:PATH;路径\到\proxy.exe所在目录", [EnvironmentVariableTarget]::User)
```

## 使用说明

### 交互式模式

直接运行程序即可启动交互式菜单：

```powershell
proxy
```

使用上下箭头导航菜单，回车键选择功能，Q/Esc 键退出。

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

配置文件位于：

```
%APPDATA%\proxy-cli\config.json
```

## 许可证

本项目采用 GNU General Public License v3.0 许可证开源。详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 问题反馈

如有任何问题或建议，请通过 GitHub Issues 反馈。
