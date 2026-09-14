# 项目说明 — Codez

## 项目名称
Codez（内部代码名 codeg）

## 目标
一个多智能体编码工作台，将多个智能体（Claude Code、Codex CLI、Gemini CLI等）统一到一个工作区中，支持会话聚合和多智能体协作。

## 核心需求

1. **多 Agent 集成**：支持 16 个内置 agent，另支持用户自定义 ACP agent
2. **双模式运行**：桌面应用（Tauri）+ 服务器/Docker 部署
3. **Zoho 定制**：应用品牌改为 "codez"，图标改为 Zoho 风格，Sahaa 为首位默认 agent
4. **ACP 协议**：支持 Agent Client Protocol 进行多 agent 协作
5. **实时通信**：WebSocket 广播，事件驱动 UI 更新

## Zoho 定制内容

- **App 名称**：codez
- **图标**：Zoho 官方 SVG logo
- **Sahaa Agent**：Zoho 自带的 AI 编程助手，内置为第一个默认 agent
- **欢迎弹窗**：首次启动展示 Sahaa 安装引导

## 范围

- 根目录: `/Users/zhangkai/project/project_item/codeg/`
- 主要工作包：`src/`（Next.js）、`src-tauri/`（Rust）

## 成功标准

- Sahaa 在 UI 中展示为首位内置 agent，图标为 Sahaa yali logo
- 所有 UI 文字显示 "codez"，所有图标为 Zoho 风格
- 桌面 + 服务器模式均可正常工作
