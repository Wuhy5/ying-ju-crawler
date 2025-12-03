use mlua::{Lua, Result as LuaResult, Value};

/// 为 Lua 引擎注册内置函数
pub fn register_builtin_functions(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // 字符串处理函数
    let trim_fn = lua.create_function(|_, s: String| Ok(s.trim().to_string()))?;
    globals.set("trim", trim_fn)?;

    let lower_fn = lua.create_function(|_, s: String| Ok(s.to_lowercase()))?;
    globals.set("lower", lower_fn)?;

    let upper_fn = lua.create_function(|_, s: String| Ok(s.to_uppercase()))?;
    globals.set("upper", upper_fn)?;

    let replace_fn = lua
        .create_function(|_, (s, from, to): (String, String, String)| Ok(s.replace(&from, &to)))?;
    globals.set("replace", replace_fn)?;

    let split_fn = lua.create_function(|lua, (s, sep): (String, String)| {
        let table = lua.create_table()?;
        for (i, part) in s.split(&sep).enumerate() {
            table.set(i + 1, part)?;
        }
        Ok(table)
    })?;
    globals.set("split", split_fn)?;

    // JSON 解析
    let json_parse_fn = lua.create_function(|lua, s: String| {
        let value: serde_json::Value = serde_json::from_str(&s)
            .map_err(|e| mlua::Error::RuntimeError(format!("JSON 解析失败: {}", e)))?;
        json_to_lua(lua, &value)
    })?;
    globals.set("json_parse", json_parse_fn)?;

    // 编码函数
    let base64_encode_fn = lua.create_function(|_, s: String| {
        use base64::Engine;
        Ok(base64::engine::general_purpose::STANDARD.encode(s))
    })?;
    globals.set("base64_encode", base64_encode_fn)?;

    let url_encode_fn =
        lua.create_function(|_, s: String| Ok(urlencoding::encode(&s).to_string()))?;
    globals.set("url_encode", url_encode_fn)?;

    let md5_fn = lua.create_function(|_, s: String| {
        let digest = md5::compute(s.as_bytes());
        Ok(format!("{:x}", digest))
    })?;
    globals.set("md5", md5_fn)?;

    // 正则匹配
    let regex_match_fn = lua.create_function(|lua, (text, pattern): (String, String)| {
        let re = regex::Regex::new(&pattern)
            .map_err(|e| mlua::Error::RuntimeError(format!("正则表达式错误: {}", e)))?;

        if let Some(captures) = re.captures(&text) {
            let table = lua.create_table()?;
            for (i, cap) in captures.iter().enumerate() {
                if let Some(m) = cap {
                    table.set(i, m.as_str())?;
                }
            }
            Ok(Some(table))
        } else {
            Ok(None)
        }
    })?;
    globals.set("regex_match", regex_match_fn)?;

    Ok(())
}

/// 将 serde_json::Value 转换为 Lua Value
fn json_to_lua(lua: &Lua, value: &serde_json::Value) -> LuaResult<Value> {
    match value {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err(mlua::Error::RuntimeError("无法转换数字".to_string()))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(lua.create_string(s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, v) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, v)?)?;
            }
            Ok(Value::Table(table))
        }
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (k, v) in obj {
                table.set(k.as_str(), json_to_lua(lua, v)?)?;
            }
            Ok(Value::Table(table))
        }
    }
}
