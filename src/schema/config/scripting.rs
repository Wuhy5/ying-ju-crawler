//! 脚本配置 (ScriptingConfig)

use crate::schema::types::{Identifier, ScriptEngine};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 脚本配置 (ScriptingConfig)
/// 配置用于执行自定义逻辑的脚本环境。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScriptingConfig {
    /// 默认脚本引擎。
    pub engine: ScriptEngine,
    /// 定义脚本模块，key 为模块名，用于在 `script` 步骤中调用。
    #[schemars(with = "HashMap<Identifier, ScriptModule>")]
    pub modules: HashMap<String, ScriptModule>,
}

/// 脚本模块 (ScriptModule)
/// 定义一个脚本文件的来源。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScriptModule {
    /// 脚本代码的来源，可以是内联代码或外部文件路径。
    #[serde(flatten)]
    pub source: ScriptSource,
    /// 脚本引擎，如果指定，则覆盖全局配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<ScriptEngine>,
}

/// 脚本来源 (ScriptSource)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ScriptSource {
    /// 直接内联在规则中的代码。
    Code(String),
    /// 相对于规则文件的脚本路径。
    File(String),
    /// 远程URL加载的脚本。
    Url(String),
}
