//! # 属性提取器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::value::{ExtractValueData, SharedValue},
};
use scraper::Html;
use std::sync::Arc;

/// 属性提取器
///
/// 从 HTML 元素中提取属性或文本内容
/// 支持的属性名：
/// - `text` - 提取文本内容
/// - `html` - 提取内部 HTML
/// - `outer_html` - 提取外部 HTML（包含自身标签）
/// - 其他 - 提取指定属性值（如 href, src, class 等）
pub struct AttrExecutor;

impl AttrExecutor {
    /// 执行属性提取
    pub fn execute(
        attr_name: &str,
        input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        match input {
            ExtractValueData::Html(html) | ExtractValueData::String(html) => {
                Self::extract_from_html(html, attr_name)
            }
            ExtractValueData::Array(arr) => {
                // 对数组中的每个元素提取属性
                let results: Vec<SharedValue> = arr
                    .iter()
                    .filter_map(|item| match item.as_ref() {
                        ExtractValueData::Html(h) | ExtractValueData::String(h) => {
                            Self::extract_from_html(h, attr_name).ok()
                        }
                        _ => None,
                    })
                    .filter(|v| !v.is_empty())
                    .collect();

                if results.is_empty() {
                    Ok(Arc::new(ExtractValueData::Null))
                } else if results.len() == 1 {
                    Ok(results.into_iter().next().unwrap())
                } else {
                    Ok(Arc::new(ExtractValueData::Array(Arc::new(results))))
                }
            }
            _ => Err(RuntimeError::Extraction(
                "Attr executor requires HTML input".to_string(),
            )),
        }
    }

    fn extract_from_html(html: &str, attr_name: &str) -> Result<SharedValue> {
        let document = Html::parse_fragment(html);

        // 获取根元素（第一个非文本元素）
        let root = document
            .root_element()
            .first_child()
            .and_then(scraper::ElementRef::wrap);

        let result = match attr_name {
            "text" => {
                // 提取所有文本内容
                let text: String = document
                    .root_element()
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();
                if text.is_empty() {
                    ExtractValueData::Null
                } else {
                    ExtractValueData::String(Arc::from(text.into_boxed_str()))
                }
            }
            "html" | "inner_html" => {
                // 提取内部 HTML
                if let Some(el) = root {
                    ExtractValueData::String(Arc::from(el.inner_html().into_boxed_str()))
                } else {
                    ExtractValueData::Null
                }
            }
            "outer_html" => {
                // 提取外部 HTML
                if let Some(el) = root {
                    ExtractValueData::Html(Arc::from(el.html().into_boxed_str()))
                } else {
                    ExtractValueData::Null
                }
            }
            attr => {
                // 提取指定属性
                if let Some(el) = root {
                    match el.value().attr(attr) {
                        Some(value) => {
                            ExtractValueData::String(Arc::from(value.to_string().into_boxed_str()))
                        }
                        None => ExtractValueData::Null,
                    }
                } else {
                    ExtractValueData::Null
                }
            }
        };

        Ok(Arc::new(result))
    }
}
