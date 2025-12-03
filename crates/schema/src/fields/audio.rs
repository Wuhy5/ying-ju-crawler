//! 音频字段规则
//!
//! 定义音频详情页和播放页的字段提取规则

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    common::{FieldRule, OptionalFieldRule},
    list_rules::TrackListRule,
};

/// 音频详情字段规则 (AudioDetailFields)
/// 定义音频详情页需要提取的所有字段
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioDetailFields {
    /// 标题（必需）
    pub title: FieldRule,

    /// 艺术家/作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: OptionalFieldRule,

    /// 封面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: OptionalFieldRule,

    /// 简介/描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: OptionalFieldRule,

    /// 专辑名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: OptionalFieldRule,

    /// 分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: OptionalFieldRule,

    /// 标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: OptionalFieldRule,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: OptionalFieldRule,

    /// 播放量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: OptionalFieldRule,

    /// 音轨列表提取规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<TrackListRule>,
}

/// 音频播放字段规则 (AudioPlayFields)
/// 定义音频播放页需要提取的字段（解析真实播放地址）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioPlayFields {
    /// 播放地址（必需）
    pub play_url: FieldRule,

    /// 音频标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: OptionalFieldRule,

    /// 艺术家（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: OptionalFieldRule,

    /// 封面图（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: OptionalFieldRule,

    /// 歌词/文本内容（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: OptionalFieldRule,

    /// 时长（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: OptionalFieldRule,
}
