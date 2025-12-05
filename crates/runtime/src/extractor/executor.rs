//! # 步骤执行器
//!
//! 使用无状态静态方法实现各种提取步骤

use crate::{
    Result,
    context::Context,
    extractor::value::{ExtractValueData, SharedValue},
};
use crawler_schema::extract::ExtractStep;

/// 步骤执行器工厂
///
/// 所有执行器都使用静态方法，无需创建实例
pub struct StepExecutorFactory;

impl StepExecutorFactory {
    /// 直接执行步骤
    pub fn execute(
        step: &ExtractStep,
        input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        match step {
            ExtractStep::Css(selector) => {
                crate::extractor::selector::css::CssSelectorExecutor::execute(
                    selector, input, context,
                )
            }
            ExtractStep::Json(selector) => {
                crate::extractor::selector::json::JsonSelectorExecutor::execute(
                    selector, input, context,
                )
            }
            ExtractStep::Regex(regex) => {
                crate::extractor::selector::regex::RegexSelectorExecutor::execute(
                    regex, input, context,
                )
            }
            ExtractStep::Filter(filter) => {
                crate::extractor::filter::executor::FilterExecutor::execute(filter, input, context)
            }
            ExtractStep::Attr(attr) => {
                crate::extractor::selector::attr::AttrExecutor::execute(attr, input, context)
            }
            ExtractStep::Index(index) => {
                crate::extractor::selector::index::IndexExecutor::execute(index, input, context)
            }
            ExtractStep::Const(value) => {
                crate::extractor::selector::const_value::ConstExecutor::execute(
                    value, input, context,
                )
            }
            ExtractStep::Var(var) => {
                crate::extractor::selector::var::VarExecutor::execute(var, input, context)
            }
            ExtractStep::Script(script) => {
                crate::script::ScriptExecutor::execute(script, input, context)
            }
            ExtractStep::UseComponent(component_ref) => {
                crate::extractor::selector::component::ComponentExecutor::execute(
                    component_ref,
                    input,
                    context,
                )
            }
            ExtractStep::Xpath(_selector) => {
                // XPath 需要 JS 环境，暂不支持
                Err(crate::error::RuntimeError::Extraction(
                    "XPath not supported in this context".into(),
                ))
            }
            ExtractStep::Map(steps) => {
                crate::extractor::selector::map::MapExecutor::execute(steps, input, context)
            }
            ExtractStep::Condition(condition) => {
                crate::extractor::selector::condition::ConditionExecutor::execute(
                    condition, input, context,
                )
            }
        }
    }
}
