//! 内置函数核心实现
//!
//! 此模块包含所有内置函数的纯 Rust 实现，与具体脚本引擎无关。
//! 各脚本引擎适配器只需将这些函数绑定到对应引擎的 API 即可。

use base64::{Engine as _, engine::general_purpose};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

// ============================================
// 字符串处理函数
// ============================================

/// 去除字符串首尾空白
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

/// 去除字符串左侧空白
pub fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
}

/// 去除字符串右侧空白
pub fn trim_end(s: &str) -> String {
    s.trim_end().to_string()
}

/// 转换为小写
pub fn lower(s: &str) -> String {
    s.to_lowercase()
}

/// 转换为大写
pub fn upper(s: &str) -> String {
    s.to_uppercase()
}

/// 字符串替换
pub fn replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// 字符串分割
pub fn split(s: &str, sep: &str) -> Vec<String> {
    s.split(sep).map(|s| s.to_string()).collect()
}

/// 字符串连接
pub fn join(arr: &[String], sep: &str) -> String {
    arr.join(sep)
}

/// 取子字符串
pub fn substring(s: &str, start: usize, end: Option<usize>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let end = end.unwrap_or(chars.len()).min(chars.len());
    let start = start.min(end);
    chars[start..end].iter().collect()
}

/// 检查字符串是否包含子串
pub fn contains(s: &str, pattern: &str) -> bool {
    s.contains(pattern)
}

/// 检查字符串是否以指定前缀开头
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// 检查字符串是否以指定后缀结尾
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// 获取字符串长度（字符数）
pub fn length(s: &str) -> usize {
    s.chars().count()
}

/// 在字符串中查找子串位置
pub fn index_of(s: &str, pattern: &str) -> i64 {
    s.find(pattern).map(|i| i as i64).unwrap_or(-1)
}

/// 重复字符串
pub fn repeat(s: &str, count: usize) -> String {
    s.repeat(count)
}

/// 反转字符串
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// 左侧填充
pub fn pad_start(s: &str, len: usize, pad: &str) -> String {
    let current_len = s.chars().count();
    if current_len >= len {
        return s.to_string();
    }
    let pad_count = len - current_len;
    let pad_chars: Vec<char> = pad.chars().collect();
    if pad_chars.is_empty() {
        return s.to_string();
    }
    let mut result = String::new();
    for i in 0..pad_count {
        result.push(pad_chars[i % pad_chars.len()]);
    }
    result.push_str(s);
    result
}

/// 右侧填充
pub fn pad_end(s: &str, len: usize, pad: &str) -> String {
    let current_len = s.chars().count();
    if current_len >= len {
        return s.to_string();
    }
    let pad_count = len - current_len;
    let pad_chars: Vec<char> = pad.chars().collect();
    if pad_chars.is_empty() {
        return s.to_string();
    }
    let mut result = s.to_string();
    for i in 0..pad_count {
        result.push(pad_chars[i % pad_chars.len()]);
    }
    result
}

// ============================================
// 正则表达式函数
// ============================================

/// 正则匹配测试
pub fn regex_match(pattern: &str, text: &str) -> bool {
    Regex::new(pattern).is_ok_and(|re| re.is_match(text))
}

/// 正则替换
pub fn regex_replace(text: &str, pattern: &str, replacement: &str) -> String {
    Regex::new(pattern)
        .map(|re| re.replace_all(text, replacement).to_string())
        .unwrap_or_else(|_| text.to_string())
}

/// 正则提取（返回第一个匹配）
pub fn regex_find(text: &str, pattern: &str) -> Option<String> {
    Regex::new(pattern)
        .ok()
        .and_then(|re| re.find(text).map(|m| m.as_str().to_string()))
}

/// 正则提取所有匹配
pub fn regex_find_all(text: &str, pattern: &str) -> Vec<String> {
    Regex::new(pattern)
        .map(|re| re.find_iter(text).map(|m| m.as_str().to_string()).collect())
        .unwrap_or_default()
}

/// 正则提取捕获组
pub fn regex_captures(text: &str, pattern: &str) -> Vec<String> {
    Regex::new(pattern)
        .ok()
        .and_then(|re| {
            re.captures(text).map(|caps| {
                caps.iter()
                    .filter_map(|m| m.map(|m| m.as_str().to_string()))
                    .collect()
            })
        })
        .unwrap_or_default()
}

