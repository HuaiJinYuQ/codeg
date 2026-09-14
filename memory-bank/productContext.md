# 产品背景 — Codez

## 为什么存在

Codez 是 Zoho 定制版的多智能体编码工作台，基于开源项目 "codeg" 构建。它将多个 AI 编程助手统一到一个界面，使开发者无需切换工具即可与不同 AI 协作。

## 解决的问题

- 多个 AI 编程工具分散，需要不断切换
- 会话历史分散在各工具，难以聚合查阅
- 多 agent 协作时缺乏统一的封装层（ACP 协议）

## Sahaa Agent（Zoho 独占）

- Zoho 自带的 AI 编程助手
- **内置为首位（sort_order=0）、默认开启**
- wire name: `"sahaa"`，registry_id: `"sahaa"`
- 分发方式: npx，cmd=`sahaa`
- 图标: `/zoho-logo/sahaa-yali-logo.png`（弹性功能图标）
- 首次启动展示 `SahaaWelcomeDialog`

## 用户体验目标

1. 开劯即见 Sahaa，作为首推 agent
2. 同一页面管理所有 AI agent的会话
3. agent 图标风格统一（Sahaa 使用 yali logo，其他内置 agent 各有其品牌图标）
4. 首次启动引导用户安装/使用 Sahaa

## 部署场景

- **桌面（macOS/Windows/Linux）**：Tauri 2 应用包
- **服务器**：docker-compose 或独立二进制
- **现有测试环境**：`/Applications/codeg.app` + 新前端静态文件，运行在 `:3080`
