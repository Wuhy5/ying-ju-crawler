//! # 登录流程执行器

use crate::{Result, context::Context, flow::FlowExecutor};
use async_trait::async_trait;
use crawler_schema::LoginFlow;

/// 登录请求
#[derive(Debug, Clone)]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone)]
pub struct LoginResponse {
    /// 是否成功
    pub success: bool,
    /// 会话信息
    pub session: Option<serde_json::Value>,
}

/// 登录流程执行器
pub struct LoginFlowExecutor {
    flow: LoginFlow,
}

impl LoginFlowExecutor {
    pub fn new(flow: LoginFlow) -> Self {
        Self { flow }
    }
}

#[async_trait]
impl FlowExecutor for LoginFlowExecutor {
    type Input = LoginRequest;
    type Output = LoginResponse;

    async fn execute(&self, input: Self::Input, context: &mut Context) -> Result<Self::Output> {
        // 设置上下文变量
        context.set("username", serde_json::json!(input.username));
        context.set("password", serde_json::json!(input.password));

        // TODO: 实现登录流程
        let _ = &self.flow;

        Ok(LoginResponse {
            success: false,
            session: None,
        })
    }
}
