//! # 音频数据模型
//!
//! 定义音频详情和播放内容的数据结构

use super::TrackItem;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 音频详情
///
/// 对应 schema 中的 AudioDetailFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDetail {
    /// 标题
    pub title: String,
    /// 艺术家/作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    /// 封面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 简介
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 专辑名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    /// 分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 播放量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<String>,
    /// 音轨列表
    #[serde(default)]
    pub tracks: Vec<TrackItem>,
    /// 原始数据
    #[serde(default)]
    pub raw: Value,
}

impl AudioDetail {
    /// 创建新的音频详情（仅必需字段）
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            artist: None,
            cover: None,
            intro: None,
            album: None,
            category: None,
            tags: None,
            update_time: None,
            play_count: None,
            tracks: Vec::new(),
            raw: Value::Null,
        }
    }
}

/// 音频播放信息
///
/// 对应 schema 中的 AudioPlayFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioPlay {
    /// 播放地址
    pub play_url: String,
    /// 音频标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 艺术家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    /// 封面图
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 歌词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<String>,
    /// 时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl AudioPlay {
    /// 创建新的音频播放信息
    pub fn new(play_url: impl Into<String>) -> Self {
        Self {
            play_url: play_url.into(),
            title: None,
            artist: None,
            cover: None,
            lyrics: None,
            duration: None,
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置艺术家
    pub fn with_artist(mut self, artist: impl Into<String>) -> Self {
        self.artist = Some(artist.into());
        self
    }
}
