//! 视频字段规则
//!
//! 定义视频详情页和播放页的字段提取规则

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    common::{FieldRule, OptionalFieldRule},
    list_rules::PlayLineListRule,
};

/// 视频详情字段规则 (VideoDetailFields)
/// 定义视频详情页需要提取的所有字段
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VideoDetailFields {
    /// 片名（必需）
    pub title: FieldRule,

    /// 封面/海报
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: OptionalFieldRule,

    /// 简介/剧情介绍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: OptionalFieldRule,

    /// 导演
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: OptionalFieldRule,

    /// 演员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: OptionalFieldRule,

    /// 分类/类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: OptionalFieldRule,

    /// 标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: OptionalFieldRule,

    /// 地区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: OptionalFieldRule,

    /// 年份
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: OptionalFieldRule,

    /// 评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: OptionalFieldRule,

    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: OptionalFieldRule,

    /// 更新信息/集数状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_info: OptionalFieldRule,

    /// 时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: OptionalFieldRule,

    /// 播放线路列表提取规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_lines: Option<PlayLineListRule>,
}

/// 视频播放字段规则 (VideoPlayFields)
/// 定义视频播放页需要提取的字段（解析真实播放地址）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VideoPlayFields {
    /// 播放地址（必需）
    pub play_url: FieldRule,

    /// 视频标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: OptionalFieldRule,

    /// 画质信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: OptionalFieldRule,
}
