//! 流程与组件 (Flow & Component)
//!
//! 定义不同类型的流程：
//! - LoginFlow: 登录流程
//! - ListFlow: 列表页流程
//! - DetailFlow: 详情页流程
//! - SearchFlow: 搜索流程

use crate::schema::{
    pipeline::Pipeline,
    types::{FilterGroup, Identifier},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Flow Trait - 统一的流程接口
// ============================================================================

/// 流程特性 (FlowTrait)
/// 所有流程类型的统一接口
pub trait FlowTrait {
    /// 获取流程描述
    fn description(&self) -> Option<&str>;

    /// 获取流程管道
    fn pipeline(&self) -> &Pipeline;

    /// 获取可变管道引用
    fn pipeline_mut(&mut self) -> &mut Pipeline;

    /// 流程类型名称
    fn flow_type(&self) -> &'static str;
}

// ============================================================================
// Component - 可重用组件
// ============================================================================

/// 可重用组件 (Component)
/// 一个可被其他管道调用的、封装了特定逻辑的子管道。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Component {
    /// 组件的功能描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 定义组件接收的输入参数 (key: 参数名, value: 默认值)。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(with = "HashMap<Identifier, serde_json::Value>")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,
    /// 组件的核心处理管道。
    pub pipeline: Pipeline,
}

// ============================================================================
// LoginFlow - 登录流程
// ============================================================================

/// 登录流程 (LoginFlow)
/// 处理网站的身份验证，支持多种认证方式
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 登录类型
    #[serde(default)]
    pub login_type: LoginType,

    /// 登录所需的凭证字段定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<CredentialField>>,

    /// 登录验证：检查是否已登录的管道
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_login: Option<Pipeline>,

    /// 登录动作管道
    pub pipeline: Pipeline,
}

impl FlowTrait for LoginFlow {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    fn flow_type(&self) -> &'static str {
        "login"
    }
}

/// 登录类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LoginType {
    /// 用户名密码登录
    #[default]
    Password,
    /// Cookie登录
    Cookie,
    /// Token登录
    Token,
    /// OAuth登录
    OAuth,
    /// 验证码登录
    Captcha,
}

/// 凭证字段定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CredentialField {
    /// 字段标识符
    #[schemars(with = "Identifier")]
    pub key: String,
    /// 字段显示名称
    pub label: String,
    /// 字段类型
    #[serde(default)]
    pub field_type: CredentialFieldType,
    /// 是否必填
    #[serde(default = "default_true")]
    pub required: bool,
    /// 占位符文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

fn default_true() -> bool {
    true
}

/// 凭证字段类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialFieldType {
    /// 普通文本
    #[default]
    Text,
    /// 密码（隐藏显示）
    Password,
    /// 邮箱
    Email,
    /// 多行文本（如Cookie）
    Textarea,
}

// ============================================================================
// ListFlow - 列表页流程
// ============================================================================

/// 列表页流程 (ListFlow)
/// 用于发现和浏览内容列表
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ListFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 分页配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationConfig>,

    /// 筛选器配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<FilterGroup>>,

    /// 列表页处理管道
    pub pipeline: Pipeline,
}

impl FlowTrait for ListFlow {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    fn flow_type(&self) -> &'static str {
        "list"
    }
}

// ============================================================================
// DetailFlow - 详情页流程
// ============================================================================

/// 详情页流程 (DetailFlow)
/// 处理单个内容项的详细信息
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DetailFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 详情页处理管道
    pub pipeline: Pipeline,
}

impl FlowTrait for DetailFlow {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    fn flow_type(&self) -> &'static str {
        "detail"
    }
}

// ============================================================================
// SearchFlow - 搜索流程
// ============================================================================

/// 搜索流程 (SearchFlow)
/// 处理搜索功能
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 分页配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationConfig>,

    /// 搜索筛选器
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<FilterGroup>>,

    /// 搜索处理管道
    pub pipeline: Pipeline,
}

impl FlowTrait for SearchFlow {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    fn flow_type(&self) -> &'static str {
        "search"
    }
}

// ============================================================================
// 共享配置类型
// ============================================================================

/// 分页配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PaginationConfig {
    /// 分页类型
    #[serde(default)]
    pub pagination_type: PaginationType,

    /// 起始页码
    #[serde(default = "default_start_page")]
    pub start_page: u32,

    /// 每页数量（如果适用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,

    /// 最大页数限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pages: Option<u32>,

    /// 页码参数名
    #[serde(default = "default_page_param")]
    #[schemars(with = "Identifier")]
    pub page_param: String,
}

fn default_start_page() -> u32 {
    1
}

fn default_page_param() -> String {
    "page".to_string()
}

/// 分页类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaginationType {
    /// 页码分页
    #[default]
    PageNumber,
    /// 偏移量分页
    Offset,
    /// 游标分页
    Cursor,
    /// 加载更多（无限滚动）
    LoadMore,
}
