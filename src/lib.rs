//! # 影视软件爬虫规则规范 (V-Editor-Optimized)
//!
//! 该规范专为可视化规则编辑器设计，强调结构的明确性、步骤的原子性和流程的模块化。
//! 其核心思想包括：
//! 1. **原子化步骤 (Atomic Steps)**：每个 `Step` 只执行一个最小化的、单一的操作。
//! 2. **显式数据流 (Explicit Data Flow)**：每个步骤都明确定义其输入 (`input`) 和输出 (`output`)，
//!    数据在变量上下文中的流动清晰可见。
//! 3. **可重用组件 (Reusable Components)**：允许将常用的管道封装成可复用的组件。
//! 4. **流程驱动 (Flow-Driven)**：以用户可自定义的"流程 (`Flow`)"为核心， 替代固定的 `discover` 和
//!    `search` 入口。
//!
//! ## 模块结构
//!
//! - `schema`: 纯数据结构定义（用于 JSON Schema 生成和序列化）
//! - `error`: 通用错误类型定义
//! - `runtime`: 运行时工具（模板渲染、规则验证等）
//!
//! ## 模板字符串规范
//!
//! 所有输入字段均采用模板字符串格式，支持变量插值。
//!
//! - 语法：`{{ variable }}` 用于插入变量值。
//! - 示例：
//!   - `https://example.com/search?q={{ keyword }}`
//!   - `User-Agent: MyBot/{{ version }}`
//! - 支持表达式与嵌套：如 `{{ user.name }}`、`{{ items[0] }}`
//! - 运行时会自动渲染所有模板字符串。
//! - 字段注释中如"模板字符串，详见顶部规范说明"即指此规范。

// 通用错误模块（最外层）
pub mod error;

// Schema 定义模块（纯数据结构）
pub mod schema;

// 运行时模块（渲染、验证等）
pub mod runtime;

// 重新导出常用类型
pub use error::{CrawlerError, ValidationErrors, ValidationResult};
pub use runtime::{RenderOptions, RuleValidate, TemplateExt};
pub use schema::*;
