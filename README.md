# Auto Shutdown - 自动关机工具

一个精致的 Windows 桌面自动关机工具，采用 Tauri 2.0 + Vue 3 + Tailwind CSS 构建。

## 功能特性

- ⏱️ **倒计时关机** - 设置 N 分钟后自动关机
- 🔢 **定时关机** - 设置具体时间（如 23:00）自动关机
- ⚡ **立即关机** - 到时间立即执行，无 Windows 系统通知
- 🎯 **快速预设** - 15分钟、30分钟、1小时等常用时间快捷按钮
- 📊 **进度显示** - 可视化倒计时进度环
- 🔔 **系统托盘** - 最小化到托盘，后台运行
- 💾 **持久化设置** - 记住用户偏好设置

## 技术栈

- **桌面框架**: Tauri 2.0
- **前端框架**: Vue 3 + TypeScript
- **样式框架**: Tailwind CSS
- **图标库**: Lucide Vue

## 开发环境

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 开发命令

```bash
# 安装依赖
npm install

# 运行开发服务器
npm run tauri dev

# 构建生产版本
npm run tauri build
```

## 使用说明

1. 以管理员身份运行应用（关机命令需要管理员权限）
2. 选择预设时间或自定义输入时间
3. 点击"开始倒计时"
4. 可选择"开始倒计时后自动隐藏到托盘"
5. 可随时取消关机计划

## 窗口特性

- 固定宽度 400px，高度自适应内容
- 无边框设计，自定义标题栏
- 深色优雅主题
- 最小化/关闭按钮支持

---

**⚠️ 警告**: 关机将强制关闭所有程序，请务必提前保存工作！

---

*最后更新: 2026-05-25 14:36:21*
