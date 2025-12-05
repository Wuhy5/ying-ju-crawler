//! # 变量存储
//!
//! 管理运行时变量的存储和访问

use serde_json::Value;
use std::collections::HashMap;

/// 变量存储
///
/// 支持嵌套变量访问，如 `user.name`、`items[0]`
#[derive(Debug, Clone, Default)]
pub struct VariableStore {
    variables: HashMap<String, Value>,
}

impl VariableStore {
    /// 创建新的变量存储
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置变量
    pub fn set<K: Into<String>>(&mut self, key: K, value: Value) {
        self.variables.insert(key.into(), value);
    }

    /// 获取变量
    pub fn get(&self, key: &str) -> Option<&Value> {
        // 支持嵌套访问：user.name 或 items[0]
        if key.contains('.') || key.contains('[') {
            self.get_nested(key)
        } else {
            self.variables.get(key)
        }
    }

    /// 获取嵌套变量
    ///
    /// 支持：
    /// - `user.name`
    /// - `items[0]`
    /// - `user.addresses[0].city`
    fn get_nested(&self, path: &str) -> Option<&Value> {
        let parts = parse_path(path);
        if parts.is_empty() {
            return None;
        }

        let first_key = match &parts[0] {
            PathPart::Key(k) => k,
            PathPart::Index(_) => return None,
        };

        let mut current = self.variables.get(first_key.as_str())?;

        for part in &parts[1..] {
            match part {
                PathPart::Key(key) => {
                    current = current.get(key.as_str())?;
                }
                PathPart::Index(idx) => {
                    current = current.get(idx)?;
                }
            }
        }

        Some(current)
    }

    /// 获取所有变量
    pub fn all(&self) -> &HashMap<String, Value> {
        &self.variables
    }

    /// 合并另一个变量存储
    pub fn merge(&mut self, other: &VariableStore) {
        self.variables.extend(other.variables.clone());
    }

    /// 清空所有变量
    pub fn clear(&mut self) {
        self.variables.clear();
    }

    /// 检查变量是否存在
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// 移除变量
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.variables.remove(key)
    }

    /// 变量数量
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

/// 路径部分
#[derive(Debug, Clone, PartialEq)]
enum PathPart {
    /// 对象键
    Key(String),
    /// 数组索引
    Index(usize),
}

/// 解析路径
///
/// 将 `user.name` 或 `items[0]` 解析为路径部分
fn parse_path(path: &str) -> Vec<PathPart> {
    let mut parts = Vec::new();
    let mut current = String::new();

    for ch in path.chars() {
        match ch {
            '.' => {
                if !current.is_empty() {
                    parts.push(PathPart::Key(current.clone()));
                    current.clear();
                }
            }
            '[' => {
                if !current.is_empty() {
                    parts.push(PathPart::Key(current.clone()));
                    current.clear();
                }
            }
            ']' => {
                if !current.is_empty() {
                    if let Ok(idx) = current.parse::<usize>() {
                        parts.push(PathPart::Index(idx));
                    }
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        parts.push(PathPart::Key(current));
    }

    parts
}
