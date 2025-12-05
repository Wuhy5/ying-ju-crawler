//! # CSS 选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::value::{ExtractValueData, SharedValue},
};
use crawler_schema::extract::SelectorStep;
use scraper::{Html, Selector};
use std::sync::Arc;

/// CSS 选择器执行器
pub struct CssSelectorExecutor;

impl CssSelectorExecutor {
    /// 执行 CSS 选择器
    pub fn execute(
        selector: &SelectorStep,
        input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        // 获取 HTML 字符串
        let html = match input {
            ExtractValueData::String(s) | ExtractValueData::Html(s) => s.as_ref(),
            ExtractValueData::Array(arr) => {
                // 如果是数组，对每个元素应用选择器
                let results: Vec<SharedValue> = arr
                    .iter()
                    .filter_map(|item| match item.as_ref() {
                        ExtractValueData::Html(h) | ExtractValueData::String(h) => {
                            Self::execute_on_html(h, selector).ok()
                        }
                        _ => None,
                    })
                    .flatten()
                    .collect();
                return Ok(Arc::new(ExtractValueData::Array(Arc::new(results))));
            }
            _ => {
                return Err(RuntimeError::Extraction(
                    "CSS selector requires HTML input".to_string(),
                ));
            }
        };

        let results = Self::execute_on_html(html, selector)?;
        if results.is_empty() {
            Ok(Arc::new(ExtractValueData::Null))
        } else if results.len() == 1 && !Self::is_select_all(selector) {
            Ok(results.into_iter().next().unwrap())
        } else {
            Ok(Arc::new(ExtractValueData::Array(Arc::new(results))))
        }
    }

    /// 在 HTML 上执行选择器
    fn execute_on_html(html: &str, selector: &SelectorStep) -> Result<Vec<SharedValue>> {
        let document = Html::parse_fragment(html);

        let (selector_str, select_all) = match selector {
            SelectorStep::Simple(s) => (s.as_str(), false),
            SelectorStep::WithOptions { expr, all } => (expr.as_str(), *all),
        };

        let css_selector = Selector::parse(selector_str).map_err(|e| {
            RuntimeError::Extraction(format!("Invalid CSS selector '{}': {:?}", selector_str, e))
        })?;

        let elements = document.select(&css_selector);

        let results: Vec<SharedValue> = if select_all {
            elements
                .map(|el| {
                    Arc::new(ExtractValueData::Html(Arc::from(
                        el.html().into_boxed_str(),
                    )))
                })
                .collect()
        } else {
            // 只取第一个匹配
            elements
                .take(1)
                .map(|el| {
                    Arc::new(ExtractValueData::Html(Arc::from(
                        el.html().into_boxed_str(),
                    )))
                })
                .collect()
        };

        Ok(results)
    }

    /// 是否选择所有匹配
    fn is_select_all(selector: &SelectorStep) -> bool {
        match selector {
            SelectorStep::Simple(_) => false,
            SelectorStep::WithOptions { all, .. } => *all,
        }
    }
}
