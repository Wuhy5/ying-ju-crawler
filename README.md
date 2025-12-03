# Ying-Ju Crawler Schema

## 项目概述

Ying-Ju-Crawler 是一个专为可视化规则编辑器设计的媒体爬虫规范定义和实现.

## 项目结构

- `crates/schema`：定义爬虫规则的核心数据结构和 JSON Schema 生成
- `crates/runtime`：实现爬虫规则的运行时逻辑，如模板渲染、配置合并和验证

## 目录结构

```plain
crates/schema/
├── Cargo.toml
└── src/
    ├── lib.rs                    # 公共导出
    ├── core.rs                   # CrawlerRule 顶级结构
    ├── extract.rs                # ExtractStep, FieldExtractor 提取流程
    ├── template.rs               # Template 字符串类型
    ├── script.rs                 # Script 脚本调用类型（支持内联/文件/URL）
    ├── config/                   # HttpConfig, Meta, ChallengeConfig
    │   ├── mod.rs
    │   ├── http.rs               # HTTP 配置
    │   ├── meta.rs               # 元数据配置
    │   └── challenge.rs          # 人机验证处理配置（Cloudflare 等）
    ├── fields/                   # 字段规则：VideoDetailFields, BookContentFields 等
    │   ├── mod.rs
    │   ├── video_detail_fields.rs
    │   ├── book_content_fields.rs
    │   └── ...
    └── flow/                     # 流程定义：SearchFlow, DetailFlow, DiscoveryFlow, LoginFlow 等
        ├── mod.rs
        ├── search.rs             # 搜索流程
        ├── detail.rs             # 详情流程
        ├── discovery.rs          # 发现页流程（分类/筛选）
        ├── login.rs              # 登录流程（脚本/WebView/凭证三种模式）
        ├── content.rs            # 内容页流程
        └── component.rs          # 可重用组件
```

```plain
crates/runtime/
├── Cargo.toml
└── src/
    ├── lib.rs                    # 公共导出
    ├── error.rs                  # 统一错误类型
    │
    ├── context/                  # 执行上下文管理
    │   ├── mod.rs
    │   ├── variable.rs           # 变量存储与访问
    │   └── state.rs              # 流程状态管理
    │
    ├── template/                 # 模板引擎
    │   ├── mod.rs
    │   ├── engine.rs             # Tera 引擎封装（单例+缓存）
    │   ├── renderer.rs           # Template trait 扩展
    │   └── validator.rs          # 模板验证
    │
    ├── http/                     # HTTP 客户端
    │   ├── mod.rs
    │   ├── client.rs             # HTTP 客户端封装（连接池）
    │   ├── config.rs             # HttpConfig 扩展（合并逻辑）
    │   └── request.rs            # 请求构建器
    │
    ├── extractor/                # 数据提取引擎
    │   ├── mod.rs
    │   ├── engine.rs             # 提取引擎核心
    │   ├── executor.rs           # 步骤执行器（策略模式）
    │   ├── selector/             # 选择器实现
    │   │   ├── mod.rs
    │   │   ├── css.rs            # CSS 选择器
    │   │   ├── json.rs           # JSONPath
    │   │   └── regex.rs          # 正则表达式
    │   ├── filter/               # 过滤器实现
    │   │   ├── mod.rs
    │   │   ├── registry.rs       # 过滤器注册表（工厂模式）
    │   │   ├── string.rs         # 字符串过滤器
    │   │   ├── convert.rs        # 类型转换过滤器
    │   │   ├── url.rs            # URL 处理过滤器
    │   │   ├── array.rs          # 数组处理过滤器
    │   │   └── encoding.rs       # 编码处理过滤器
    │   └── value.rs              # 中间值类型（避免频繁转换）
    │
    ├── script/                   # 脚本引擎
    │   ├── mod.rs
    │   ├── engine.rs             # 脚本引擎抽象（支持多引擎）
    │   ├── loader.rs             # 脚本加载器
    │   └── context.rs            # 脚本执行上下文
    │
    ├── webview/                  # WebView 提供者（依赖注入）
    │   ├── mod.rs
    │   ├── provider.rs           # WebViewProvider trait
    │   ├── request.rs            # WebView 请求配置
    │   └── response.rs           # WebView 响应结果
    │
    ├── flow/                     # 流程执行器
    │   ├── mod.rs
    │   ├── executor.rs           # 流程执行器 trait
    │   ├── search.rs             # 搜索流程
    │   ├── detail.rs             # 详情流程
    │   ├── discovery.rs          # 发现流程
    │   ├── content.rs            # 内容流程
    │   └── login.rs              # 登录流程
    │
    ├── crawler/                  # 爬虫运行时（顶层）
    │   ├── mod.rs
    │   ├── runtime.rs            # CrawlerRuntime 主入口
    │   └── builder.rs            # Runtime 构建器（支持 WebView 注入）
    │
    └── util/                     # 工具函数
        ├── mod.rs
        ├── cache.rs              # 缓存工具（LRU）
        └── concurrent.rs         # 并发控制
```

## 主要特性

- 支持使用 JSON 格式或者 TOML 格式来定义爬虫规则，方便用户进行编辑和管理。
- 提供完整的运行时实现，支持模板渲染、HTTP 请求、数据提取和脚本执行等功能。
- 采用模块化设计，方便扩展和维护。

## 版本同步

包版本和规范版本保持一致：

- `Cargo.toml` 中的版本 = 规范版本
- 当更新规范时，同时更新此包版本
