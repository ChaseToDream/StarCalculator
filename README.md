# ✨ 星辰计算器 StarCalculator

一款基于 Tauri 2.0 + Vue 3 + TypeScript + Rust 开发的跨平台桌面计算器应用，采用模块化架构设计，支持数学计算、数据换算和工程计算三大核心功能模块。

---

## 📋 功能特性

### 📊 数学计算模块

- 基础四则运算（加、减、乘、除）
- 完整的计算器界面
- 支持小数点和清除功能
- 历史记录功能

### 🔄 数据换算模块

- **长度换算**: 米、千米、厘米、毫米、英寸、英尺、码、英里
- **重量换算**: 千克、克、毫克、吨、磅、盎司
- **温度换算**: 摄氏度、华氏度、开尔文
- **进制转换**: 二进制、八进制、十进制、十六进制

### 🔧 工程计算模块

- **面积计算**: 矩形、正方形、圆形、三角形、梯形
- **体积计算**: 立方体、长方体、球体、圆柱体、圆锥体
- **力学计算**: 力、压力、功、功率
- **电路计算**: 欧姆定律（电压、电流）、电功率

### 🎨 UI 特性

- 现代化界面设计
- 深色/浅色主题切换
- 响应式布局
- 流畅的动画效果

---

## 🛠️ 技术栈

- **前端框架**: Vue 3 (Composition API)
- **状态管理**: Pinia
- **构建工具**: Vite
- **桌面框架**: Tauri 2.0
- **后端语言**: Rust
- **开发语言**: TypeScript

---

## 🚀 快速开始

### 环境要求

- Node.js >= 18
- Rust >= 1.70
- Tauri CLI >= 2.0

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 生产构建

```bash
npm run tauri build
```

### 仅前端构建

```bash
npm run build
```

---

## 📁 项目结构

```text
StarCalculator/
├── src/                          # 前端源代码
│   ├── components/               # UI组件
│   │   ├── ModuleSwitcher/      # 模块切换组件
│   │   └── HistoryPanel/        # 历史记录组件
│   ├── modules/                  # 功能模块
│   │   ├── math/                # 数学计算模块
│   │   ├── converter/           # 数据换算模块
│   │   └── engineering/         # 工程计算模块
│   ├── stores/                   # Pinia状态管理
│   │   └── calculatorStore.ts
│   ├── styles/                   # 全局样式
│   │   └── global.css
│   ├── types/                    # TypeScript类型定义
│   │   └── index.ts
│   ├── App.vue                   # 根组件
│   └── main.ts                   # 应用入口
├── src-tauri/                    # Tauri后端
│   ├── src/
│   │   ├── commands/            # Tauri命令
│   │   │   ├── math.rs
│   │   │   ├── converter.rs
│   │   │   └── engineering.rs
│   │   ├── modules/             # Rust计算模块
│   │   │   ├── math/
│   │   │   ├── converter/
│   │   │   └── engineering/
│   │   ├── lib.rs               # 库入口
│   │   └── main.rs              # 应用入口
│   ├── capabilities/            # 权限配置
│   └── tauri.conf.json          # Tauri配置
└── package.json
```

---

## 🔌 API 说明

### 数学计算 API

```typescript
// 加法
await invoke("math_add", { a: 1, b: 2 });

// 减法
await invoke("math_subtract", { a: 5, b: 3 });

// 乘法
await invoke("math_multiply", { a: 4, b: 5 });

// 除法
await invoke("math_divide", { a: 10, b: 2 });

// 三角函数
await invoke("math_sin", { x: Math.PI / 2 });
await invoke("math_cos", { x: 0 });
await invoke("math_tan", { x: Math.PI / 4 });

// 对数和幂运算
await invoke("math_ln", { x: Math.E });
await invoke("math_log10", { x: 100 });
await invoke("math_pow", { base: 2, exponent: 3 });
await invoke("math_sqrt", { x: 16 });

// 阶乘
await invoke("math_factorial", { n: 5 });
```

### 数据换算 API

```typescript
// 长度换算
await invoke("converter_length", { value: 1, from: "meter", to: "kilometer" });

// 重量换算
await invoke("converter_weight", { value: 1, from: "kilogram", to: "pound" });

// 温度换算
await invoke("converter_temperature", {
  value: 0,
  from: "celsius",
  to: "fahrenheit",
});

// 进制转换
await invoke("converter_number_base", {
  value: "255",
  from: "decimal",
  to: "hexadecimal",
});
```

### 工程计算 API

```typescript
// 面积计算
await invoke("engineering_rectangle_area", { width: 5, height: 3 });
await invoke("engineering_circle_area", { radius: 5 });

// 体积计算
await invoke("engineering_sphere_volume", { radius: 3 });
await invoke("engineering_cylinder_volume", { radius: 2, height: 5 });

// 力学计算
await invoke("engineering_force", { mass: 10, acceleration: 9.8 });
await invoke("engineering_power", { work_val: 1000, time: 10 });

// 电路计算
await invoke("engineering_ohms_law_voltage", { current: 2, resistance: 10 });
await invoke("engineering_electrical_power", { voltage: 220, current: 5 });
```

---

## 🔧 模块扩展指南

### 添加新的计算模块

1. **在 Rust 后端添加模块**:
   - 在 `src-tauri/src/modules/` 下创建新模块文件夹
   - 实现计算逻辑
   - 在 `src-tauri/src/commands/` 下创建对应命令文件
   - 在 `src-tauri/src/lib.rs` 中注册新命令

2. **在前端添加模块**:
   - 在 `src/modules/` 下创建新的 Vue 组件
   - 在 `src/App.vue` 中引入并注册新模块
   - 在 `src/components/ModuleSwitcher/index.vue` 中添加模块切换按钮

---

## 📝 开发说明

### 前后端通信

- 使用 `@tauri-apps/api/core` 的 `invoke` 函数调用 Rust 命令
- Rust 端使用 `#[tauri::command]` 宏暴露函数
- 数据通过 JSON 序列化自动转换

### 状态管理

- 使用 Pinia 进行全局状态管理
- 主题切换、模块选择、历史记录等状态统一管理

---

## 🎯 质量标准

- ✅ 跨平台支持（Windows、macOS、Linux）
- ✅ 离线使用，无需网络连接
- ✅ 启动时间 < 3秒
- ✅ 内存占用优化
- ✅ 完整的错误处理机制
- ✅ 模块化架构，易于扩展

---

## 📄 许可证

MIT License

---

## 🙏 致谢

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Vite](https://vitejs.dev/)