// ============================================
// 编码/解码函数
// ============================================

/// Base64 编码
pub fn base64_encode(s: &str) -> String {
    general_purpose::STANDARD.encode(s)
}

/// Base64 解码
pub fn base64_decode(s: &str) -> Result<String, String> {
    general_purpose::STANDARD
        .decode(s)
        .map_err(|e| e.to_string())
        .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
}

/// URL 编码
pub fn url_encode(s: &str) -> String {
    urlencoding::encode(s).to_string()
}

/// URL 解码
pub fn url_decode(s: &str) -> Result<String, String> {
    urlencoding::decode(s)
        .map(|s| s.to_string())
        .map_err(|e| e.to_string())
}

/// HTML 实体编码
pub fn html_encode(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// HTML 实体解码
pub fn html_decode(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

/// 十六进制编码
pub fn hex_encode(s: &str) -> String {
    s.bytes().map(|b| format!("{:02x}", b)).collect()
}

/// 十六进制解码
pub fn hex_decode(s: &str) -> Result<String, String> {
    let bytes: Result<Vec<u8>, _> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    bytes
        .map_err(|e| e.to_string())
        .and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
}

// ============================================
// 加密/哈希函数
// ============================================

/// MD5 哈希
pub fn md5(s: &str) -> String {
    format!("{:x}", md5::compute(s))
}

/// SHA256 哈希
pub fn sha256(s: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// SHA1 哈希
pub fn sha1(s: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

// ============================================
// 中文处理函数 (使用 zhconv 库)
// ============================================

/// 繁体转简体
pub fn t2s(s: &str) -> String {
    zhconv::zhconv(s, zhconv::Variant::ZhCN)
}

/// 简体转繁体
pub fn s2t(s: &str) -> String {
    zhconv::zhconv(s, zhconv::Variant::ZhTW)
}

/// 繁体转简体（大陆标准）
pub fn to_zh_cn(s: &str) -> String {
    zhconv::zhconv(s, zhconv::Variant::ZhCN)
}

/// 简体转繁体（台湾标准）
pub fn to_zh_tw(s: &str) -> String {
    zhconv::zhconv(s, zhconv::Variant::ZhTW)
}

/// 简体转繁体（香港标准）
pub fn to_zh_hk(s: &str) -> String {
    zhconv::zhconv(s, zhconv::Variant::ZhHK)
}

/// 转换为繁体（通用）
pub fn to_zh_hant(s: &str) -> String {
    zhconv::zhconv(s, zhconv::Variant::ZhHant)
}

/// 转换为简体（通用）
pub fn to_zh_hans(s: &str) -> String {
    zhconv::zhconv(s, zhconv::Variant::ZhHans)
}

/// 判断是否为简体中文
pub fn is_hans(s: &str) -> bool {
    zhconv::is_hans(s)
}

/// 中文数字转阿拉伯数字章节
/// 例如: "第一百二十三章" -> "第123章"
pub fn to_num_chapter(s: &str) -> String {
    let re = Regex::new(r"第([零一二三四五六七八九十百千万]+)章").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let cn_num = &caps[1];
        let num = cn_to_num(cn_num);
        format!("第{}章", num)
    })
    .to_string()
}

/// 中文数字转阿拉伯数字
pub fn cn_to_num(s: &str) -> i64 {
    let mut result: i64 = 0;
    let mut temp: i64 = 0;
    let mut section: i64 = 0;

    for c in s.chars() {
        match c {
            '零' => {}
            '一' | '壹' => temp = 1,
            '二' | '贰' | '两' => temp = 2,
            '三' | '叁' => temp = 3,
            '四' | '肆' => temp = 4,
            '五' | '伍' => temp = 5,
            '六' | '陆' => temp = 6,
            '七' | '柒' => temp = 7,
            '八' | '捌' => temp = 8,
            '九' | '玖' => temp = 9,
            '十' | '拾' => {
                if temp == 0 {
                    temp = 1;
                }
                section += temp * 10;
                temp = 0;
            }
            '百' | '佰' => {
                section += temp * 100;
                temp = 0;
            }
            '千' | '仟' => {
                section += temp * 1000;
                temp = 0;
            }
            '万' => {
                section += temp;
                result += section * 10000;
                section = 0;
                temp = 0;
            }
            '亿' => {
                section += temp;
                result += section * 100000000;
                section = 0;
                temp = 0;
            }
            _ => {}
        }
    }

    result + section + temp
}

/// 阿拉伯数字转中文数字
pub fn num_to_cn(n: i64) -> String {
    if n == 0 {
        return "零".to_string();
    }

    let digits = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
    let units = ["", "十", "百", "千"];
    let big_units = ["", "万", "亿"];

    let mut result = String::new();
    let mut n = n;
    let mut big_unit_idx = 0;
    let mut need_zero = false;

    while n > 0 {
        let section = (n % 10000) as usize;
        if section > 0 {
            let mut section_str = String::new();
            let mut s = section;
            let mut unit_idx = 0;
            let mut section_need_zero = false;

            while s > 0 {
                let digit = s % 10;
                if digit > 0 {
                    if section_need_zero {
                        section_str = format!("零{}", section_str);
                    }
                    section_str = format!("{}{}{}", digits[digit], units[unit_idx], section_str);
                    section_need_zero = false;
                } else {
                    section_need_zero = true;
                }
                s /= 10;
                unit_idx += 1;
            }

            if need_zero && !result.is_empty() {
                result = format!("零{}", result);
            }
            result = format!("{}{}{}", section_str, big_units[big_unit_idx], result);
            need_zero = false;
        } else {
            need_zero = true;
        }

        n /= 10000;
        big_unit_idx += 1;
    }

    // 处理 "一十" -> "十" 的情况
    if result.starts_with("一十") {
        result = result.replacen("一十", "十", 1);
    }

    result
}

// ============================================
// JSON 处理函数
// ============================================

/// 解析 JSON 字符串
pub fn json_parse(s: &str) -> Result<Value, String> {
    serde_json::from_str(s).map_err(|e| e.to_string())
}

/// 将值转换为 JSON 字符串
pub fn json_stringify(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// 格式化 JSON 字符串（美化输出）
pub fn json_stringify_pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_default()
}

/// 获取 JSON 路径值
pub fn json_path(value: &Value, path: &str) -> Option<Value> {
    use jsonpath_rust::JsonPath;

    let path = if path.starts_with("$.") {
        path.to_string()
    } else if path.starts_with('.') {
        format!("${}", path)
    } else {
        format!("$.{}", path)
    };

    value.query(&path).ok().and_then(|results| {
        if results.is_empty() {
            None
        } else if results.len() == 1 {
            Some(results[0].clone())
        } else {
            Some(Value::Array(results.into_iter().cloned().collect()))
        }
    })
}

// ============================================
// 数组处理函数
// ============================================

/// 获取数组第一个元素
pub fn first<T: Clone>(arr: &[T]) -> Option<T> {
    arr.first().cloned()
}

/// 获取数组最后一个元素
pub fn last<T: Clone>(arr: &[T]) -> Option<T> {
    arr.last().cloned()
}

/// 获取数组指定索引元素
pub fn at<T: Clone>(arr: &[T], index: i64) -> Option<T> {
    let len = arr.len() as i64;
    let idx = if index < 0 { len + index } else { index };
    if idx >= 0 && idx < len {
        arr.get(idx as usize).cloned()
    } else {
        None
    }
}

/// 数组切片
pub fn slice<T: Clone>(arr: &[T], start: i64, end: Option<i64>) -> Vec<T> {
    let len = arr.len() as i64;
    let start = if start < 0 {
        (len + start).max(0) as usize
    } else {
        start.min(len) as usize
    };
    let end = match end {
        Some(e) if e < 0 => (len + e).max(0) as usize,
        Some(e) => e.min(len) as usize,
        None => len as usize,
    };
    if start >= end {
        vec![]
    } else {
        arr[start..end].to_vec()
    }
}

/// 数组去重
pub fn unique(arr: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    arr.iter()
        .filter(|s| seen.insert(s.to_string()))
        .cloned()
        .collect()
}

/// 数组扁平化（仅支持一层）
pub fn flatten(arr: &[Value]) -> Vec<Value> {
    arr.iter()
        .flat_map(|v| match v {
            Value::Array(inner) => inner.clone(),
            other => vec![other.clone()],
        })
        .collect()
}

// ============================================
// 类型转换函数
// ============================================

/// 转换为整数
pub fn to_int(s: &str) -> Option<i64> {
    // 尝试直接解析
    if let Ok(n) = s.parse::<i64>() {
        return Some(n);
    }
    // 尝试解析浮点数后取整
    if let Ok(f) = s.parse::<f64>() {
        return Some(f as i64);
    }
    // 提取数字部分
    let num_str: String = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    num_str.parse().ok()
}

/// 转换为浮点数
pub fn to_float(s: &str) -> Option<f64> {
    // 尝试直接解析
    if let Ok(f) = s.parse::<f64>() {
        return Some(f);
    }
    // 提取数字部分（包含小数点）
    let num_str: String = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '-' || *c == '.')
        .collect();
    num_str.parse().ok()
}

/// 转换为字符串
pub fn to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        _ => value.to_string(),
    }
}

