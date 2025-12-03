// TODO: RustPython 引擎实现
//
// 策略: rustpython Interpreter 不支持 Send/Sync (!Send + !Sync)
// 与 Boa 类似,采用无状态模式 - 每次执行时创建新的 Interpreter
//
// 需要实现的功能:
// 1. 每次 execute 时创建新的 Interpreter::without_stdlib()
// 2. 注册内置函数到 VM (trim, base64_encode, md5 等)
// 3. 实现 ScriptEngine trait:
//    - execute(&self, script: &str, context: &ScriptContext) -> Result<String>
//    - execute_json(&self, script: &str, context: &ScriptContext) -> Result<serde_json::Value>
//    - set_timeout(&mut self, duration: Duration)
//    - engine_name(&self) -> &str
// 4. JSON <-> PyObjectRef 转换
// 5. 将 ScriptContext 映射到 Python 全局变量
//
// RustPython 使用方式:
// ```rust
// let interp = Interpreter::without_stdlib(Default::default());
// interp.enter(|vm| {
//     let scope = vm.new_scope_with_builtins();
//     let code = vm.compile(source, Mode::Exec, "<embedded>".to_owned())?;
//     vm.run_code_obj(code, scope)?;
// });
// ```

use super::{context::ScriptContext, engine::ScriptEngine};
use crate::Result;
use std::time::Duration;

pub struct PythonScriptEngine;

impl PythonScriptEngine {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl ScriptEngine for PythonScriptEngine {
    fn execute(&self, _script: &str, _context: &ScriptContext) -> Result<String> {
        // TODO: 实现 Python 脚本执行
        Ok("TODO: Python execution".to_string())
    }

    fn execute_json(&self, _script: &str, _context: &ScriptContext) -> Result<serde_json::Value> {
        // TODO: 实现 Python 脚本执行并返回 JSON
        Ok(serde_json::json!(null))
    }

    fn set_timeout(&mut self, _duration: Duration) {
        // TODO: rustpython 可能支持超时,需要研究解决方案
    }

    fn engine_name(&self) -> &str {
        "python"
    }
}

impl Default for PythonScriptEngine {
    fn default() -> Self {
        Self::new().expect("创建 Python 引擎失败")
    }
}
