//! Schema定义模块
//! 包含爬虫规则的所有数据结构定义

pub mod core;
pub mod config {
    pub mod http;
    pub mod limits;
    pub mod meta;
    pub mod scripting;
    pub mod traits;

    pub use http::*;
    pub use limits::*;
    pub use meta::*;
    pub use scripting::*;
    pub use traits::*;
}
pub mod flow;
pub mod pipeline;
pub mod render;
pub mod template;
pub mod types;

// 重新导出常用类型
pub use core::*;
pub use flow::*;
pub use pipeline::{Pipeline, PipelineExt, Step, StepCategory, StepTrait};
pub use render::*;
pub use template::Template;
pub use types::*;
