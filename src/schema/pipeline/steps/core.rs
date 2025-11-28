//! 核心操作步骤 (Core Steps)

use crate::{
    error::ValidationErrors,
    schema::{
        pipeline::{StepCategory, StepTrait},
        template::Template,
        types::{ExtractType, HttpMethod, Identifier},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// HTTP请求步骤 (StepHttpRequest)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepHttpRequest {
    /// 要请求的URL。
    /// 模板字符串，详见顶部规范说明。
    pub url: Template,
    /// 将包含响应体、状态码、头的响应对象存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
    /// HTTP方法。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    /// POST请求体。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模板字符串，详见顶部规范说明。
    pub body: Option<Template>,
    /// 本次请求的自定义头。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 请求头模板字符串，详见顶部规范说明。
    pub headers: Option<std::collections::HashMap<String, Template>>,
}

/// CSS选择器步骤 (StepSelector)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepSelector {
    /// 输入的HTML字符串。
    /// 模板字符串，详见顶部规范说明。
    pub input: Template,
    /// CSS选择器表达式。
    /// 模板字符串，详见顶部规范说明。
    pub query: Template,
    /// 提取方式。
    pub extract: ExtractType,
    /// 将提取的结果存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// CSS选择器(全部)步骤 (StepSelectorAll)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepSelectorAll {
    /// 输入的HTML字符串。
    /// 模板字符串，详见顶部规范说明。
    pub input: Template,
    /// CSS选择器表达式。
    /// 模板字符串，详见顶部规范说明。
    pub query: Template,
    /// 提取方式。
    pub extract: ExtractType,
    /// 将提取的字符串数组存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// JSONPath步骤 (StepJsonPath)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepJsonPath {
    /// 输入的JSON字符串或对象。
    /// 模板字符串，详见顶部规范说明。
    pub input: Template,
    /// JSONPath 查询表达式。
    /// 模板字符串，详见顶部规范说明。
    pub query: Template,
    /// 将提取结果存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// 执行脚本步骤 (StepScript)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepScript {
    /// 要调用的函数，格式为 `模块名.函数名`。
    #[schemars(regex(pattern = r"^[a-zA-Z_][a-zA-Z0-9_]*\.[a-zA-Z_][a-zA-Z0-9_]*$"))]
    pub call: String,
    /// 传递给函数的参数。这是一个动态值，允许在模板中构建复杂的JSON对象。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模板字符串，详见顶部规范说明。
    pub with: Option<Template>,
    /// 将函数返回值存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// 调用组件步骤 (StepCall)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepCall {
    /// 要调用的组件名，必须在 `[components]` 中定义。
    #[schemars(with = "Identifier")]
    pub component: String,
    /// 传递给组件的输入参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 输入参数模板字符串，详见顶部规范说明。
    pub with: Option<std::collections::HashMap<String, Template>>,
    /// 将组件的输出（一个包含所有输出变量的Map）存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

// --- StepTrait 实现 ---

impl StepTrait for StepHttpRequest {
    fn name(&self) -> &'static str {
        "http_request"
    }

    fn description(&self) -> &'static str {
        "发起HTTP网络请求"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Core
    }

    fn templates(&self) -> Vec<&Template> {
        let mut templates = vec![&self.url];
        if let Some(ref body) = self.body {
            templates.push(body);
        }
        if let Some(ref headers) = self.headers {
            templates.extend(headers.values());
        }
        templates
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}

impl StepTrait for StepSelector {
    fn name(&self) -> &'static str {
        "selector"
    }

    fn description(&self) -> &'static str {
        "从HTML中使用CSS选择器提取单个元素"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Core
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.input, &self.query]
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}

impl StepTrait for StepSelectorAll {
    fn name(&self) -> &'static str {
        "selector_all"
    }

    fn description(&self) -> &'static str {
        "从HTML中使用CSS选择器提取所有匹配的元素"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Core
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.input, &self.query]
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}

impl StepTrait for StepJsonPath {
    fn name(&self) -> &'static str {
        "json_path"
    }

    fn description(&self) -> &'static str {
        "从JSON数据中使用JSONPath提取信息"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Core
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.input, &self.query]
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}

impl StepTrait for StepScript {
    fn name(&self) -> &'static str {
        "script"
    }

    fn description(&self) -> &'static str {
        "调用脚本模块中的函数"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Core
    }

    fn templates(&self) -> Vec<&Template> {
        self.with.as_ref().map(|t| vec![t]).unwrap_or_default()
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }

    fn validate(&self, errors: &mut ValidationErrors) {
        // 验证call格式 (这是静态schema验证，不是运行时验证)
        if !self.call.contains('.') {
            errors.push(crate::error::CrawlerError::InvalidConfigValue {
                field: "call".to_string(),
                reason: "格式必须为 '模块名.函数名'".to_string(),
            });
        }
    }
}

impl StepTrait for StepCall {
    fn name(&self) -> &'static str {
        "call"
    }

    fn description(&self) -> &'static str {
        "调用已定义的可重用组件"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Core
    }

    fn templates(&self) -> Vec<&Template> {
        self.with
            .as_ref()
            .map(|m| m.values().collect())
            .unwrap_or_default()
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}
