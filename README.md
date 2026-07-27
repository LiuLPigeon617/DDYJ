# DDYJ 地震预警

基于 **Tauri 2 + Rust + Vue 3** 的 iOS 地震预警应用。

## 功能

- **实时地震数据** — 同时接入 USGS (全球) 和 CENC (中国地震台网中心) 双数据源
- **地震列表** — 按时间排序，震级颜色标注，相对时间显示
- **地图视图** — SVG 世界地图，地震位置可视化，震级动画标记
- **实时监控** — 后台定时轮询，新地震自动推送通知
- **统计概览** — 最大震级、1小时/24小时/总计统计
- **设置面板** — 震级阈值、轮询间隔、通知/声音开关

## 数据源

| 数据源 | 说明 | 覆盖范围 |
|--------|------|----------|
| **USGS** | 美国地质调查局实时 GeoJSON API | 全球 |
| **CENC** | 中国地震台网中心 (ceic.ac.cn) | 中国及邻区 |

- USGS API: `https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_day.geojson`
- CENC API: `https://www.ceic.ac.cn/ajax/speedsearch`

## 技术栈

| 层 | 技术 |
|----|------|
| 前端 | Vue 3 + TypeScript + Pinia + Vite |
| 后端 | Rust + Tokio + Reqwest |
| 框架 | Tauri 2 (Mobile / iOS) |
| 通知 | tauri-plugin-notification |

## 项目结构

```
DDYJ/
├── src/                    # Vue 前端
│   ├── components/         # UI 组件
│   ├── stores/             # Pinia 状态管理
│   ├── types/              # TypeScript 类型定义
│   └── assets/             # 样式文件
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── lib.rs          # Tauri 命令注册
│   │   ├── earthquake.rs   # 地震数据获取 (USGS + CENC)
│   │   └── monitor.rs      # 后台监控逻辑
│   ├── capabilities/       # 权限配置
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
├── package.json
└── vite.config.ts
```

## 开发

### 前端开发 (Windows 可用)

```bash
cd D:\Repo\DDYJ
npm install
npm run dev          # 启动 Vite 开发服务器
npm run build        # 构建前端到 dist/
```

### iOS 构建 (需要 macOS + Xcode)

> **⚠️ 重要**: iOS 应用必须在 macOS 上使用 Xcode 编译。Windows 可以开发前端代码和 Rust 逻辑，但最终构建 iOS 包需要 Mac 环境。

```bash
# 在 macOS 上:
cd D:\Repo\DDYJ
npm install
npx tauri ios init      # 初始化 iOS Xcode 工程 (仅首次)
npx tauri ios dev       # 在模拟器/真机上调试
npx tauri ios build     # 构建 iOS 发布包
```

### 前提条件

- **Node.js** >= 18
- **Rust** (rustup 安装)
- **macOS + Xcode** (iOS 构建必需)
- **Apple Developer 账号** (真机部署/发布必需)
