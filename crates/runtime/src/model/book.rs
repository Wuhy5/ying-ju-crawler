//! # 书籍数据模型
//!
//! 定义书籍详情和正文内容的数据结构

use super::ChapterItem;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 书籍详情
///
/// 对应 schema 中的 BookDetailFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookDetail {
    /// 书名
    pub title: String,
    /// 作者
    pub author: String,
    /// 封面 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 简介
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 连载状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 最新章节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_chapter: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 字数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<String>,
    /// 目录页 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc_url: Option<String>,
    /// 章节列表
    #[serde(default)]
    pub chapters: Vec<ChapterItem>,
    /// 原始数据
    #[serde(default)]
    pub raw: Value,
}

impl BookDetail {
    /// 创建新的书籍详情（仅必需字段）
    pub fn new(title: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            cover: None,
            intro: None,
            category: None,
            tags: None,
            status: None,
            last_chapter: None,
            update_time: None,
            word_count: None,
            toc_url: None,
            chapters: Vec::new(),
            raw: Value::Null,
        }
    }
}

/// 书籍正文内容
///
/// 对应 schema 中的 BookContentFields 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookContent {
    /// 正文内容
    pub content: String,
    /// 章节标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 上一页 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_url: Option<String>,
    /// 下一页 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<String>,
}

impl BookContent {
    /// 创建新的书籍正文
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            title: None,
            prev_url: None,
            next_url: None,
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置分页链接
    pub fn with_pagination(mut self, prev: Option<String>, next: Option<String>) -> Self {
        self.prev_url = prev;
        self.next_url = next;
        self
    }
}
