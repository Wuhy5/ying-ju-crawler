//! # 正则表达式选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use crawler_schema::RegexStep;

/// 正则表达式选择器执行器
pub struct RegexSelectorExecutor {
    regex: RegexStep,
}

impl RegexSelectorExecutor {
    pub fn new(regex: RegexStep) -> Self {
        Self { regex }
    }
}

impl StepExecutor for RegexSelectorExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // 获取字符串
        let text = input
            .as_string()
            .ok_or_else(|| RuntimeError::Extraction("Regex requires string input".to_string()))?;

        // 解析正则配置
        let (pattern, group, global) = match &self.regex {
            RegexStep::Simple(p) => (p.as_str(), 1, false),
            RegexStep::WithOptions {
                pattern,
                group,
                global,
            } => (pattern.as_str(), *group, *global),
        };

        // 编译正则表达式
        let re = regex::Regex::new(pattern)
            .map_err(|e| RuntimeError::Extraction(format!("Invalid regex pattern: {}", e)))?;

        if global {
            // 全局匹配
            let matches: Vec<String> = re
                .captures_iter(&text)
                .filter_map(|cap| cap.get(group).map(|m| m.as_str().to_string()))
                .collect();

            if matches.is_empty() {
                Ok(ExtractValue::Null)
            } else {
                Ok(ExtractValue::Array(
                    matches.into_iter().map(ExtractValue::String).collect(),
                ))
            }
        } else {
            // 单次匹配
            let result = re
                .captures(&text)
                .and_then(|cap| cap.get(group))
                .map(|m| m.as_str().to_string());

            match result {
                Some(s) => Ok(ExtractValue::String(s)),
                None => Ok(ExtractValue::Null),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_simple() {
        let executor = RegexSelectorExecutor::new(RegexStep::Simple(r"(\d+)".to_string()));
        let input = ExtractValue::String("age: 25".to_string());
        let context = Context::new();

        let result = executor.execute(&input, &context).unwrap();
        assert_eq!(result.as_string(), Some("25".to_string()));
    }
}
