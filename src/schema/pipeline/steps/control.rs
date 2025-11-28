//! 控制流步骤 (Control Flow Steps)

use crate::schema::{
    pipeline::{Pipeline, StepCategory, StepTrait},
    template::Template,
    types::Identifier,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 循环: ForEach步骤 (StepLoopForEach)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepLoopForEach {
    /// 要遍历的数组变量。
    /// 模板字符串，详见顶部规范说明。
    pub input: Template,
    /// 在循环中，当前项的变量名。
    #[serde(rename = "as")]
    #[schemars(with = "Identifier")]
    pub r#as: String,
    /// 对每一项执行的子管道。
    pub pipeline: Pipeline,
}

/// 日志输出步骤 (StepLog)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepLog {
    /// 要打印的消息。
    /// 模板字符串，详见顶部规范说明。
    pub message: Template,
}

// --- StepTrait 实现 ---

impl StepTrait for StepLoopForEach {
    fn name(&self) -> &'static str {
        "loop_for_each"
    }

    fn description(&self) -> &'static str {
        "遍历数组中的每一项并执行子管道"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Control
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.input]
    }

    fn output_variable(&self) -> Option<&str> {
        None // 循环本身没有输出，内部管道可能有输出
    }
}

impl StepTrait for StepLog {
    fn name(&self) -> &'static str {
        "log"
    }

    fn description(&self) -> &'static str {
        "打印调试信息"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Debug
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.message]
    }

    fn output_variable(&self) -> Option<&str> {
        None
    }
}
