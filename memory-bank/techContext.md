# 技术上下文 — Codez

## 技术栈

| 层 | 技术 |
|-----|------|
| 桌面运行时 | Tauri 2（Rust 后端 + webview 前端）|
| 服务器运行时 | 独立 Rust 二进制（Axum HTTP + WebSocket）|
| 前端 | Next.js 16（静态导出模式）+ React 19 + TypeScript (strict) |
| 样式 | Tailwind CSS v4 + shadcn/ui |
| 国际化 | next-intl（10 种语言）|
| 数据库 | SeaORM + SQLite |
| 包管理器 | pnpm |
| Rust 版本 | 2021 edition |

## 开发命令

### 前端
```bash
pnpm eslint .                  # lint
pnpm test                      # vitest 全跑（CI 用同一条命令）
pnpm build                     # 静态导出构建
```

### 后端 Rust（在 `src-tauri/` 目录下）
```bash
# 服务器模式
cargo build --no-default-features --bin codeg-server --release
cargo check --no-default-features --bin codeg-server
cargo clippy --no-default-features --bin codeg-server --lib -- -D warnings

# 桌面模式（默认 feature）
cargo check
cargo test --features test-utils
cargo clippy --all-targets --features test-utils -- -D warnings
```

### 服务器启动
```bash
# 使用已安装的旧二进制 + 新前端
CODEG_STATIC_DIR=/Users/zhangkai/project/project_item/codeg/out \
  /Applications/codeg.app/Contents/MacOS/codeg-server

# DB 路径
~/Library/Application\ Support/app.codeg/codeg.db
```

## 环境限制

- 此机无 Rust 编译工具链（rustup 安装失败，网络受限）
- brew 安装 rust 无效
- 必须在有 Rust 工具链的机器上执行编译

## 依赖

| 名称 | 用途 |
|------|------|
| `@tauri-apps/api` | Tauri 2 前端 API |
| `next-intl` | 国际化 |
| `shadcn/ui` + `radix-ui` | UI 组件库 |
| `tailwindcss v4` | 样式 |
| `monaco-editor` | 代码编辑器 |
| `xterm.js` | 终端模拟 |
| `thiserror` | Rust 错误类型定义 |
| `axum` | Rust HTTP 框架 |
| `sea-orm` | Rust ORM |
| `sacp` / `sacp-tokio` | ACP 协议实现 |

## 关键路径

```
/Users/zhangkai/project/project_item/codeg/     # 项目根目录
  src/                                           # Next.js 前端
    app/workspace/layout.tsx                     # SahaaWelcomeDialog 挂载点
    components/agent-icon.tsx                    # agent 图标注册表
    components/layout/sahaa-welcome-dialog.tsx   # Sahaa 欢迎弹窗
    i18n/messages/                               # 10 种语言 i18n 文件
  src-tauri/
    src/
      models/agent.rs                            # AgentType 定义（含 Sahaa）
      acp/registry.rs                            # ACP 元数据和功能
      db/service/custom_agent_service.rs         # 有过时的 seed_sahaa_agent（待删）
      db/service/agent_setting_service.rs        # Sahaa 迁移函数（待删）
      db/mod.rs                                  # init_database 入口
    tauri.conf.json                              # productName=codez, id=app.codez
    icons/                                       # 已更新为 Zoho logo
  public/
    icon.svg                                     # Zoho logo
    zoho-logo/
      zoho-logo-web.svg                          # Zoho 官方 logo
      sahaa-yali-logo.png                        # Sahaa agent 图标
  memory-bank/                                   # 记忆库
  out/                                           # Next.js 静态导出结果（供服务器使用）
```

## 代码风格

- Prettier：无分号、尾逗号（es5）、2 空格缩进、80 字符宽度
- ESLint：next/core-web-vitals + typescript + prettier
- TypeScript：strict 模式，`noUnusedLocals` + `noUnusedParameters`
- Rust：2021 edition，使用 `thiserror` 定义错误类型
- 注释使用中文
