//! 渲染层数据模型 (Rendering Data Models)
//!
//! 本模块定义了用于前端渲染的标准化数据结构，包括：
//! - ItemSummary: 列表页轻量级数据模型
//! - ItemDetail: 详情页完整数据模型
//! - MediaContent: 媒体特定内容结构（视频、音频、书籍、漫画）

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::MediaType;

/// 列表项摘要 (ItemSummary)
/// 用于列表页渲染的标准化数据结构，包含最少的必需字段。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ItemSummary {
    /// 唯一标识符，通常使用 canonical URL
    pub id: String,
    /// 标题
    pub title: String,
    /// 详情页 URL
    pub url: String,
    /// 媒体类型
    pub media_type: MediaType,
    /// 封面图 URL（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 简介或摘要（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 标签列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 元信息，如评分、年份、地区、时长等（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::HashMap<String, String>>,
}

/// 详情数据 (ItemDetail)
/// 用于详情页渲染的完整数据结构，包含元数据和媒体特定内容。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ItemDetail {
    /// 唯一标识符
    pub id: String,
    /// 标题
    pub title: String,
    /// 详情页 URL
    pub url: String,
    /// 媒体类型
    pub media_type: MediaType,
    /// 封面图 URL（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 详细描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 结构化元数据，如作者、导演、演员、年份、时长、语言、地区等（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// 标签列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 媒体特定内容，如播放线路、章节、音轨、图片等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<MediaContent>,
}

/// 媒体特定内容 (MediaContent)
/// 根据不同媒体类型存储播放/阅读内容的结构化数据。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MediaContent {
    /// 视频内容：播放线路列表
    Video {
        /// 播放线路列表
        play_lines: Vec<VideoPlayLine>,
    },
    /// 音频内容：音轨列表
    Audio {
        /// 音轨列表
        tracks: Vec<AudioTrack>,
    },
    /// 书籍内容：章节列表
    Book {
        /// 章节列表
        chapters: Vec<BookChapter>,
    },
    /// 漫画内容：章节/页面列表
    Manga {
        /// 漫画章节列表
        chapters: Vec<MangaChapter>,
    },
}

/// 视频播放线路 (VideoPlayLine)
/// 表示一个播放源及其集数列表。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VideoPlayLine {
    /// 线路名称，如 "线路1"、"高清"
    pub name: String,
    /// 集数列表
    pub episodes: Vec<VideoEpisode>,
}

/// 视频集数 (VideoEpisode)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VideoEpisode {
    /// 集数名称，如 "第1集"、"EP01"
    pub name: String,
    /// 播放 URL
    pub url: String,
    /// 画质标识（可选），如 "1080p"、"720p"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}

/// 音频音轨 (AudioTrack)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioTrack {
    /// 音轨名称
    pub name: String,
    /// 播放 URL
    pub url: String,
    /// 时长（可选），如 "03:45"、"225" (秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// 音质标识（可选），如 "320kbps"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}

/// 书籍章节 (BookChapter)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BookChapter {
    /// 章节标题
    pub title: String,
    /// 章节 URL
    pub url: String,
    /// 章节顺序（可选），用于排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

/// 漫画章节 (MangaChapter)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MangaChapter {
    /// 章节标题
    pub title: String,
    /// 图片 URL 列表
    pub images: Vec<String>,
    /// 章节顺序（可选），用于排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}