/// 转换为布尔值
pub fn to_bool(s: &str) -> bool {
    matches!(s.to_lowercase().as_str(), "true" | "1" | "yes" | "on")
}

// ============================================
// 日期时间函数
// ============================================

/// 获取当前时间戳（秒）
pub fn timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// 获取当前时间戳（毫秒）
pub fn timestamp_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// 格式化时间戳
pub fn format_timestamp(ts: i64, format: &str) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_opt(ts, 0)
        .single()
        .map(|dt| dt.format(format).to_string())
        .unwrap_or_default()
}

/// 解析日期字符串为时间戳
pub fn parse_date(s: &str, format: &str) -> Option<i64> {
    use chrono::NaiveDateTime;
    NaiveDateTime::parse_from_str(s, format)
        .ok()
        .map(|dt| dt.and_utc().timestamp())
}

// ============================================
// URL 处理函数
// ============================================

/// 解析 URL
pub fn parse_url(url_str: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    if let Ok(url) = url::Url::parse(url_str) {
        result.insert("scheme".to_string(), url.scheme().to_string());
        result.insert(
            "host".to_string(),
            url.host_str().unwrap_or_default().to_string(),
        );
        result.insert("port".to_string(), url.port().unwrap_or(0).to_string());
        result.insert("path".to_string(), url.path().to_string());
        result.insert(
            "query".to_string(),
            url.query().unwrap_or_default().to_string(),
        );
        result.insert(
            "fragment".to_string(),
            url.fragment().unwrap_or_default().to_string(),
        );
    }
    result
}

