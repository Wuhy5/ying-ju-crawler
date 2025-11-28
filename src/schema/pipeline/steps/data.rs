//! 数据处理与转换步骤 (Data Processing Steps)

use crate::{
    error::{CrawlerError, ValidationErrors},
    schema::{
        pipeline::{StepCategory, StepTrait},
        template::Template,
        types::{CacheScope, Identifier},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ItemSummary的有效字段列表
const ITEM_SUMMARY_FIELDS: &[&str] = &[
    "id",
    "title",
    "url",
    "media_type",
    "cover",
    "summary",
    "tags",
    "meta",
];

/// ItemDetail的有效字段列表
const ITEM_DETAIL_FIELDS: &[&str] = &[
    "id",
    "title",
    "url",
    "media_type",
    "cover",
    "description",
    "metadata",
    "tags",
    "content",
];

/// 字符串操作: 模板步骤 (StepStringTemplate)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepStringTemplate {
    /// 模板字符串。
    #[serde(rename = "template")]
    pub template_str: Template,
    /// 输出结果变量名。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// 设置常量步骤 (StepConstant)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepConstant {
    /// 常量值。
    pub value: serde_json::Value,
    /// 将此常量存入的变量名。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// 字段映射步骤 (StepMapField)
/// 用于将解析层的自由格式字段映射到渲染层的标准化模型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepMapField {
    /// 输入数据的变量名或模板字符串。
    /// 模板字符串，详见顶部规范说明。
    pub input: Template,
    /// 目标模型类型: "item_summary" 或 "item_detail"
    #[schemars(regex(pattern = "^(item_summary|item_detail)$"))]
    pub target: String,
    /// 字段映射规则列表。
    pub mappings: Vec<FieldMapping>,
    /// 将映射结果存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// 缓存获取步骤 (StepCacheGet)
/// 从缓存中获取值。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepCacheGet {
    /// 缓存键。
    /// 模板字符串，详见顶部规范说明。
    pub key: Template,
    /// 将获取的值存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
    /// 缓存作用域。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CacheScope>,
}

/// 缓存设置步骤 (StepCacheSet)
/// 将值存入缓存。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepCacheSet {
    /// 缓存键。
    /// 模板字符串，详见顶部规范说明。
    pub key: Template,
    /// 要缓存的值。
    /// 模板字符串，详见顶部规范说明。
    pub value: Template,
    /// 过期时间（秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
    /// 缓存作用域。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CacheScope>,
}

/// 字段映射规则 (FieldMapping)
/// 定义源字段到目标字段的映射关系。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FieldMapping {
    /// 源字段路径，支持点号访问嵌套字段，如 "data.title" 或简单字段名 "title"
    pub from: String,
    /// 目标字段名，必须是目标模型中的有效字段名
    /// 如 ItemSummary: id, title, url, media_type, cover, summary, tags, meta
    /// 如 ItemDetail: id, title, url, media_type, cover, description, metadata, tags, content
    pub to: String,
    /// 可选的转换函数（预留），如 "to_lowercase", "trim", "parse_int" 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}

// --- StepTrait 实现 ---

impl StepTrait for StepStringTemplate {
    fn name(&self) -> &'static str {
        "string_template"
    }

    fn description(&self) -> &'static str {
        "使用变量格式化字符串模板"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Data
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.template_str]
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}

impl StepTrait for StepConstant {
    fn name(&self) -> &'static str {
        "constant"
    }

    fn description(&self) -> &'static str {
        "创建一个常量值变量"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Data
    }

    fn templates(&self) -> Vec<&Template> {
        vec![]
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}

impl StepTrait for StepMapField {
    fn name(&self) -> &'static str {
        "map_field"
    }

    fn description(&self) -> &'static str {
        "将解析层字段映射到渲染层标准模型"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Data
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.input]
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }

    fn validate(&self, errors: &mut ValidationErrors) {
        // 验证target类型 (静态schema验证)
        if self.target != "item_summary" && self.target != "item_detail" {
            errors.push(CrawlerError::InvalidConfigValue {
                field: "target".to_string(),
                reason: "必须是 'item_summary' 或 'item_detail'".to_string(),
            });
            return;
        }

        // 验证字段映射
        let valid_fields = if self.target == "item_summary" {
            ITEM_SUMMARY_FIELDS
        } else {
            ITEM_DETAIL_FIELDS
        };

        for mapping in &self.mappings {
            if !valid_fields.contains(&mapping.to.as_str()) {
                errors.push(CrawlerError::InvalidFieldMapping {
                    field: mapping.to.clone(),
                    model: self.target.clone(),
                });
            }
        }
    }
}

impl StepTrait for StepCacheGet {
    fn name(&self) -> &'static str {
        "cache_get"
    }

    fn description(&self) -> &'static str {
        "从缓存中获取值"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Cache
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.key]
    }

    fn output_variable(&self) -> Option<&str> {
        Some(&self.output)
    }
}

impl StepTrait for StepCacheSet {
    fn name(&self) -> &'static str {
        "cache_set"
    }

    fn description(&self) -> &'static str {
        "将值存入缓存"
    }

    fn category(&self) -> StepCategory {
        StepCategory::Cache
    }

    fn templates(&self) -> Vec<&Template> {
        vec![&self.key, &self.value]
    }

    fn output_variable(&self) -> Option<&str> {
        None // cache_set没有输出变量
    }
}
