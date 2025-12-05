//! # 数据模型模块
//!
//! 定义所有提取结果的数据模型，按媒体类型组织
//!
//! ## 模块结构
//!
//! - `common`: 通用数据模型（搜索、列表项等）
//! - `book`: 书籍相关模型
//! - `video`: 视频相关模型
//! - `audio`: 音频相关模型
//! - `manga`: 漫画相关模型

mod audio;
mod book;
mod common;
mod manga;
mod video;

pub use audio::*;
pub use book::*;
pub use common::*;
pub use manga::*;
pub use video::*;
