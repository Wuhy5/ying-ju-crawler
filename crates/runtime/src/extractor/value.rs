//! # 提取值类型
//!
//! 中间值表示，使用 Arc 实现零拷贝处理

use serde::{Deserialize, Serialize, ser::SerializeSeq};
use serde_json::Value;
use std::sync::Arc;

/// 共享的提取值（使用 Arc 实现廉价克隆）
pub type SharedValue = Arc<ExtractValueData>;

/// 提取过程中的中间值表示
///
/// 所有变体都使用 Arc 包装，使克隆成本从 O(n) 降低到 O(1)
#[derive(Debug, Clone, Default)]
pub enum ExtractValueData {
    /// 字符串（使用 Arc<str> 零拷贝）
    String(Arc<str>),
    /// JSON 值（使用 Arc 廉价克隆）
    Json(Arc<Value>),
    /// HTML 字符串（使用 Arc<str> 零拷贝）
    Html(Arc<str>),
    /// 数组（包含共享值）
    Array(Arc<Vec<SharedValue>>),
    /// 空值
    #[default]
    Null,
}

impl ExtractValueData {
    /// 获取字符串引用（零拷贝）
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            Self::Html(h) => Some(h),
            _ => None,
        }
    }

    /// 转换为 JSON 引用（零拷贝）
    pub fn as_json_ref(&self) -> Option<&Value> {
        match self {
            Self::Json(v) => Some(v),
            _ => None,
        }
    }

    /// 获取数组切片（零拷贝）
    pub fn as_array_slice(&self) -> Option<&[SharedValue]> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// 转换为拥有所有权的 JSON（仅在导出时需要）
    pub fn to_owned_json(&self) -> Value {
        match self {
            Self::String(s) => Value::String(s.to_string()),
            Self::Json(v) => (**v).clone(),
            Self::Html(h) => Value::String(h.to_string()),
            Self::Array(arr) => Value::Array(arr.iter().map(|v| v.to_owned_json()).collect()),
            Self::Null => Value::Null,
        }
    }

    /// 从 JSON 值创建（用于默认值）
    pub fn from_json(value: &Value) -> Self {
        match value {
            Value::String(s) => Self::String(Arc::from(s.clone().into_boxed_str())),
            Value::Array(arr) => {
                let items: Vec<SharedValue> =
                    arr.iter().map(|v| Arc::new(Self::from_json(v))).collect();
                Self::Array(Arc::new(items))
            }
            other => Self::Json(Arc::new(other.clone())),
        }
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        match self {
            Self::String(s) => s.is_empty(),
            Self::Json(v) => v.is_null(),
            Self::Html(h) => h.is_empty(),
            Self::Array(arr) => arr.is_empty(),
            Self::Null => true,
        }
    }

    /// 是否为数组
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// 是否为真值
    ///
    /// 用于条件判断，以下情况返回 false：
    /// - Null
    /// - 空字符串
    /// - 空数组
    /// - JSON 的 false、null、空字符串、空数组
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::String(s) => !s.is_empty(),
            Self::Html(h) => !h.is_empty(),
            Self::Array(arr) => !arr.is_empty(),
            Self::Json(v) => match v.as_ref() {
                Value::Null => false,
                Value::Bool(b) => *b,
                Value::String(s) => !s.is_empty(),
                Value::Array(arr) => !arr.is_empty(),
                Value::Number(_) => true,
                Value::Object(_) => true,
            },
        }
    }
}

impl From<String> for ExtractValueData {
    fn from(s: String) -> Self {
        Self::String(Arc::from(s.into_boxed_str()))
    }
}

impl From<&str> for ExtractValueData {
    fn from(s: &str) -> Self {
        Self::String(Arc::from(s.to_string().into_boxed_str()))
    }
}

impl From<Value> for ExtractValueData {
    fn from(v: Value) -> Self {
        Self::Json(Arc::new(v))
    }
}

// 自定义 Serde 实现以支持序列化
impl Serialize for ExtractValueData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::String(s) => serializer.serialize_str(s),
            Self::Json(v) => v.serialize(serializer),
            Self::Html(h) => serializer.serialize_str(h),
            Self::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr.iter() {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Self::Null => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for ExtractValueData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self::from_json(&value))
    }
}
