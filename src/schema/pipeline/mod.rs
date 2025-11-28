//! 管道与步骤 (Pipeline & Step)

pub mod steps;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub use steps::*;

use crate::{
    error::{CrawlerError, ValidationErrors},
    schema::template::Template,
};

/// 步骤Trait (StepTrait)
/// 为步骤提供统一的接口，支持插件系统和验证。
pub trait StepTrait {
    /// 步骤的唯一名称标识
    fn name(&self) -> &'static str;

    /// 步骤的人类可读描述
    fn description(&self) -> &'static str {
        "未提供描述"
    }

    /// 步骤的类别（用于分组显示）
    fn category(&self) -> StepCategory {
        StepCategory::Other
    }

    /// 获取此步骤使用的所有模板
    fn templates(&self) -> Vec<&Template> {
        Vec::new()
    }

    /// 获取此步骤的输出变量名
    fn output_variable(&self) -> Option<&str> {
        None
    }

    /// 获取此步骤依赖的变量名（从模板中提取）
    /// 注意：此方法需要使用 runtime::TemplateExt trait
    fn required_variables(&self) -> HashSet<String> {
        HashSet::new()
    }

    /// 验证步骤配置（基本验证，不含运行时逻辑）
    fn validate(&self, _errors: &mut ValidationErrors) {
        // 默认不做任何验证，具体验证逻辑由 runtime 模块提供
    }
}

/// 步骤类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StepCategory {
    /// 核心操作（HTTP请求、选择器等）
    Core,
    /// 数据处理（模板、常量、映射等）
    Data,
    /// 控制流（循环、条件等）
    Control,
    /// 缓存操作
    Cache,
    /// 调试工具
    Debug,
    /// 其他
    Other,
}

/// 管道 (Pipeline)
/// 一个由多个步骤组成的执行序列。
pub type Pipeline = Vec<Step>;

/// 步骤 (Step)
/// 管道中的一个原子操作单元。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Step {
    // --- 核心操作 ---
    /// **HTTP请求**: 发起网络请求。
    HttpRequest(steps::StepHttpRequest),
    /// **CSS选择器**: 从HTML中提取单个元素。
    Selector(steps::StepSelector),
    /// **CSS选择器(全部)**: 从HTML中提取所有匹配的元素数组。
    SelectorAll(steps::StepSelectorAll),
    /// **JSONPath**: 从JSON数据中提取信息。
    JsonPath(steps::StepJsonPath),
    /// **执行脚本**: 调用脚本模块中的函数。
    Script(steps::StepScript),
    /// **调用组件**: 执行一个在 `[components]` 中定义的组件。
    Call(steps::StepCall),

    // --- 数据处理与转换 ---
    /// **字符串操作: 模板**: 使用变量格式化字符串。
    StringTemplate(steps::StepStringTemplate),
    /// **设置常量**: 创建一个值为常量的变量。
    Constant(steps::StepConstant),
    /// **字段映射**: 将解析层字段映射到渲染层标准模型。
    MapField(steps::StepMapField),

    // --- 缓存操作 ---
    /// **缓存获取**: 从缓存中获取值。
    CacheGet(steps::StepCacheGet),
    /// **缓存设置**: 将值存入缓存。
    CacheSet(steps::StepCacheSet),

    // --- 控制流 ---
    /// **循环: ForEach**: 遍历数组中的每一项并执行子管道。
    LoopForEach(steps::StepLoopForEach),

    // --- 调试 ---
    /// **日志输出**: 打印调试信息。
    Log(steps::StepLog),
}

impl Step {
    /// 获取步骤的内部trait实现
    fn as_trait(&self) -> &dyn StepTrait {
        match self {
            Step::HttpRequest(s) => s,
            Step::Selector(s) => s,
            Step::SelectorAll(s) => s,
            Step::JsonPath(s) => s,
            Step::Script(s) => s,
            Step::Call(s) => s,
            Step::StringTemplate(s) => s,
            Step::Constant(s) => s,
            Step::MapField(s) => s,
            Step::CacheGet(s) => s,
            Step::CacheSet(s) => s,
            Step::LoopForEach(s) => s,
            Step::Log(s) => s,
        }
    }
}

impl StepTrait for Step {
    fn name(&self) -> &'static str {
        self.as_trait().name()
    }

    fn description(&self) -> &'static str {
        self.as_trait().description()
    }

    fn category(&self) -> StepCategory {
        self.as_trait().category()
    }

    fn templates(&self) -> Vec<&Template> {
        self.as_trait().templates()
    }

    fn output_variable(&self) -> Option<&str> {
        self.as_trait().output_variable()
    }

    fn required_variables(&self) -> HashSet<String> {
        self.as_trait().required_variables()
    }

    fn validate(&self, errors: &mut ValidationErrors) {
        self.as_trait().validate(errors)
    }
}

/// 管道扩展trait
pub trait PipelineExt {
    /// 验证管道中的所有步骤
    fn validate(&self) -> Result<(), CrawlerError>;

    /// 获取管道中所有步骤的输出变量
    fn output_variables(&self) -> HashSet<String>;

    /// 获取管道依赖的所有外部变量
    fn required_external_variables(&self) -> HashSet<String>;
}

impl PipelineExt for Pipeline {
    fn validate(&self) -> Result<(), CrawlerError> {
        let mut errors = ValidationErrors::new();

        for (index, step) in self.iter().enumerate() {
            let mut step_errors = ValidationErrors::new();
            step.validate(&mut step_errors);

            for err in step_errors {
                errors.push(CrawlerError::PipelineValidation {
                    step_index: index,
                    message: err.to_string(),
                });
            }
        }

        errors.into_result()
    }

    fn output_variables(&self) -> HashSet<String> {
        self.iter()
            .filter_map(|step| step.output_variable())
            .map(|s| s.to_string())
            .collect()
    }

    fn required_external_variables(&self) -> HashSet<String> {
        let mut required: HashSet<String> = HashSet::new();
        let mut defined: HashSet<String> = HashSet::new();

        for step in self.iter() {
            // 收集此步骤需要的变量（排除已定义的）
            for var in step.required_variables() {
                let root_var = var.split('.').next().unwrap_or(&var);
                let root_var = root_var.split('[').next().unwrap_or(root_var);
                if !defined.contains(root_var) {
                    required.insert(root_var.to_string());
                }
            }

            // 添加此步骤定义的变量
            if let Some(output) = step.output_variable() {
                defined.insert(output.to_string());
            }
        }

        required
    }
}
