//! 错误处理模块
//! 
//! 提供爬虫规则相关的自定义错误类型，包括：
//! - 模板错误
//! - 验证错误
//! - 配置错误
//! - 运行时错误

use thiserror::Error;

/// 爬虫规则错误类型
#[derive(Debug, Error, Clone)]
pub enum CrawlerError {
    // --- 模板相关错误 ---
    /// 模板语法错误
    #[error("模板语法错误: {message}")]
    TemplateSyntax { message: String },

    /// 模板渲染错误
    #[error("模板渲染错误: {message}")]
    TemplateRender { message: String },

    /// 模板变量未定义
    #[error("模板变量 '{variable}' 未定义")]
    UndefinedVariable { variable: String },

    // --- 验证相关错误 ---
    /// 标识符无效
    #[error("无效的标识符 '{identifier}': {reason}")]
    InvalidIdentifier { identifier: String, reason: String },

    /// 组件未定义
    #[error("组件 '{component}' 未定义")]
    UndefinedComponent { component: String },

    /// 流程未定义
    #[error("流程 '{flow}' 未定义")]
    UndefinedFlow { flow: String },

    /// 循环引用检测
    #[error("检测到循环引用: {path}")]
    CircularReference { path: String },

    /// 字段映射错误
    #[error("字段映射错误: 目标字段 '{field}' 在模型 '{model}' 中不存在")]
    InvalidFieldMapping { field: String, model: String },

    /// 管道验证错误
    #[error("管道验证错误 (步骤 {step_index}): {message}")]
    PipelineValidation { step_index: usize, message: String },

    // --- 配置相关错误 ---
    /// 配置缺失
    #[error("缺少必需的配置项: {field}")]
    MissingConfig { field: String },

    /// 配置值无效
    #[error("配置项 '{field}' 的值无效: {reason}")]
    InvalidConfigValue { field: String, reason: String },

    /// 脚本模块未定义
    #[error("脚本模块 '{module}' 未定义")]
    UndefinedScriptModule { module: String },

    /// 脚本函数未定义
    #[error("脚本函数 '{module}.{function}' 未定义")]
    UndefinedScriptFunction { module: String, function: String },

    // --- 运行时相关错误 ---
    /// 超出资源限制
    #[error("超出资源限制: {limit_type} (当前: {current}, 最大: {max})")]
    ResourceLimitExceeded {
        limit_type: String,
        current: usize,
        max: usize,
    },

    /// 递归深度超限
    #[error("递归深度超出限制 (当前: {current}, 最大: {max})")]
    RecursionLimitExceeded { current: usize, max: usize },

    /// 执行超时
    #[error("执行超时: {operation} (耗时: {elapsed_ms}ms, 限制: {limit_ms}ms)")]
    ExecutionTimeout {
        operation: String,
        elapsed_ms: u64,
        limit_ms: u64,
    },

    // --- 其他错误 ---
    /// JSON解析错误
    #[error("JSON解析错误: {0}")]
    JsonParse(String),

    /// IO错误
    #[error("IO错误: {0}")]
    Io(String),

    /// 多个验证错误
    #[error("验证发现 {count} 个错误")]
    MultipleErrors {
        count: usize,
        errors: Vec<CrawlerError>,
    },
}

impl From<serde_json::Error> for CrawlerError {
    fn from(e: serde_json::Error) -> Self {
        CrawlerError::JsonParse(e.to_string())
    }
}

impl From<std::io::Error> for CrawlerError {
    fn from(e: std::io::Error) -> Self {
        CrawlerError::Io(e.to_string())
    }
}

/// 验证结果类型
pub type ValidationResult<T> = Result<T, CrawlerError>;

/// 验证错误收集器
#[derive(Debug, Default, Clone)]
pub struct ValidationErrors {
    errors: Vec<CrawlerError>,
}

impl ValidationErrors {
    /// 创建新的错误收集器
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// 添加错误
    pub fn push(&mut self, error: CrawlerError) {
        self.errors.push(error);
    }

    /// 添加多个错误
    pub fn extend(&mut self, errors: impl IntoIterator<Item = CrawlerError>) {
        self.errors.extend(errors);
    }

    /// 是否有错误
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// 错误数量
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// 获取所有错误
    pub fn into_errors(self) -> Vec<CrawlerError> {
        self.errors
    }

    /// 转换为Result
    pub fn into_result(self) -> ValidationResult<()> {
        if self.errors.is_empty() {
            Ok(())
        } else if self.errors.len() == 1 {
            Err(self.errors.into_iter().next().unwrap())
        } else {
            Err(CrawlerError::MultipleErrors {
                count: self.errors.len(),
                errors: self.errors,
            })
        }
    }

    /// 迭代错误
    pub fn iter(&self) -> impl Iterator<Item = &CrawlerError> {
        self.errors.iter()
    }
}

impl IntoIterator for ValidationErrors {
    type Item = CrawlerError;
    type IntoIter = std::vec::IntoIter<CrawlerError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_errors_empty() {
        let errors = ValidationErrors::new();
        assert!(!errors.has_errors());
        assert!(errors.into_result().is_ok());
    }

    #[test]
    fn test_validation_errors_single() {
        let mut errors = ValidationErrors::new();
        errors.push(CrawlerError::UndefinedVariable {
            variable: "test".to_string(),
        });
        assert!(errors.has_errors());
        
        let result = errors.into_result();
        assert!(matches!(result, Err(CrawlerError::UndefinedVariable { .. })));
    }

    #[test]
    fn test_validation_errors_multiple() {
        let mut errors = ValidationErrors::new();
        errors.push(CrawlerError::UndefinedVariable {
            variable: "a".to_string(),
        });
        errors.push(CrawlerError::UndefinedComponent {
            component: "b".to_string(),
        });
        
        let result = errors.into_result();
        assert!(matches!(result, Err(CrawlerError::MultipleErrors { count: 2, .. })));
    }
}
