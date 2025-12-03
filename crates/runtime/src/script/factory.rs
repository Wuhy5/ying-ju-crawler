//! 脚本引擎工厂

use crate::script::*;
use std::sync::Arc;

/// 脚本语言类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptLanguage {
    /// Rhai (Rust-like 语法)
    Rhai,
    /// JavaScript (ECMAScript)
    JavaScript,
    /// Lua
    Lua,
    /// Python
    Python,
}

impl ScriptLanguage {
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rhai" | "rs" => Some(Self::Rhai),
            "javascript" | "js" | "ecmascript" => Some(Self::JavaScript),
            "lua" => Some(Self::Lua),
            "python" | "py" => Some(Self::Python),
            _ => None,
        }
    }

    /// 转为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rhai => "rhai",
            Self::JavaScript => "javascript",
            Self::Lua => "lua",
            Self::Python => "python",
        }
    }
}

/// 脚本引擎工厂
pub struct ScriptEngineFactory;

impl ScriptEngineFactory {
    /// 创建指定语言的脚本引擎
    pub fn create(language: ScriptLanguage) -> Arc<dyn ScriptEngine> {
        match language {
            ScriptLanguage::Rhai => Arc::new(RhaiScriptEngine::new()),
            ScriptLanguage::JavaScript => Arc::new(JsScriptEngine::new()),
            ScriptLanguage::Lua => Arc::new(LuaScriptEngine::new()),
            ScriptLanguage::Python => {
                Arc::new(PythonScriptEngine::new().expect("Failed to create Python engine"))
            }
        }
    }

    /// 从字符串创建引擎
    pub fn create_from_str(lang: &str) -> Option<Arc<dyn ScriptEngine>> {
        ScriptLanguage::from_str(lang).map(Self::create)
    }

    /// 创建默认引擎 (Rhai)
    pub fn create_default() -> Arc<dyn ScriptEngine> {
        Self::create(ScriptLanguage::Rhai)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        let rhai = ScriptEngineFactory::create(ScriptLanguage::Rhai);
        assert_eq!(rhai.engine_name(), "rhai");

        let js = ScriptEngineFactory::create(ScriptLanguage::JavaScript);
        assert_eq!(js.engine_name(), "javascript");

        let lua = ScriptEngineFactory::create(ScriptLanguage::Lua);
        assert_eq!(lua.engine_name(), "lua");

        let python = ScriptEngineFactory::create(ScriptLanguage::Python);
        assert_eq!(python.engine_name(), "python");
    }

    #[test]
    fn test_parse_language() {
        assert_eq!(ScriptLanguage::from_str("rhai"), Some(ScriptLanguage::Rhai));
        assert_eq!(
            ScriptLanguage::from_str("js"),
            Some(ScriptLanguage::JavaScript)
        );
        assert_eq!(ScriptLanguage::from_str("lua"), Some(ScriptLanguage::Lua));
        assert_eq!(
            ScriptLanguage::from_str("python"),
            Some(ScriptLanguage::Python)
        );
        assert_eq!(ScriptLanguage::from_str("py"), Some(ScriptLanguage::Python));
        assert_eq!(ScriptLanguage::from_str("unknown"), None);
    }
}
