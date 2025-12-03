# Ying-Ju Crawler Schema - Copilot 开发指南

## 项目概述

Ying-Ju 媒体爬虫规范库，包含两个核心 crate：
- **`crawler-schema`**：纯数据结构定义（JSON Schema 生成、序列化）
- **`crawler-runtime`**：运行时逻辑（模板渲染、配置合并、验证）

## 架构设计

### Schema / Runtime 分离模式（关键）
```
crawler-schema: 只包含 #[derive(Serialize, Deserialize, JsonSchema)] 的纯数据结构
crawler-runtime: 通过扩展 trait 为 schema 类型添加运行时方法
```
示例：`Template` 在 schema 中只是字符串包装，运行时功能通过 `TemplateExt` trait 实现。

### 模块结构
```
schema/
├── core.rs          # CrawlerRule 顶级结构
├── extract.rs       # ExtractStep, FieldExtractor 提取流程
├── template.rs      # Template 字符串类型
├── script.rs        # Script 脚本调用（内联/文件/URL）
├── config/          # HttpConfig, Meta, ChallengeConfig
│   ├── http.rs      # HTTP 配置
│   ├── meta.rs      # 元数据
│   └── challenge.rs # 人机验证配置（Cloudflare 等）
├── fields/          # 字段规则：VideoDetailFields, BookContentFields 等
└── flow/            # 流程定义
    ├── search.rs    # 搜索流程
    ├── detail.rs    # 详情流程
    ├── discovery.rs # 发现页（分类/筛选）
    ├── login.rs     # 登录（Script/WebView/Credential）
    └── content.rs   # 内容页

runtime/
├── template/        # 模板引擎（Tera）
├── http/            # HTTP 客户端
├── extractor/       # 数据提取引擎
├── script/          # 脚本执行引擎（Rhai/JS）
├── webview/         # WebView 提供者（依赖注入）
│   ├── provider.rs  # WebViewProvider trait
│   ├── request.rs   # 请求配置
│   └── response.rs  # 响应结果
├── flow/            # 流程执行器
├── crawler/         # Runtime 主入口
└── context/         # 执行上下文
```

### 数据流模型
```
CrawlerRule → Flow (search/detail/discovery) → FieldExtractor → ExtractStep[]
                  ↓
            ChallengeHandler → WebViewProvider (依赖注入)
```
- 每个 `ExtractStep` 只执行一个原子操作（css/json/regex/filter/attr/index）
- 步骤链式执行，前一步输出作为后一步输入
- WebView 能力通过 trait 注入，Runtime 不直接依赖 GUI 库

## 开发约定

### Serde 标注（必须遵守）
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]  // 所有结构体必加
pub struct MyType {
    pub required_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]  // 可选字段
    pub optional_field: Option<String>,
}

// 枚举：内部标签或 snake_case
#[serde(tag = "type", rename_all = "snake_case")]
#[serde(untagged)]  // 或无标签联合类型
```

### 扩展 trait 模式
为 schema 类型添加运行时功能时，在 runtime 模块定义 trait：
```rust
// runtime/template.rs
pub trait TemplateExt {
    fn render(&self, context: &HashMap<String, Value>) -> Result<String, RuntimeError>;
}

impl TemplateExt for Template { /* ... */ }
```

### 模板字符串规范
所有 `Template` 类型支持 Tera 语法：
- 变量插值：`{{ variable }}`、`{{ user.name }}`、`{{ items[0] }}`
- 约定变量：`{{ detail_url }}`、`{{ keyword }}`、`{{ page }}`

### 错误处理
- Schema 阶段：`SchemaError`（解析错误）
- Runtime 阶段：`RuntimeError`（渲染、验证、执行错误）
- 使用 `thiserror` 定义，中文错误消息

### 项目规范
- 代码风格遵循 Rust 官方规范
- **完成代码任务后必须执行 `cargo +nightly fmt` 格式化代码**
- 使用 Clippy 检查代码质量（`cargo clippy`）
- 部分警告（如未使用的代码 `dead_code`、`unused_variables`）可以暂时忽略

### 开发设计
- 多使用 trait 和泛型，减少重复代码
- 保持模块内聚，职责单一
- 明确区分 schema 和 runtime 逻辑
- 多使用设计模式（如策略模式、工厂模式）提升扩展性

### 性能
- 避免不必要的内存分配和拷贝
- 使用引用传递大数据结构
- 缓存重复计算结果（如模板编译）
- 开始设计时就应该考虑性能影响
- 多使用高性能的库代替低性能的实现

## 常用命令

```bash
cargo build                      # 构建所有 crate
cargo test                       # 运行所有测试
cargo +nightly fmt               # 格式化代码（必须使用 nightly）
cargo clippy                     # 代码质量检查
cargo run --bin generate_schema  # 生成 JSON Schema
```

## 开发流程

完成代码任务后，按以下顺序执行：
1. `cargo +nightly fmt` - 格式化代码
2. `cargo clippy` - 检查代码质量（可忽略 `dead_code`、`unused_variables` 等警告）
3. `cargo test` - 运行测试（如有）

## 版本管理

`Cargo.toml` 版本 = 规范版本，更新规范时同步更新版本号。
