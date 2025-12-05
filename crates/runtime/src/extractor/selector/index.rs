//! # 索引/切片执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::value::{ExtractValueData, SharedValue},
};
use crawler_schema::extract::IndexStep;
use std::sync::Arc;

/// 索引执行器
pub struct IndexExecutor;

impl IndexExecutor {
    /// 执行索引/切片
    pub fn execute(
        index: &IndexStep,
        input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        match input {
            ExtractValueData::Array(arr) => match index {
                IndexStep::Single(idx) => {
                    let index_pos = if *idx < 0 {
                        (arr.len() as i32 + idx) as usize
                    } else {
                        *idx as usize
                    };

                    if index_pos < arr.len() {
                        Ok(arr[index_pos].clone())
                    } else {
                        Err(RuntimeError::Extraction("Index out of bounds".to_string()))
                    }
                }
                IndexStep::Slice(slice_str) => {
                    // 解析切片：start:end 或 start:end:step
                    let parts: Vec<&str> = slice_str.split(':').collect();
                    let start = parts
                        .first()
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(0);
                    let end = parts
                        .get(1)
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(arr.len());

                    let sliced: Vec<SharedValue> = arr
                        .iter()
                        .skip(start)
                        .take(end.saturating_sub(start))
                        .cloned()
                        .collect();
                    Ok(Arc::new(ExtractValueData::Array(Arc::new(sliced))))
                }
            },
            _ => Err(RuntimeError::Extraction(
                "Index operation requires array input".to_string(),
            )),
        }
    }
}
