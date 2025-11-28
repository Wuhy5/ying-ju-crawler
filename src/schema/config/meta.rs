//! 元数据 (Meta)

use crate::schema::types::MediaType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 元数据 (Meta)
/// 描述规则的基本信息。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// 规则名称，如“示例影视站”。
    pub name: String,
    /// 规则作者。
    pub author: String,
    /// 规则版本号，建议遵循 SemVer (如 "1.0.2")。
    pub version: String,
    /// 本规则遵循的规范版本号。
    pub spec_version: String,
    /// 目标网站的主域名。
    pub domain: String,
    /// 规则主要适用的媒体类型。
    pub media_type: MediaType,
    /// 规则的详细描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 目标网站的编码，默认为 "UTF-8"。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// 数据源的图标URL，用于UI展示。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}
