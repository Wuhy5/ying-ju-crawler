//! # 流程执行器 trait
//!
//! 定义所有流程执行器的通用接口

use crate::{Result, context::Context};
use async_trait::async_trait;

/// 流程执行器 trait
///
/// 所有流程执行器都需要实现此 trait
#[async_trait]
pub trait FlowExecutor: Send + Sync {
    /// 输入类型
    type Input: Send;
    /// 输出类型
    type Output: Send;

    /// 执行流程
    async fn execute(&self, input: Self::Input, context: &mut Context) -> Result<Self::Output>;
}
