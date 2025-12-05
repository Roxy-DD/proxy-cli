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

- **Windows**：Windows 10/11 (x86_64)

## 代理类型说明

本工具设置的是**会话级代理**（Session-level Proxy），通过环境变量 `http_proxy` 和 `https_proxy` 实现：

- ✅ **仅对当前进程及其子进程有效**
- ✅ **不影响系统全局代理设置**
- ✅ **关闭终端后自动失效**
- ✅ **适合临时使用代理的场景**

如果需要系统级代理，请使用 Windows 系统设置或其他工具。

## 安装方法

### 方法一：双击安装（推荐）

1. 访问项目的 [GitHub Releases 页面](https://github.com/Roxy-DD/proxy-cli/releases) 下载最新版本的 `proxy-windows-x86_64.zip`。
2. 解压下载的文件。
3. **直接双击运行 `proxy.exe`**。
4. 程序会自动检测并提示安装到 PowerShell 配置文件，输入 `Y` 确认即可。
5. **重启** PowerShell 终端即可使用。

### 方法二：手动安装

1. 从 Release 页面下载并解压文件。
2. 将 `proxy.exe` 放置在任意位置。
3. 将以下函数添加到你的 PowerShell 配置文件 (`$PROFILE`) 中：

```powershell
function proxy {
    # 请修改为实际的 proxy.exe 路径
    $binaryPath = "C:\path\to\your\proxy.exe"
    $output = & $binaryPath
    $output | ForEach-Object {
        if ($_ -match "^#SET_PROXY:(.+)$") {
            $env:HTTP_PROXY = $matches[1]
            $env:HTTPS_PROXY = $matches[1]
        } elseif ($_ -eq "#CLEAR_PROXY") {
            $env:HTTP_PROXY = ""
            $env:HTTPS_PROXY = ""
        }
    }
}
```

### 方法三：从源码编译

1. 克隆仓库：`git clone https://github.com/Roxy-DD/proxy-cli.git`
2. 运行安装脚本：`.\install.ps1`
3. 重启终端。

## 使用说明

本工具设计为纯交互式工具，操作非常简单。

### 启动

在终端输入以下命令启动工具：

```powershell
proxy
```

### 交互界面

启动后，你将看到一个交互式菜单：

1.  **状态显示**：顶部会显示当前代理是否开启，以及当前的端口。
2.  **菜单选项**：
    *   **Enable/Disable Proxy**：回车键切换代理的开启/关闭状态。
    *   **Change Port**：选择此项后，输入新的端口号（如 7890）并回车。
    *   **Exit**：退出工具。

**提示**：
*   使用 `↑` `↓` 箭头键移动光标。
*   使用 `Enter` 键确认选择。
*   代理设置仅在**当前 PowerShell 会话**中有效。关闭窗口后自动失效。

## 配置文件

配置文件用于保存你的端口设置和上次的启用状态。

*   **位置**：%APPDATA%\proxy-cli\config.json
*   **格式**：JSON

通常你不需要手动修改此文件，工具会自动管理。

## 技术原理

本工具通过 **Wrapper Pattern（包装器模式）** 巧妙地解决了子进程无法修改父进程环境变量的限制。

### 核心问题
在操作系统中，子进程（Rust 程序）继承父进程（PowerShell）的环境变量，但子进程对环境变量的修改仅限于自身，无法反向影响父进程。因此，直接在 Rust 中设置 `HTTP_PROXY` 是无效的。

### 解决方案
我们采用 "Rust 指挥，PowerShell 执行" 的策略：

1. **Rust 端 (指挥官)**：
   - 运行交互式界面 (TUI)，让用户选择操作。
   - 程序退出前，将需要执行的操作以**特殊指令**的形式打印到标准输出 (`stdout`)。
   - 例如：`#SET_PROXY:http://127.0.0.1:7890` 或 `#CLEAR_PROXY`。

2. **PowerShell 端 (执行者)**：
   - 我们在 PowerShell 中定义了一个 `proxy` 函数（Wrapper）。
   - 该函数调用 Rust 二进制文件，并捕获其输出。
   - 解析输出内容，一旦发现特殊指令，立即在**当前 Shell 上下文**中执行环境变量的修改操作 (`$env:HTTP_PROXY = ...`)。

这种设计既保留了 Rust 编写高性能、跨平台 TUI 的优势，又完美实现了对 Shell 环境的控制。

## 许可证

本项目采用 GNU General Public License v3.0 许可证开源。详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 问题反馈

如有任何问题或建议，请通过 GitHub Issues 反馈。
