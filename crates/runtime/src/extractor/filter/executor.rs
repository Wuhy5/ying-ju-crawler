//! # 过滤器执行器

use crate::{
    Result,
    context::Context,
    extractor::{SharedValue, filter::registry::global_registry, value::ExtractValueData},
};
use crawler_schema::extract::FilterStep;
use serde_json::Value;
use std::sync::Arc;

/// 过滤器执行器
pub struct FilterExecutor;

impl FilterExecutor {
    /// 解析过滤器管道字符串
    ///
    /// 例如：`"trim | lower | replace(a, b)"`
    fn parse_pipeline(pipeline: &str) -> Vec<(String, Vec<Value>)> {
        let mut filters = Vec::new();

        for part in pipeline.split('|') {
            let part = part.trim();
            if let Some(open_paren) = part.find('(') {
                // 带参数的过滤器
                let name = part[..open_paren].trim().to_string();
                let args_str = &part[open_paren + 1..part.len() - 1];
                let args: Vec<Value> = args_str
                    .split(',')
                    .map(|s| Value::String(s.trim().to_string()))
                    .collect();
                filters.push((name, args));
            } else {
                // 无参数的过滤器
                filters.push((part.to_string(), vec![]));
            }
        }

        filters
    }

    /// 执行过滤器
    pub fn execute(
        filter: &FilterStep,
        input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        let registry = global_registry();
        let mut current = Arc::new(input.clone());

        match filter {
            FilterStep::Pipeline(pipeline) => {
                let filters = Self::parse_pipeline(pipeline);
                for (name, args) in filters {
                    current = registry.apply(&name, current, &args)?;
                }
            }
            FilterStep::List(filters) => {
                for filter_config in filters {
                    let args = filter_config.args.as_deref().unwrap_or(&[]);
                    current = registry.apply(&filter_config.name, current, args)?;
                }
            }
        }

        Ok(current)
    }
}
