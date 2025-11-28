//! 核心结构体与顶级规则文件结构

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::schema::{
    config::{http::HttpConfig, limits::RuntimeLimits, meta::Meta, scripting::ScriptingConfig},
    flow::{Component, DetailFlow, ListFlow, LoginFlow, SearchFlow},
    types::Identifier,
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
    /// 运行时资源限制配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<RuntimeLimits>,
    /// 脚本引擎的全局配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripting: Option<ScriptingConfig>,
    /// 定义可在此规则中复用的"组件"或"函数"。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(with = "HashMap<Identifier, Component>")]
    pub components: Option<HashMap<String, Component>>,
    /// 登录流程（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<LoginFlow>,
    /// 列表页流程（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<ListFlow>,
    /// 详情页流程（必需）
    pub detail: DetailFlow,
    /// 搜索流程（必需）
    pub search: SearchFlow,
}

impl CrawlerRule {
    /// 获取运行时限制（使用配置值或默认值）
    pub fn limits_or_default(&self) -> RuntimeLimits {
        self.limits.clone().unwrap_or_default()
    }

    /// 获取HTTP配置（使用配置值或默认值）
    pub fn http_or_default(&self) -> HttpConfig {
        self.http.clone().unwrap_or_default()
    }
}
