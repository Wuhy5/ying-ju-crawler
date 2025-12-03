//! # 模板渲染器
//!
//! 为 schema::Template 提供渲染能力

use crate::{Result, context::Context, template::TemplateEngine};
use crawler_schema::Template;

/// 模板渲染器 trait
///
/// 为 Template 类型提供渲染能力
pub trait TemplateRenderer {
    /// 渲染模板
    fn render(&self, context: &Context) -> Result<String>;

    /// 提取模板中的变量
    fn extract_variables(&self) -> Vec<String>;

    /// 检查是否为静态模板
    fn is_static(&self) -> bool;

    /// 渲染模板（使用自定义引擎）
    fn render_with_engine(&self, context: &Context, engine: &TemplateEngine) -> Result<String>;
}

impl TemplateRenderer for Template {
    fn render(&self, context: &Context) -> Result<String> {
        let engine = TemplateEngine::new()?;
        self.render_with_engine(context, &engine)
    }

    fn extract_variables(&self) -> Vec<String> {
        let engine = TemplateEngine::new().expect("Failed to create TemplateEngine");
        engine.extract_variables(self.as_str())
    }

    fn is_static(&self) -> bool {
        let engine = TemplateEngine::new().expect("Failed to create TemplateEngine");
        engine.is_static(self.as_str())
    }

    fn render_with_engine(&self, context: &Context, engine: &TemplateEngine) -> Result<String> {
        // 如果是静态模板，直接返回
        if engine.is_static(self.as_str()) {
            return Ok(self.as_str().to_string());
        }

        // 构建 Tera Context
        let mut tera_ctx = tera::Context::new();
        for (key, value) in context.all_variables() {
            tera_ctx.insert(key, value);
        }

        // 渲染模板
        engine.render_str(self.as_str(), &tera_ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_template_render() {
        let template = Template::from("Hello, {{ name }}!");
        let mut context = Context::new();
        context.set("name", json!("Alice"));

        let result = template.render(&context).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_static_template() {
        let template = Template::from("https://example.com");
        assert!(template.is_static());

        let context = Context::new();
        let result = template.render(&context).unwrap();
        assert_eq!(result, "https://example.com");
    }
}
