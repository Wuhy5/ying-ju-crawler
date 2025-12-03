//! # 发现流程执行器

use crate::{Result, context::Context, flow::FlowExecutor};
use async_trait::async_trait;
use crawler_schema::DiscoveryFlow;

/// 发现请求
#[derive(Debug, Clone)]
pub struct DiscoveryRequest {
    /// 筛选条件
    pub filters: std::collections::HashMap<String, String>,
    /// 页码
    pub page: u32,
}

/// 发现响应
#[derive(Debug, Clone)]
pub struct DiscoveryResponse {
    /// 结果列表
    pub items: Vec<serde_json::Value>,
    /// 是否有下一页
    pub has_next: bool,
}

/// 发现流程执行器
pub struct DiscoveryFlowExecutor {
    flow: DiscoveryFlow,
}

impl DiscoveryFlowExecutor {
    pub fn new(flow: DiscoveryFlow) -> Self {
        Self { flow }
    }
}

#[async_trait]
impl FlowExecutor for DiscoveryFlowExecutor {
    type Input = DiscoveryRequest;
    type Output = DiscoveryResponse;

    async fn execute(&self, input: Self::Input, context: &mut Context) -> Result<Self::Output> {
        // 设置上下文变量
        for (key, value) in &input.filters {
            context.set(key, serde_json::json!(value));
        }
        context.set("page", serde_json::json!(input.page));

        // TODO: 实现发现流程
        let _ = &self.flow;

        Ok(DiscoveryResponse {
            items: vec![],
            has_next: false,
        })
    }
}
