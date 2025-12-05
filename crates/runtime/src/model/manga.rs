//! # 漫画数据模型
//!
//! 定义漫画详情和章节内容的数据结构

use super::{EpisodeItem, PlayLine};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 漫画详情
///
/// 对应 schema 中的 MangaDetailFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaDetail {
    /// 标题
    pub title: String,
    /// 章节线路列表
    pub play_lines: Vec<PlayLine>,
    /// 封面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// 简介
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 原始数据
    #[serde(default)]
    pub raw: Value,
}

impl MangaDetail {
    /// 创建新的漫画详情（仅必需字段）
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            play_lines: Vec::new(),
            cover: None,
            author: None,
            intro: None,
            category: None,
            status: None,
            tags: None,
            update_time: None,
            score: None,
            raw: Value::Null,
        }
    }

    /// 设置章节线路
    pub fn with_play_lines(mut self, lines: Vec<PlayLine>) -> Self {
        self.play_lines = lines;
        self
    }

    /// 添加单个线路
    pub fn add_play_line(&mut self, name: impl Into<String>, episodes: Vec<EpisodeItem>) {
        self.play_lines.push(PlayLine {
            name: name.into(),
            episodes,
        });
    }
}

/// 漫画章节内容
///
/// 对应 schema 中的 MangaContentFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaContent {
    /// 图片列表
    pub images: Vec<String>,
    /// 章节标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 下一章 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<String>,
    /// 上一章 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_url: Option<String>,
}

impl MangaContent {
    /// 创建新的漫画章节内容
    pub fn new(images: Vec<String>) -> Self {
        Self {
            images,
            title: None,
            next_url: None,
            prev_url: None,
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 添加图片
    pub fn add_image(&mut self, url: impl Into<String>) {
        self.images.push(url.into());
    }
}
