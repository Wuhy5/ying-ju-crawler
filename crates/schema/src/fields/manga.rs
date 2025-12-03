//! 漫画字段规则
//!
//! 定义漫画详情页和阅读页的字段提取规则

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    common::{FieldRule, OptionalFieldRule},
    list_rules::ChapterListRule,
};

/// 漫画详情字段规则 (MangaDetailFields)
/// 定义漫画详情页需要提取的所有字段
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MangaDetailFields {
    /// 漫画名（必需）
    pub title: FieldRule,

    /// 作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: OptionalFieldRule,

    /// 封面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: OptionalFieldRule,

    /// 简介/描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: OptionalFieldRule,

    /// 分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: OptionalFieldRule,

    /// 标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: OptionalFieldRule,

    /// 连载状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: OptionalFieldRule,

    /// 最新章节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_chapter: OptionalFieldRule,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: OptionalFieldRule,

    /// 章节列表提取规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<ChapterListRule>,
}

/// 漫画阅读字段规则 (MangaReadFields)
/// 定义漫画阅读页需要提取的字段
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MangaReadFields {
    /// 图片列表（必需）
    pub images: FieldRule,

    /// 章节标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: OptionalFieldRule,

    /// 下一章 URL（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_chapter_url: OptionalFieldRule,

    /// 上一章 URL（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_chapter_url: OptionalFieldRule,
}
