//! # 过滤器执行器

use crate::{
    Result,
    context::Context,
    extractor::{ExtractValue, StepExecutor, filter::registry::global_registry},
};
use crawler_schema::FilterStep;
use serde_json::Value;

/// 过滤器执行器
pub struct FilterExecutor {
    filter: FilterStep,
}

impl FilterExecutor {
    pub fn new(filter: FilterStep) -> Self {
        Self { filter }
    }

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
}

impl StepExecutor for FilterExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        let registry = global_registry();
        let mut current = input.clone();

        match &self.filter {
            FilterStep::Pipeline(pipeline) => {
                let filters = Self::parse_pipeline(pipeline);
                for (name, args) in filters {
                    current = registry.apply(&name, &current, &args)?;
                }
            }
            FilterStep::List(filters) => {
                for filter_config in filters {
                    let args = filter_config.args.as_deref().unwrap_or(&[]);
                    current = registry.apply(&filter_config.name, &current, args)?;
                }
            }
        }

        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pipeline() {
        let filters = FilterExecutor::parse_pipeline("trim | lower | replace(a, b)");
        assert_eq!(filters.len(), 3);
        assert_eq!(filters[0].0, "trim");
        assert_eq!(filters[1].0, "lower");
        assert_eq!(filters[2].0, "replace");
        assert_eq!(filters[2].1.len(), 2);
    }
}
