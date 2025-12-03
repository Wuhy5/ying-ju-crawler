//! Rhai 脚本引擎实现

use crate::{
    Result,
    error::RuntimeError,
    script::{ScriptContext, ScriptEngine},
};
use rhai::{AST, Dynamic, Engine, Scope};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

/// Rhai 脚本引擎
pub struct RhaiScriptEngine {
    /// Rhai 引擎实例
    engine: Arc<Mutex<Engine>>,
    /// 编译缓存
    ast_cache: Arc<Mutex<lru::LruCache<String, Arc<AST>>>>,
    /// 执行超时设置
    timeout: Duration,
}

impl RhaiScriptEngine {
    /// 创建新的 Rhai 引擎
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // 基础配置
        engine.set_max_expr_depths(100, 50);
        engine.set_max_string_size(1024 * 1024);
        engine.set_max_array_size(10000);

        // 注册内置函数
        super::builtin::rhai::register_all(&mut engine);

        Self {
            engine: Arc::new(Mutex::new(engine)),
            ast_cache: Arc::new(Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(100).unwrap(),
            ))),
            timeout: Duration::from_secs(5),
        }
    }

    /// 编译脚本（带缓存）
    fn compile_cached(&self, script: &str) -> Result<Arc<AST>> {
        {
            let cache = self.ast_cache.lock().unwrap();
            if let Some(ast) = cache.peek(script) {
                return Ok(Arc::clone(ast));
            }
        }

        let engine = self.engine.lock().unwrap();
        let ast = engine
            .compile(script)
            .map_err(|e| RuntimeError::ScriptSyntax(format!("[Rhai] {}", e)))?;

        let ast = Arc::new(ast);

        {
            let mut cache = self.ast_cache.lock().unwrap();
            cache.put(script.to_string(), Arc::clone(&ast));
        }

        Ok(ast)
    }

    /// 创建脚本作用域
    fn create_scope(&self, context: &ScriptContext) -> Scope {
        let mut scope = Scope::new();

        for (key, value) in &context.variables {
            scope.push_dynamic(key.clone(), dynamic_from_json(value.clone()));
        }

        scope.push("input", context.input.clone());

        scope
    }
}

impl ScriptEngine for RhaiScriptEngine {
    fn execute(&self, script: &str, context: &ScriptContext) -> Result<String> {
        let ast = self.compile_cached(script)?;
        let mut scope = self.create_scope(context);
        let engine = self.engine.lock().unwrap();

        let result: Dynamic = engine
            .eval_ast_with_scope(&mut scope, &ast)
            .map_err(|e| RuntimeError::ScriptRuntime(format!("[Rhai] {}", e)))?;

        Ok(result.to_string())
    }

    fn execute_json(&self, script: &str, context: &ScriptContext) -> Result<serde_json::Value> {
        let result = self.execute(script, context)?;
        serde_json::from_str(&result).or_else(|_| Ok(serde_json::Value::String(result)))
    }

    fn set_timeout(&mut self, duration: Duration) {
        self.timeout = duration;
    }

    fn engine_name(&self) -> &str {
        "rhai"
    }
}

impl Default for RhaiScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 将 serde_json::Value 转换为 Rhai Dynamic
fn dynamic_from_json(value: serde_json::Value) -> Dynamic {
    match value {
        serde_json::Value::Null => Dynamic::UNIT,
        serde_json::Value::Bool(b) => Dynamic::from(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Dynamic::from(i)
            } else if let Some(f) = n.as_f64() {
                Dynamic::from(f)
            } else {
                Dynamic::UNIT
            }
        }
        serde_json::Value::String(s) => Dynamic::from(s),
        serde_json::Value::Array(arr) => {
            let vec: Vec<Dynamic> = arr.into_iter().map(dynamic_from_json).collect();
            Dynamic::from(vec)
        }
        serde_json::Value::Object(obj) => {
            let map: rhai::Map = obj
                .into_iter()
                .map(|(k, v)| (k.into(), dynamic_from_json(v)))
                .collect();
            Dynamic::from(map)
        }
    }
}
