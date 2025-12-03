// TODO: mlua Lua 引擎实现
//
// 策略: mlua 已启用 send feature, Lua 实例支持 Send + Sync
// 可以安全地跨线程共享
//
// 需要实现的功能:
// 1. 使用 Lua::new() 创建引擎实例
// 2. 注册内置函数 (trim, base64_encode, md5 等)
// 3. 实现 ScriptEngine trait:
//    - execute(&self, script: &str, context: &ScriptContext) -> Result<String>
//    - execute_json(&self, script: &str, context: &ScriptContext) -> Result<serde_json::Value>
//    - set_timeout(&mut self, duration: Duration)
//    - engine_name(&self) -> &str
// 4. JSON <-> Lua Value 转换
// 5. 将 ScriptContext 映射到 Lua 全局变量

use super::{context::ScriptContext, engine::ScriptEngine};
use crate::Result;
use std::time::Duration;

pub struct LuaScriptEngine;

impl LuaScriptEngine {
    pub fn new() -> Self {
        Self
    }
}

impl ScriptEngine for LuaScriptEngine {
    fn execute(&self, _script: &str, _context: &ScriptContext) -> Result<String> {
        // TODO: 实现 Lua 脚本执行
        Ok("TODO: Lua execution".to_string())
    }

    fn execute_json(&self, _script: &str, _context: &ScriptContext) -> Result<serde_json::Value> {
        // TODO: 实现 Lua 脚本执行并返回 JSON
        Ok(serde_json::json!(null))
    }

    fn set_timeout(&mut self, _duration: Duration) {
        // TODO: mlua 不直接支持超时,需要研究解决方案
    }

    fn engine_name(&self) -> &str {
        "lua"
    }
}

impl Default for LuaScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}
