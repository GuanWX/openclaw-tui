# AI Token Monitor

系统托盘应用，实时监控 AI API 账户的 Token 余额。

## 功能

- ✅ OpenAI API 余额查询（显示剩余额度、本月使用量）
- ✅ GitHub Copilot 使用情况查询
- ✅ 系统托盘常驻，点击查看详情
- ✅ 定时自动刷新
- ✅ 跨平台支持（macOS / Windows / Linux）

## 截图

```
┌─────────────────────────────┐
│   🤖 AI Token Monitor       │
├─────────────────────────────┤
│ 🟢 OpenAI              ●    │
│   Credits Remaining  $12.34 │
│   Used This Month    $5.67  │
├─────────────────────────────┤
│ 🟣 Copilot            ●     │
│   Tokens Used      125,000  │
│   Monthly Limit   1,000,000 │
├─────────────────────────────┤
│      [🔄 Refresh]           │
│        ⚙️ Settings          │
└─────────────────────────────┘
```

## 安装

### 前置要求

1. **Rust**: https://rustup.rs
2. **Node.js 18+**: https://nodejs.org
3. **系统依赖**:
   - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
   - **Windows**: Microsoft Visual Studio C++ Build Tools
   - **Linux**: `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`

### 构建步骤

```bash
# 克隆项目
cd ai-token-monitor

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建发布版本
npm run tauri build
```

## 配置

首次运行后，配置文件位于：

- **macOS**: `~/Library/Application Support/ai-token-monitor/config.json`
- **Windows**: `%APPDATA%\ai-token-monitor\config.json`
- **Linux**: `~/.config/ai-token-monitor/config.json`

手动编辑配置：

```json
{
  "openai_api_key": "sk-xxx",
  "github_token": "ghp_xxx",
  "refresh_interval": 300
}
```

### 获取 API Key

**OpenAI**:
1. 登录 https://platform.openai.com/api-keys
2. 创建 API Key（需要 Billing 权限）

**GitHub Copilot**:
1. 登录 https://github.com/settings/tokens
2. 创建 Personal Access Token，勾选 `copilot` scope
3. 注意：Copilot API 仅对 GitHub Enterprise 可用

## 开发

```
ai-token-monitor/
├── src/                    # React 前端
│   ├── App.tsx
│   ├── main.tsx
│   └── styles.css
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs         # Tauri 主程序
│   │   ├── config.rs       # 配置管理
│   │   └── api/
│   │       ├── openai.rs   # OpenAI API
│   │       └── copilot.rs  # Copilot API
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── README.md
```

## 技术栈

- **前端**: React + TypeScript + Vite
- **后端**: Tauri 2.0 + Rust
- **HTTP 客户端**: reqwest
- **系统托盘**: tauri tray-icon

## 已知限制

1. **OpenAI**: 需要 API Key 有 Billing 访问权限，免费账户无法查询余额
2. **Copilot**: GitHub 未公开个人版的使用量 API，Enterprise 可用 GraphQL API 查询
3. **跨平台**: Linux 需要安装 WebKit 依赖

## License

MIT