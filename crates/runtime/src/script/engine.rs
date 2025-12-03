//! 脚本引擎统一抽象接口

use crate::{Result, script::context::ScriptContext};
use std::time::Duration;

/// 脚本引擎统一接口
///
/// 所有脚本引擎(Rhai/JS/Python/Lua)都实现此 trait
pub trait ScriptEngine: Send + Sync {
    /// 执行脚本并返回字符串结果
    fn execute(&self, script: &str, context: &ScriptContext) -> Result<String>;

    /// 执行脚本并返回 JSON 值
    fn execute_json(&self, script: &str, context: &ScriptContext) -> Result<serde_json::Value>;

    /// 设置执行超时
    fn set_timeout(&mut self, duration: Duration);

    /// 获取引擎类型名称
    fn engine_name(&self) -> &str;
}
