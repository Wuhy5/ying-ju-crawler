//! # 提取值类型
//!
//! 中间值表示，避免频繁的类型转换

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 提取值
///
/// 表示提取过程中的中间值,支持多种数据类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtractValue {
    /// 字符串
    String(String),
    /// JSON 值
    Json(Value),
    /// HTML 字符串
    Html(String),
    /// 数组
    Array(Vec<ExtractValue>),
    /// 空值
    Null,
}

impl ExtractValue {
    /// 转换为字符串
    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            Self::Json(v) => v.as_str().map(|s| s.to_string()),
            Self::Html(h) => Some(h.clone()),
            Self::Array(arr) => {
                if arr.len() == 1 {
                    arr[0].as_string()
                } else {
                    None
                }
            }
            Self::Null => None,
        }
    }

    /// 转换为 JSON
    pub fn as_json(&self) -> Value {
        match self {
            Self::String(s) => Value::String(s.clone()),
            Self::Json(v) => v.clone(),
            Self::Html(h) => Value::String(h.clone()),
            Self::Array(arr) => Value::Array(arr.iter().map(|v| v.as_json()).collect()),
            Self::Null => Value::Null,
        }
    }

    /// 从 JSON 创建
    pub fn from_json(value: &Value) -> Self {
        match value {
            Value::String(s) => Self::String(s.clone()),
            Value::Array(arr) => Self::Array(arr.iter().map(|v| Self::from_json(v)).collect()),
            _ => Self::Json(value.clone()),
        }
    }

    /// 转换为数组
    pub fn as_array(&self) -> Option<&[ExtractValue]> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        match self {
            Self::String(s) => s.is_empty(),
            Self::Json(Value::Null) => true,
            Self::Json(Value::String(s)) => s.is_empty(),
            Self::Json(Value::Array(arr)) => arr.is_empty(),
            Self::Array(arr) => arr.is_empty(),
            Self::Null => true,
            _ => false,
        }
    }

    /// 是否为数组
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }
}

impl From<String> for ExtractValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for ExtractValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<Value> for ExtractValue {
    fn from(v: Value) -> Self {
        Self::Json(v)
    }
}

impl Default for ExtractValue {
    fn default() -> Self {
        Self::Null
    }
}
