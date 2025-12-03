//! # 详情流程执行器

use crate::{Result, context::Context, flow::FlowExecutor};
use async_trait::async_trait;
use crawler_schema::DetailFlow;

/// 详情请求
#[derive(Debug, Clone)]
pub struct DetailRequest {
    /// 详情页 URL
    pub url: String,
}

/// 详情响应
#[derive(Debug, Clone)]
pub struct DetailResponse {
    /// 详情数据
    pub data: serde_json::Value,
}

/// 详情流程执行器
pub struct DetailFlowExecutor {
    flow: DetailFlow,
}

impl DetailFlowExecutor {
    pub fn new(flow: DetailFlow) -> Self {
        Self { flow }
    }
}

#[async_trait]
impl FlowExecutor for DetailFlowExecutor {
    type Input = DetailRequest;
    type Output = DetailResponse;

    async fn execute(&self, input: Self::Input, context: &mut Context) -> Result<Self::Output> {
        // 设置上下文变量
        context.set("detail_url", serde_json::json!(input.url));

        // TODO: 实现详情流程
        // 1. 渲染 URL
        // 2. 发起 HTTP 请求
        // 3. 提取字段数据
        let _ = &self.flow;

        Ok(DetailResponse {
            data: serde_json::json!({}),
        })
    }
}
