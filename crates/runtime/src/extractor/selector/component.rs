//! # 组件引用执行器
//!
//! 处理 `use_component` 步骤，引用预定义的可复用组件。
//!
//! 组件执行需要在运行时解析组件定义并执行其提取逻辑。
//! 当前实现为占位符，完整实现需要访问全局组件注册表。

use crate::{
    Result,
    context::Context,
    extractor::value::{ExtractValueData, SharedValue},
};
use crawler_schema::flow::ComponentRef;
use std::sync::Arc;

/// 组件引用执行器
pub struct ComponentExecutor;

impl ComponentExecutor {
    /// 获取组件名称
    fn component_name(component_ref: &ComponentRef) -> &str {
        match component_ref {
            ComponentRef::Simple(name) => name,
            ComponentRef::WithArgs { name, .. } => name,
        }
    }

    /// 执行组件引用
    pub fn execute(
        component_ref: &ComponentRef,
        input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        // TODO: 完整实现需要：
        // 1. 从上下文获取全局组件注册表
        // 2. 根据名称查找组件定义
        // 3. 合并参数（组件默认 inputs + 调用时的 args）
        // 4. 执行组件的 extractor 步骤
        //
        // 当前返回输入值作为占位
        let _ = Self::component_name(component_ref); // 避免 dead_code 警告
        Ok(Arc::new(input.clone()))
    }
}
