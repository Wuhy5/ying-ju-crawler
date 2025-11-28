//! 运行时模块
//!
//! 包含爬虫规则的运行时实现：
//! - 模板渲染
//! - 规则验证
//! - 执行器（未来扩展）

pub mod template;
pub mod validation;

pub use template::{RenderOptions, TemplateExt, escape_html};
pub use validation::RuleValidate;
