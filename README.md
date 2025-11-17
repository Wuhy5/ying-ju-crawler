# Crawler Schema 包

这个包提供了媒体爬虫规范的 Rust 类型定义和 JSON Schema 自动生成工具。

## 功能

### 1. Rust 类型定义
`src/lib.rs` 包含完整的 Rust 结构定义：
- `RuleFile` - 规则文件根结构
- `Meta` - 元数据
- `HttpConfig` - HTTP 配置
- `Step` - 管道步骤类型
- `ParseRules` - 解析规则
- 以及其他所有支持类型

### 2. JSON Schema 自动生成

一个可执行的 Rust 程序，自动从代码生成 JSON Schema：

```bash
cd packages/schema
cargo run --bin generate_schema
```

这会在 `../crawler-docs/docs/schema/schema.json` 生成最新的 Schema 文件。

## 优势

✅ **Single Source of Truth（单一真实来源）**
- 所有类型定义在 Rust 中
- Schema 自动从 Rust 代码生成
- 永远不会产生不一致

✅ **易于维护**
- 修改 Rust 类型定义后，运行生成命令即可更新 Schema
- 无需手动编辑 JSON Schema

✅ **类型安全**
- Rust 编译器保证类型一致性
- serde 自动处理序列化

✅ **自动文档**
- Rust 文档注释自动转为 JSON Schema 描述

## 使用流程

### 1. 添加新的步骤类型

在 `src/lib.rs` 的 `Step` 枚举中添加：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Step {
    // 现有类型...
    
    // 新类型
    NewType {
        param1: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        param2: Option<String>,
    },
}
```

然后在 `src/bin/generate_schema.rs` 中添加对应的 schema 生成函数：

```rust
fn new_type_step_schema() -> Value {
    json!({
        "type": "object",
        "required": ["type", "param1"],
        "properties": {
            "type": { "const": "newtype" },
            "param1": { "type": "string" },
            "param2": { "type": "string" }
        }
    })
}
```

然后在 `step_schema()` 函数的 `oneOf` 中添加：

```rust
fn step_schema() -> Value {
    json!({
        "oneOf": [
            // ... 现有
            new_type_step_schema(),  // 新增
        ]
    })
}
```

### 2. 添加新的媒体类型

在 `src/lib.rs` 的 `MediaType` 枚举中添加：

```rust
pub enum MediaType {
    Video,
    Audio,
    Book,
    Manga,
    Podcast,  // 新类型
}
```

在 `src/bin/generate_schema.rs` 中更新 media_type enum：

```rust
"media_type": {
    "type": "string",
    "enum": ["video", "audio", "book", "manga", "podcast"],  // 添加新值
    ...
}
```

### 3. 生成新的 Schema

```bash
cd packages/schema
cargo run --bin generate_schema
```

## 验证规则文件

使用生成的 Schema 验证 TOML 规则文件：

```bash
# 将 TOML 转换为 JSON
python3 -c "import toml, json; \
    data = toml.load('rule.toml'); \
    json.dump(data, open('rule.json', 'w'))"

# 使用 ajv 验证
npm install -g ajv-cli
ajv validate -s ../crawler-docs/docs/schema/schema.json -d rule.json
```

或在 Rust 中使用：

```rust
use crawler_schema::RuleFile;

let content = std::fs::read_to_string("rule.toml")?;
let rule: RuleFile = toml::from_str(&content)?;
// 类型检查保证了有效性
```

## 文件结构

```
packages/schema/
├── Cargo.toml                      # 包定义
├── src/
│   ├── lib.rs                     # Rust 类型定义
│   └── bin/
│       └── generate_schema.rs     # Schema 生成工具
└── README.md                      # 本文件
```

## 版本同步

包版本和规范版本保持一致：
- `Cargo.toml` 中的版本 = 规范版本
- 当更新规范时，同时更新此包版本

## 扩展建议

未来可以考虑：
1. **更多步骤类型** - 随规范演进添加
2. **动态 Schema** - 基于运行时配置生成
3. **多语言支持** - 为不同语言生成相应的类型定义（Python、TypeScript 等）
4. **自动化测试** - 验证 Schema 的完整性和一致性
