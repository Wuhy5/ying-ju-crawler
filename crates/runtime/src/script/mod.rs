//! 脚本执行引擎
//!
//! 支持多种脚本语言进行复杂的数据处理和转换:
//! - Rhai (Rust-like)
//! - JavaScript (通过 Boa)
//! - Lua (通过 mlua)
//! - Python (通过 RustPython)

pub mod context;
pub mod engine;
pub mod executor;
pub mod factory;

// 各引擎实现
pub mod js_engine;
pub mod lua_engine;
pub mod python_engine;
pub mod rhai_engine;

// 内置函数库
pub mod builtin;

pub use context::ScriptContext;
pub use engine::ScriptEngine;
pub use executor::ScriptExecutor;
pub use factory::{ScriptEngineFactory, ScriptLanguage};
pub use js_engine::JsScriptEngine;
pub use lua_engine::LuaScriptEngine;
pub use python_engine::PythonScriptEngine;
pub use rhai_engine::RhaiScriptEngine;
