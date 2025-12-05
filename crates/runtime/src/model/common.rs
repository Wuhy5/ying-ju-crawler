//! # 通用数据模型
//!
//! 定义搜索结果、列表项等通用模型

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 搜索结果项
///
/// 表示搜索/发现列表中的单个项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItem {
    /// 标题
    pub title: String,
    /// 详情页 URL
    pub url: String,
    /// 封面图 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 简介/摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 作者/创作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// 最新章节/更新信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    /// 评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 原始数据
    #[serde(default)]
    pub raw: Value,
}

impl SearchItem {
    /// 创建新的搜索项（仅必需字段）
    pub fn new(title: String, url: String) -> Self {
        Self {
            title,
            url,
            cover: None,
            summary: None,
            author: None,
            latest: None,
            score: None,
            status: None,
            category: None,
            raw: Value::Null,
        }
    }

    /// 设置封面
    pub fn with_cover(mut self, cover: impl Into<String>) -> Self {
        self.cover = Some(cover.into());
        self
    }

    /// 设置作者
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// 设置简介
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }
}

/// 章节项
///
/// 表示书籍/漫画的章节
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterItem {
    /// 章节标题
    pub title: String,
    /// 章节 URL
    pub url: String,
}

impl ChapterItem {
    /// 创建新的章节项
    pub fn new(title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            url: url.into(),
        }
    }
}

/// 剧集项
///
/// 表示视频的剧集/集数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeItem {
    /// 剧集名称
    pub name: String,
    /// 播放页 URL
    pub url: String,
}

impl EpisodeItem {
    /// 创建新的剧集项
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }
}

/// 播放线路
///
/// 表示视频的播放线路（包含多个剧集）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayLine {
    /// 线路名称
    pub name: String,
    /// 剧集列表
    pub episodes: Vec<EpisodeItem>,
}

impl PlayLine {
    /// 创建新的播放线路
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            episodes: Vec::new(),
        }
    }

    /// 添加剧集
    pub fn add_episode(&mut self, episode: EpisodeItem) {
        self.episodes.push(episode);
    }
}

/// 音轨项
///
/// 表示音频专辑的单个音轨
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackItem {
    /// 音轨名称
    pub name: String,
    /// 音轨 URL
    pub url: String,
    /// 时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl TrackItem {
    /// 创建新的音轨项
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            duration: None,
        }
    }

    /// 设置时长
    pub fn with_duration(mut self, duration: impl Into<String>) -> Self {
        self.duration = Some(duration.into());
        self
    }
}
