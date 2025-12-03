//! Rhai 引擎内置函数

use rhai::Engine;

/// 注册所有内置函数到 Rhai 引擎
pub fn register_all(engine: &mut Engine) {
    register_string_functions(engine);
    register_json_functions(engine);
    register_encoding_functions(engine);
    register_regex_functions(engine);
}

fn register_string_functions(engine: &mut Engine) {
    engine.register_fn("trim", |s: &str| s.trim().to_string());
    engine.register_fn("lower", |s: &str| s.to_lowercase());
    engine.register_fn("upper", |s: &str| s.to_uppercase());
    engine.register_fn("replace", |s: &str, from: &str, to: &str| {
        s.replace(from, to)
    });
    engine.register_fn("split", |s: &str, sep: &str| {
        s.split(sep).map(|s| s.to_string()).collect::<Vec<_>>()
    });
}

fn register_json_functions(engine: &mut Engine) {
    engine.register_fn(
        "json_parse",
        |s: &str| -> Result<rhai::Dynamic, Box<rhai::EvalAltResult>> {
            serde_json::from_str::<serde_json::Value>(s)
                .map(dynamic_from_json)
                .map_err(|e| e.to_string().into())
        },
    );
}

fn register_encoding_functions(engine: &mut Engine) {
    engine.register_fn("base64_encode", |s: &str| {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.encode(s)
    });

    engine.register_fn("url_encode", |s: &str| urlencoding::encode(s).to_string());
    engine.register_fn("md5", |s: &str| format!("{:x}", md5::compute(s)));
}

fn register_regex_functions(engine: &mut Engine) {
    engine.register_fn("regex_match", |pattern: &str, text: &str| -> bool {
        regex::Regex::new(pattern)
            .map(|re| re.is_match(text))
            .unwrap_or(false)
    });
}

fn dynamic_from_json(value: serde_json::Value) -> rhai::Dynamic {
    match value {
        serde_json::Value::Null => rhai::Dynamic::UNIT,
        serde_json::Value::Bool(b) => rhai::Dynamic::from(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                rhai::Dynamic::from(i)
            } else if let Some(f) = n.as_f64() {
                rhai::Dynamic::from(f)
            } else {
                rhai::Dynamic::UNIT
            }
        }
        serde_json::Value::String(s) => rhai::Dynamic::from(s),
        serde_json::Value::Array(arr) => {
            rhai::Dynamic::from(arr.into_iter().map(dynamic_from_json).collect::<Vec<_>>())
        }
        serde_json::Value::Object(obj) => {
            let map: rhai::Map = obj
                .into_iter()
                .map(|(k, v)| (k.into(), dynamic_from_json(v)))
                .collect();
            rhai::Dynamic::from(map)
        }
    }
}
