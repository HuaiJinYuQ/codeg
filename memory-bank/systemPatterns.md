# 系统架构与模式 — Codez

## 整体架构

### 双模式运行（三种二进制）

```
codeg（桌面）          codeg-server（服务器）       codeg-mcp（MCP伴生）
Tauri 窗口管理          Axum HTTP + WebSocket        stdio MCP 协议
feature: tauri-runtime  no-default-features          no-default-features
```

### 共享核心（同一套业务逻辑）
- `app_state.rs` → `AppState` 结构体，两种模式通过 `EventEmitter` 枚举区分事件发射
- `web/event_bridge.rs` → `EventEmitter::Tauri(AppHandle)` 或 `EventEmitter::WebOnly(Arc<WebEventBroadcaster>)`
- `web/router.rs` → Axum 路由，接受 `Arc<AppState>`
- `web/handlers/` → HTTP API 端点，全部使用 `Extension<Arc<AppState>>`

## 后端模块结构（`src-tauri/src/`）

```
app_state.rs          # 共享状态（db、连接管理器、终端管理器、事件广播器等）
models/               # 共享数据结构（agent、conversation、message、folder等）
parsers/              # 每个智能体一个解析器（16 个 + 自定义）
commands/             # 业务逻辑（_core 函数共用，#[tauri::command] 仅桌面）
web/                  # Axum HTTP API + WebSocket + 静态文件 + 认证中间件
acp/                  # Agent Client Protocol 连接管理
db/                   # SeaORM + SQLite（entities、migration、service）
chat_channel/         # 聊天频道管理
forge/                # Forge 功能
```

## AgentType 内置 Agent 列表（16 个）

```rust
pub enum AgentType {
    ClaudeCode,   // wire: "claude_code"
    Codex,        // wire: "codex"
    OpenCode,     // wire: "open_code"
    Gemini,       // wire: "gemini"
    OpenClaw,     // wire: "open_claw"
    Cline,        // wire: "cline"
    Hermes,       // wire: "hermes"
    CodeBuddy,    // wire: "code_buddy"
    KimiCode,     // wire: "kimi_code"
    Pi,           // wire: "pi"
    Grok,         // wire: "grok"
    Cursor,       // wire: "cursor"
    DeepSeek,     // wire: "deepseek"
    Qoder,        // wire: "qoder"
    Antigravity,  // wire: "antigravity"
    Sahaa,        // wire: "sahaa" —— 新增（Zoho 内置）
    Custom(&'static str),  // wire: "custom:<id>"
}
```

## 数据流

- 桌面模式：前端 `invoke()` → Tauri 命令 → 业务逻辑 → 返回数据
- 服务器模式：前端 `fetch()` → Axum HTTP API → 同一业务逻辑 → 返回 JSON
- 实时通信：后端事件 → EventEmitter（Tauri 事件 / WebSocket 广播）→ 前端

## 条件编译约定

- `#[cfg(feature = "tauri-runtime")]` — 仅桌面模式编译（Tauri 窗口、通知等）
- `#[cfg_attr(feature = "tauri-runtime", tauri::command)]` — 函数始终可用，仅在桌面模式标记为 Tauri 命令
- `_core` 后缀函数 — 接受普通引用参数（`&AppDatabase`、`&EventEmitter`），供 Web handlers 和 Tauri 命令共用

## 前端核心库（`src/`）

- **`lib/transport/`** — Transport 抽象层（自动检测 Tauri/Web 切换）
- **`lib/adapters/`** — AI 响应到组件渲染的适配器
- **`lib/types.ts`** — Rust 模型的 TypeScript 镜像
- **`lib/api.ts`** — 主 API 客户端
- **`lib/tauri.ts`** — Tauri API 封装
- **`components/agent-icon.tsx`** — 内置 agent 图标注册表（COLOR_ICONS + MONO_ICONS）

## Agent 图标注册表

```typescript
// 彩色 icon（SVG 或 img）
const COLOR_ICONS = {
  claude_code, codex, gemini, open_claw, kimi_code, pi, deepseek,
  sahaa,  // → SahaaColorIcon（<img src="/zoho-logo/sahaa-yali-logo.png">）
}

// 单色 icon（currentColor SVG）
const MONO_ICONS = {
  open_code, cline, hermes, code_buddy, grok, cursor, qoder, antigravity
}
```

## 重要约束

- **仅支持静态导出**：`next.config.ts` 设置 `output: "export"`，不支持动态路由
- **路径别名**：`@/*` 映射到 `./src/*`
- **服务器部署**：通过环境变量配置（`CODEG_PORT`、`CODEG_HOST`、`CODEG_TOKEN`、`CODEG_DATA_DIR`、`CODEG_STATIC_DIR`）
