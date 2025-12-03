//! # 步骤执行器
//!
//! 使用策略模式实现各种提取步骤

use crate::{Result, context::Context, extractor::value::ExtractValue};
use crawler_schema::ExtractStep;

/// 步骤执行器 trait（策略模式）
pub trait StepExecutor: Send + Sync {
    /// 执行步骤
    fn execute(&self, input: &ExtractValue, context: &Context) -> Result<ExtractValue>;
}

/// 步骤执行器工厂（工厂模式）
pub struct StepExecutorFactory;

impl StepExecutorFactory {
    /// 根据步骤类型创建执行器
    pub fn create(step: &ExtractStep) -> Box<dyn StepExecutor> {
        match step {
            ExtractStep::Css(selector) => Box::new(
                crate::extractor::selector::css::CssSelectorExecutor::new(selector.clone()),
            ),
            ExtractStep::Json(selector) => Box::new(
                crate::extractor::selector::json::JsonSelectorExecutor::new(selector.clone()),
            ),
            ExtractStep::Regex(regex) => Box::new(
                crate::extractor::selector::regex::RegexSelectorExecutor::new(regex.clone()),
            ),
            ExtractStep::Filter(filter) => Box::new(
                crate::extractor::filter::executor::FilterExecutor::new(filter.clone()),
            ),
            ExtractStep::Attr(attr) => Box::new(
                crate::extractor::selector::attr::AttrExecutor::new(attr.clone()),
            ),
            ExtractStep::Index(index) => Box::new(
                crate::extractor::selector::index::IndexExecutor::new(index.clone()),
            ),
            ExtractStep::Const(value) => Box::new(
                crate::extractor::selector::const_value::ConstExecutor::new(value.clone()),
            ),
            ExtractStep::Var(var) => Box::new(crate::extractor::selector::var::VarExecutor::new(
                var.clone(),
            )),
            ExtractStep::Script(_script) => {
                // TODO: 实现脚本执行器
                Box::new(crate::extractor::selector::noop::NoopExecutor)
            }
        }
    }
}
