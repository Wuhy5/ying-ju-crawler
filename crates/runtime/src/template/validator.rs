//! # 模板验证器
//!
//! 提供模板语法验证功能

use crate::{Result, template::TemplateEngine};
use crawler_schema::template::Template;

/// 模板验证器 trait
pub trait TemplateValidator {
    /// 验证模板语法
    fn validate(&self) -> Result<()>;
}

impl TemplateValidator for Template {
    fn validate(&self) -> Result<()> {
        let engine = TemplateEngine::new()?;
        engine.validate(self.as_str())
    }
}
