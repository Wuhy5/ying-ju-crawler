//! 可重用组件 (Component)

use crate::extract::FieldExtractor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 可重用组件 (Component)
/// 封装可复用的字段提取逻辑
///
/// # 示例
///
/// ```toml
/// [components.parse_video_url]
/// description = "解析加密的视频地址"
/// inputs = { encrypted_url = "" }
///
/// [components.parse_video_url.extractor]
/// steps = [{ script = "decrypt.parse_m3u8" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Component {
    /// 组件的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 定义组件接收的输入参数 (key: 参数名, value: 默认值)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,

    /// 组件的提取逻辑
    pub extractor: FieldExtractor,
}
