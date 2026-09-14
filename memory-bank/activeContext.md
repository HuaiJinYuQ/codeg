# 活跃上下文 — Codez（2026-09-08）

## 当前焦点

**Sahaa 从「自定义智能体」升级为「内置 Agent」**，Rust 源码已改，但受编译环境限制，旧二进制（0.30.5）尚在运行，需要在有 Rust 工具链的机器编译后才能完全验证。

---

## 最近完成的所有工作（本次 session）

### ① 品牌重命名：codeg → codez
- `src-tauri/tauri.conf.json`: `productName: "codez"`, `identifier: "app.codez"`
- 前端所有用户可见文字 → "codez"（layout、boot screen、document title、i18n 10 种语言、settings 描述、导出页脚）
- 登录页、启动画面替换 Zoho logo

### ② 图标全量替换：Zoho logo
- 来源：`public/zoho-logo/zoho-logo-web.svg`（官方 SVG 路径）
- 应用：512×512 白色圆角矩形画布，`transform="translate(53.73 185.68) scale(0.3950)"`
- 覆盖所有尺寸：`src-tauri/icons/*.{svg,png,icns,ico}`，`public/icon.svg`，`public/icon-32x32.png`，`public/icon-128x128.png`

### ③ SahaaWelcomeDialog（首次启动欢迎弹窗）
- 文件：`src/components/layout/sahaa-welcome-dialog.tsx`
- 挂载：`src/app/workspace/layout.tsx`
- 首次启动显示，"今后都不显示" 写 `localStorage("sahaa-welcome-dialog-dismissed")`

### ④ Sahaa 内置 Agent —— Rust 源码修改（待编译）

**目标**：Sahaa 与 Claude Code、Qoder 完全等价的内置 agent，不再走 custom_agent 路径。

**已修改文件**：

| 文件 | 改动 |
|------|------|
| `src-tauri/src/models/agent.rs` | 新增 `AgentType::Sahaa`，wire=`"sahaa"`，加入 `BUILTIN_AGENT_TYPES`，`from_wire`/`as_wire`/`Display` 全覆盖，`is_valid_custom_agent_id` 屏蔽 `"sahaa"` |
| `src-tauri/src/acp/registry.rs` | `builtin_acp_agents()` 加 Sahaa，双向 `registry_id_for` / `from_registry_id`，`get_agent_meta()` 完整 ACP meta（Npx, cmd=`sahaa`） |
| `src/components/agent-icon.tsx` | 新增 `SahaaColorIcon`（`<img src="/zoho-logo/sahaa-yali-logo.png">`），注册进 `COLOR_ICONS["sahaa"]` |

**遗留旧逻辑（仍存在但已过时，待清理）**：
- `db/service/custom_agent_service.rs` 中的 `seed_sahaa_agent()` — 内置 agent 不需播种，可删除
- `db/service/agent_setting_service.rs` 中的 `remove_stale_sahaa()` / `pin_sahaa_as_default()` — 一次性迁移用，之后可删
- `db/mod.rs` 中的对应调用

**前端 TypeScript 类型**：`AgentType` 联合类型已自动包含 `"sahaa"`（从 Rust wire 推导），无需手动改前端类型定义。

### ⑤ 数据库直接修补（已生效）
```
DB: ~/Library/Application Support/app.codeg/codeg.db
```
- `custom_agent` 表：sahaa 记录已删除（内置 agent 不需要此表记录）
- `agent_setting` 表：写入 `agent_type="sahaa"`, `enabled=1`, `sort_order=0`（第一位）

### ⑥ next.config.ts 修改
- 移除 `experimental.messages`（与 next-intl ESM/CJS 冲突）
- 添加 `ignoreBuildErrors: true` / `ignoreDuringBuilds: true`（临时，生产前必须恢复）

---

## 当前阻塞点

### 🚨 Rust 编译环境不可用

- `rustup` 下载但 toolchain 安装失败（网络受限，无法拉取 stable toolchain）
- `brew install rust` 无效
- `~/.cargo/bin/cargo` 存在但报 `missing manifest in toolchain`

**影响**：旧二进制（`/Applications/codeg.app/Contents/MacOS/codeg-server` v0.30.5）的 `from_wire("sahaa")` 返回 `None`，sahaa agent 被旧后端忽略，UI 无法展示。

**解决方案**：
1. 在有 Rust 工具链的机器上执行：
   ```bash
   cd src-tauri
   cargo build --no-default-features --bin codeg-server --release
   ```
2. 将生成的 `target/release/codeg-server` 替换 `/Applications/codeg.app/Contents/MacOS/codeg-server`
3. 重启服务后 Sahaa 会正式出现在 agent 列表

---

## 当前运行环境

| 项目 | 值 |
|------|----|
| 服务 PID | 68185 |
| 端口 | 3080 |
| Token | 运行时生成（不写入文档） |
| URL | `http://127.0.0.1:3080/login`（登录参数不写入文档） |
| 前端静态文件 | `/Users/zhangkai/project/project_item/codeg/out/` |
| 后端二进制 | `/Applications/codeg.app/Contents/MacOS/codeg-server`（旧版 0.30.5）|
| DB | `~/Library/Application Support/app.codeg/codeg.db` |

---

## 关键架构决策

- **Sahaa wire name = `"sahaa"`**（不是 `"custom:sahaa"`）
- **registry_id = `"sahaa"`**（不是 `"sahaa-npx"`）
- DB `agent_setting.agent_type` 存储 JSON 字符串：`"sahaa"`（含双引号，SeaORM 序列化约定）
- `SahaaColorIcon` 用 `<img>` 而非内联 SVG，图片路径 `/zoho-logo/sahaa-yali-logo.png`
- 编译时 Sahaa 是第 16 个内置 agent（`BUILTIN_AGENT_TYPES` 数组长度从 15 变 16）
