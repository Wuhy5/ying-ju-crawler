//! # 模板引擎
//!
//! Tera 引擎封装，提供单例和缓存支持

use crate::{Result, error::RuntimeError};
use std::sync::{Arc, RwLock};
use tera::Tera;

/// 模板引擎
///
/// 封装 Tera 引擎，提供缓存和优化
#[derive(Clone)]
pub struct TemplateEngine {
    tera: Arc<RwLock<Tera>>,
}

impl TemplateEngine {
    /// 创建新的模板引擎
    pub fn new() -> Result<Self> {
        let tera = Tera::default();

        Ok(Self {
            tera: Arc::new(RwLock::new(tera)),
        })
    }

    /// 渲染模板字符串
    ///
    /// # 参数
    /// - `template`: 模板字符串
    /// - `context`: 上下文变量
    pub fn render_str(&self, template: &str, context: &tera::Context) -> Result<String> {
        self.tera
            .write()
            .map_err(|e| RuntimeError::TemplateRender {
                message: format!("Failed to acquire write lock: {}", e),
            })?
            .render_str(template, context)
            .map_err(|e| RuntimeError::TemplateRender {
                message: format!("Failed to render template '{}': {}", template, e),
            })
    }

    /// 提取模板中的变量
    ///
    /// 解析模板字符串，返回所有使用的变量名
    pub fn extract_variables(&self, template: &str) -> Vec<String> {
        let mut variables = Vec::new();

        // 简单的正则匹配 {{ variable }}
        let re =
            regex::Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_\.]*)\s*(?:\|[^}]*)?\}\}").unwrap();

        for cap in re.captures_iter(template) {
            if let Some(var) = cap.get(1) {
                let var_name = var.as_str().split('.').next().unwrap_or(var.as_str());
                if !variables.contains(&var_name.to_string()) {
                    variables.push(var_name.to_string());
                }
            }
        }

        variables
    }

    /// 检查是否为静态模板（不含变量）
    pub fn is_static(&self, template: &str) -> bool {
        !template.contains("{{") && !template.contains("{%")
    }
    /// 验证模板语法
    pub fn validate(&self, template: &str) -> Result<()> {
        // 尝试用空上下文渲染，检查语法错误
        let ctx = tera::Context::new();
        match self
            .tera
            .write()
            .map_err(|e| RuntimeError::TemplateRender {
                message: format!("Failed to acquire write lock: {}", e),
            })?
            .render_str(template, &ctx)
        {
            Ok(_) => Ok(()),
            Err(e) if matches!(e.kind, tera::ErrorKind::Msg(_)) => Ok(()),
            Err(e) => Err(RuntimeError::TemplateValidation {
                template: template.to_string(),
                error: e.to_string(),
            }),
        }
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default TemplateEngine")
    }
}