/// 拼接 URL
pub fn join_url(base: &str, path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }
    if path.starts_with("//") {
        if let Ok(base_url) = url::Url::parse(base) {
            return format!("{}:{}", base_url.scheme(), path);
        }
        return format!("https:{}", path);
    }
    url::Url::parse(base)
        .and_then(|base_url| base_url.join(path))
        .map(|u| u.to_string())
        .unwrap_or_else(|_| {
            format!(
                "{}/{}",
                base.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        })
}

/// 获取 URL 查询参数
pub fn get_query_param(url_str: &str, key: &str) -> Option<String> {
    url::Url::parse(url_str).ok().and_then(|url| {
        url.query_pairs()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.to_string())
    })
}

/// 设置 URL 查询参数
pub fn set_query_param(url_str: &str, key: &str, value: &str) -> String {
    if let Ok(mut url) = url::Url::parse(url_str) {
        url.query_pairs_mut().append_pair(key, value);
        url.to_string()
    } else {
        url_str.to_string()
    }
}

// ============================================
// 工具函数
// ============================================

/// 生成 UUID
pub fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let random: u64 = (now.as_nanos() as u64) ^ (std::process::id() as u64);
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        (now.as_secs() & 0xFFFFFFFF) as u32,
        (random >> 48) & 0xFFFF,
        (random >> 32) & 0xFFFF,
        (random >> 16) & 0xFFFF,
        random & 0xFFFFFFFFFFFF
    )
}

/// 生成随机整数
pub fn random_int(min: i64, max: i64) -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let range = (max - min + 1) as u64;
    min + (seed % range) as i64
}

/// 休眠（毫秒）- 同步版本，在异步环境中应使用 tokio::time::sleep
pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

/// 打印日志（供脚本调试使用）
pub fn log(message: &str) {
    tracing::info!("[Script] {}", message);
}

/// 打印警告日志
pub fn warn(message: &str) {
    tracing::warn!("[Script] {}", message);
}

/// 打印错误日志
pub fn error(message: &str) {
    tracing::error!("[Script] {}", message);
}
