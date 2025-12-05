//! # 正则表达式选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::value::{ExtractValueData, SharedValue},
};
use crawler_schema::extract::RegexStep;
use std::sync::Arc;

/// 正则表达式选择器执行器
pub struct RegexSelectorExecutor;

impl RegexSelectorExecutor {
    /// 执行正则匹配
    pub fn execute(
        regex: &RegexStep,
        input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        // 获取字符串
        let text = input
            .as_str()
            .ok_or_else(|| RuntimeError::Extraction("Regex requires string input".to_string()))?;

        // 解析正则配置
        let (pattern, group, global) = match regex {
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
            let matches: Vec<SharedValue> = re
                .captures_iter(text)
                .filter_map(|cap| {
                    cap.get(group).map(|m| {
                        Arc::new(ExtractValueData::String(Arc::from(
                            m.as_str().to_string().into_boxed_str(),
                        )))
                    })
                })
                .collect();

            if matches.is_empty() {
                Ok(Arc::new(ExtractValueData::Null))
            } else {
                Ok(Arc::new(ExtractValueData::Array(Arc::new(matches))))
            }
        } else {
            // 单次匹配
            let result = re.captures(text).and_then(|cap| cap.get(group)).map(|m| {
                Arc::new(ExtractValueData::String(Arc::from(
                    m.as_str().to_string().into_boxed_str(),
                )))
            });

            match result {
                Some(s) => Ok(s),
                None => Ok(Arc::new(ExtractValueData::Null)),
            }
        }
    }
}
