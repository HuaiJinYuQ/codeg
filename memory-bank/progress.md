# 项目进度 — Codez

## 已完成功能

### 核心基础设施
- ✅ Tauri 2 桌面应用骨架
- ✅ Axum 服务器模式（`codeg-server`）
- ✅ MCP 伴生进程（`codeg-mcp`）
- ✅ SeaORM + SQLite 数据层
- ✅ WebSocket 实时通信
- ✅ Transport 抽象层（自动 Tauri/Web 切换）
- ✅ Docker 多阶段构建 + docker-compose

### 智能体集成（解析器）
- ✅ Claude Code
- ✅ Codex CLI
- ✅ Gemini CLI
- ✅ OpenCode
- ✅ OpenClaw
- ✅ Cline
- ✅ Cursor
- ✅ DeepSeek
- ✅ Grok
- ✅ Hermes
- ✅ Kimi Code
- ␅ Pi
- ✅ Qoder
- ✅ CodeBuddy
- ✅ Antigravity

### 前端功能
- ✅ 会话聚合工作台
- ✅ 多智能体委派（ACP）
- ✅ 异步任务看板
- ✅ Monaco 代码编辑器集成
- ✅ xterm.js 终端集成
- ✅ Git 操作界面（commit/merge/push/stash）
- ✅ 画布（Canvas）视图
- ✅ 设置界面
- ✅ 国际化（10 种语言）
- ✅ 主题切换（亮色/暗色）
- ✅ Forge 功能

### 品牌定制化（Zoho/Codez 定制，本 session 完成）
- ✅ App 名称改为 "codez"
- ✅ Tauri productName / identifier 更新
- ✅ Zoho logo 应用到全部图标资产
- ✅ 登录页、启动画面更新为 Zoho 风格
- ✅ SahaaWelcomeDialog（首次启动弹窗）

### Sahaa 内置 Agent（前端已完成，Rust 待编译）
- ✅ `src-tauri/src/models/agent.rs` — `AgentType::Sahaa` 完整添加
- ✅ `src-tauri/src/acp/registry.rs` — `builtin_acp_agents()` / `registry_id_for()` / `from_registry_id()` / `get_agent_meta()` 全部完善
- ✅ `src/components/agent-icon.tsx` — `SahaaColorIcon` + `COLOR_ICONS["sahaa"]`
- ✅ DB `agent_setting` 直接修补：wire `"sahaa"`, enabled=1, sort_order=0
- ✅ DB `custom_agent` 旧记录已清除
- ❌ Rust 编译验证（待有 Cargo 环境的机器执行）
- ❌ 新 `codeg-server` 二进制部署（待编译后替换）

## 当前版本
**0.30.5**（已发布），内部定制正在进行中

## 已知问题
- Rust 编译环境不可用（该机无法拉取 rustup stable toolchain）
- 旧 codeg-server 二进制（v0.30.5）不认识 wire name `"sahaa"`，导致 Sahaa 在当前 UI 上无法显示
- next.config.ts 有临时 `ignoreBuildErrors: true`，生产前需恢复

## 技术债务
- `db/service/custom_agent_service.rs` 中的 `seed_sahaa_agent()` 已过时，内置 agent 不需播种
- `db/service/agent_setting_service.rs` 中的 `remove_stale_sahaa()` / `pin_sahaa_as_default()` 可以在下一版本删除
- `db/mod.rs` 中对应调用同步删除

## 决策演进
- 选择 Tauri 2 而非 Electron，获得更小的包体积和更好的原生性能
- 选择 SeaORM + SQLite 而非文件系统存储，便于索引和查询
- 静态导出（无 SSR）以同时支持 Tauri 内嵌和独立服务器两种模式
- Sahaa 从xe custom_agent 升级为内置（builtin）agent，与 Claude Code、Qoder 等完全等价
