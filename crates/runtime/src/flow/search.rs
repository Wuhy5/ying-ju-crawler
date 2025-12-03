//! # 搜索流程执行器

use crate::{Result, context::Context, flow::FlowExecutor};
use async_trait::async_trait;
use crawler_schema::SearchFlow;

/// 搜索请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 搜索关键词
    pub keyword: String,
    /// 页码
    pub page: u32,
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResponse {
    /// 搜索结果列表
    pub items: Vec<serde_json::Value>,
    /// 是否有下一页
    pub has_next: bool,
}

/// 搜索流程执行器
pub struct SearchFlowExecutor {
    flow: SearchFlow,
}

impl SearchFlowExecutor {
    pub fn new(flow: SearchFlow) -> Self {
        Self { flow }
    }
}

#[async_trait]
impl FlowExecutor for SearchFlowExecutor {
    type Input = SearchRequest;
    type Output = SearchResponse;

    async fn execute(&self, input: Self::Input, context: &mut Context) -> Result<Self::Output> {
        // 设置上下文变量
        context.set("keyword", serde_json::json!(input.keyword));
        context.set("page", serde_json::json!(input.page));

        // TODO: 实现搜索流程
        // 1. 渲染 URL
        // 2. 发起 HTTP 请求
        // 3. 提取数据
        let _ = &self.flow;

        Ok(SearchResponse {
            items: vec![],
            has_next: false,
        })
    }
}
