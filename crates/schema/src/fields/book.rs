//! 书籍字段规则
//!
//! 定义书籍详情页和正文页的字段提取规则

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    common::{FieldRule, OptionalFieldRule},
    list_rules::ChapterListRule,
};

/// 书籍详情字段规则 (BookDetailFields)
/// 定义书籍详情页需要提取的所有字段
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BookDetailFields {
    /// 书名（必需）
    pub title: FieldRule,

    /// 作者（必需）
    pub author: FieldRule,

    /// 封面图 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: OptionalFieldRule,

    /// 简介/描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: OptionalFieldRule,

    /// 分类/类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: OptionalFieldRule,

    /// 标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: OptionalFieldRule,

    /// 连载状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: OptionalFieldRule,

    /// 最新章节名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_chapter: OptionalFieldRule,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: OptionalFieldRule,

    /// 字数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: OptionalFieldRule,

    /// 目录页 URL（如果目录在单独页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc_url: OptionalFieldRule,

    /// 章节列表提取规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<ChapterListRule>,
}

/// 书籍内容字段规则 (BookContentFields)
/// 定义书籍正文页需要提取的字段
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BookContentFields {
    /// 正文内容（必需）
    pub content: FieldRule,

    /// 章节标题（可选，可能需要从正文页重新获取）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: OptionalFieldRule,

    /// 下一页 URL（用于分页章节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_url: OptionalFieldRule,
}
