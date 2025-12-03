//! 核心结构体与顶级规则文件结构

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    config::{ChallengeConfig, HttpConfig, Meta},
    flow::{ContentFlow, DetailFlow, DiscoveryFlow, LoginFlow, SearchFlow},
};

/// 影视软件爬虫规则 (CrawlerRule)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CrawlerRule {
    /// 规则的元数据，用于在软件中识别和展示。
    pub meta: Meta,
    /// 全局的网络请求配置，可被流程局部配置覆盖。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,
    /// 人机验证/反爬挑战处理配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<ChallengeConfig>,

    // ===== 流程定义 =====
    /// 登录流程（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<LoginFlow>,
    /// 发现页流程（可选）
    /// 提供筛选器和分页配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery: Option<DiscoveryFlow>,
    /// 详情页流程（必需）
    pub detail: DetailFlow,
    /// 搜索流程（必需）
    pub search: SearchFlow,
    /// 内容页流程（可选）
    /// 用于播放页、阅读页等需要进一步解析内容的场景
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentFlow>,
}
