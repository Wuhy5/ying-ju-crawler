//! # 索引/切片执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use crawler_schema::IndexStep;

/// 索引执行器
pub struct IndexExecutor {
    index: IndexStep,
}

impl IndexExecutor {
    pub fn new(index: IndexStep) -> Self {
        Self { index }
    }
}

impl StepExecutor for IndexExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        match input {
            ExtractValue::Array(arr) => match &self.index {
                IndexStep::Single(idx) => {
                    let index = if *idx < 0 {
                        (arr.len() as i32 + idx) as usize
                    } else {
                        *idx as usize
                    };

                    arr.get(index)
                        .cloned()
                        .ok_or_else(|| RuntimeError::Extraction("Index out of bounds".to_string()))
                }
                IndexStep::Slice(slice_str) => {
                    // 解析切片：start:end 或 start:end:step
                    let parts: Vec<&str> = slice_str.split(':').collect();
                    let start = parts
                        .get(0)
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(0);
                    let end = parts
                        .get(1)
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(arr.len());

                    let sliced: Vec<ExtractValue> = arr[start..end.min(arr.len())].to_vec();
                    Ok(ExtractValue::Array(sliced))
                }
            },
            _ => Err(RuntimeError::Extraction(
                "Index operation requires array input".to_string(),
            )),
        }
    }
}
