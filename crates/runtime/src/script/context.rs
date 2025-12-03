//! 脚本执行上下文

use serde_json::Value;
use std::collections::HashMap;

/// 脚本执行上下文
///
/// 包含脚本执行时可访问的所有数据和服务
#[derive(Debug, Clone)]
pub struct ScriptContext {
    /// 当前输入值（提取流程的中间结果）
    pub input: String,

    /// 上下文变量（模板变量、提取的字段等）
    pub variables: HashMap<String, Value>,
    // TODO: 添加更多服务
    // pub http_client: Arc<HttpClient>,
    // pub cookie_jar: Arc<CookieJar>,
    // pub cache: Arc<RwLock<HashMap<String, Value>>>,
}

impl ScriptContext {
    /// 创建新的脚本上下文
    pub fn new(input: String, variables: HashMap<String, Value>) -> Self {
        Self { input, variables }
    }

    /// 设置输入值
    pub fn with_input(mut self, input: String) -> Self {
        self.input = input;
        self
    }

    /// 添加变量
    pub fn with_variable(mut self, key: String, value: Value) -> Self {
        self.variables.insert(key, value);
        self
    }
}

impl Default for ScriptContext {
    fn default() -> Self {
        Self {
            input: String::new(),
            variables: HashMap::new(),
        }
    }
}
