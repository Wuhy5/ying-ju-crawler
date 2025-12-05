//! # 视频数据模型
//!
//! 定义视频详情和播放内容的数据结构

use super::PlayLine;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 视频详情
///
/// 对应 schema 中的 VideoDetailFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDetail {
    /// 片名
    pub title: String,
    /// 封面/海报
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 简介/剧情介绍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 导演
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<String>,
    /// 演员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<String>,
    /// 分类/类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 地区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 年份
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    /// 评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 更新信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_info: Option<String>,
    /// 时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// 播放线路列表
    #[serde(default)]
    pub play_lines: Vec<PlayLine>,
    /// 原始数据
    #[serde(default)]
    pub raw: Value,
}

impl VideoDetail {
    /// 创建新的视频详情（仅必需字段）
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            cover: None,
            intro: None,
            director: None,
            actors: None,
            category: None,
            tags: None,
            region: None,
            year: None,
            score: None,
            language: None,
            update_info: None,
            duration: None,
            play_lines: Vec::new(),
            raw: Value::Null,
        }
    }
}

/// 视频播放信息
///
/// 对应 schema 中的 VideoPlayFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPlay {
    /// 播放地址
    pub play_url: String,
    /// 视频标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 画质信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}

impl VideoPlay {
    /// 创建新的视频播放信息
    pub fn new(play_url: impl Into<String>) -> Self {
        Self {
            play_url: play_url.into(),
            title: None,
            quality: None,
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置画质
    pub fn with_quality(mut self, quality: impl Into<String>) -> Self {
        self.quality = Some(quality.into());
        self
    }
}
