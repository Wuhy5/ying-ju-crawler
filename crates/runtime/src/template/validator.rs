//! # 模板验证器
//!
//! 提供模板语法验证功能

use crate::{Result, template::TemplateEngine};
use crawler_schema::Template;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_template() {
        let template = Template::from("Hello, {{ name }}!");
        assert!(template.validate().is_ok());
    }

    #[test]
    fn test_invalid_template() {
        let template = Template::from("Hello, {{ name }!"); // 缺少闭合
        assert!(template.validate().is_err());
    }
}
