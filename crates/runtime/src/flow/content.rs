//! # 内容流程执行器

use crate::{Result, context::Context, flow::FlowExecutor};
use async_trait::async_trait;
use crawler_schema::ContentFlow;

/// 内容请求
#[derive(Debug, Clone)]
pub struct ContentRequest {
    /// 内容页 URL
    pub url: String,
}

/// 内容响应
#[derive(Debug, Clone)]
pub struct ContentResponse {
    /// 内容数据
    pub data: serde_json::Value,
}

/// 内容流程执行器
pub struct ContentFlowExecutor {
    flow: ContentFlow,
}

impl ContentFlowExecutor {
    pub fn new(flow: ContentFlow) -> Self {
        Self { flow }
    }
}

#[async_trait]
impl FlowExecutor for ContentFlowExecutor {
    type Input = ContentRequest;
    type Output = ContentResponse;

    async fn execute(&self, input: Self::Input, context: &mut Context) -> Result<Self::Output> {
        // 设置上下文变量
        context.set("content_url", serde_json::json!(input.url));

        // TODO: 实现内容流程
        let _ = &self.flow;

        Ok(ContentResponse {
            data: serde_json::json!({}),
        })
    }
}
