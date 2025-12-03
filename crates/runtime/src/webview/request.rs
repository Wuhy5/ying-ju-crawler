//! WebView 请求类型

use std::{collections::HashMap, time::Duration};

/// WebView 请求
///
/// 描述需要打开的 WebView 窗口配置
#[derive(Debug, Clone)]
pub struct WebViewRequest {
    /// 要加载的 URL
    pub url: String,

    /// 窗口标题
    pub title: Option<String>,

    /// 提示用户的说明文案
    pub tip: Option<String>,

    /// 自定义 User-Agent
    pub user_agent: Option<String>,

    /// 初始 Cookie（在加载前注入）
    pub initial_cookies: HashMap<String, String>,

    /// 初始请求头
    pub initial_headers: HashMap<String, String>,

    /// 超时时间
    pub timeout: Duration,

    /// 页面加载完成后注入的 JavaScript
    pub inject_script: Option<String>,

    /// 成功检测脚本
    /// WebView 会周期性执行此脚本，返回 true 时视为完成
    pub success_check: Option<String>,

    /// 检测间隔
    pub check_interval: Duration,

    /// 完成后执行的脚本（用于提取数据）
    pub finish_script: Option<String>,

    /// 需要提取的 Cookie 名称
    pub extract_cookies: Option<Vec<String>>,

    /// 窗口尺寸
    pub window_size: Option<(u32, u32)>,

    /// 是否允许重定向
    pub allow_redirects: bool,
}

impl Default for WebViewRequest {
    fn default() -> Self {
        Self {
            url: String::new(),
            title: None,
            tip: None,
            user_agent: None,
            initial_cookies: HashMap::new(),
            initial_headers: HashMap::new(),
            timeout: Duration::from_secs(120),
            inject_script: None,
            success_check: None,
            check_interval: Duration::from_millis(500),
            finish_script: None,
            extract_cookies: None,
            window_size: None,
            allow_redirects: true,
        }
    }
}

impl WebViewRequest {
    /// 创建新请求
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置提示文案
    pub fn with_tip(mut self, tip: impl Into<String>) -> Self {
        self.tip = Some(tip.into());
        self
    }

    /// 设置 User-Agent
    pub fn with_user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// 设置超时
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 设置成功检测脚本
    pub fn with_success_check(mut self, script: impl Into<String>) -> Self {
        self.success_check = Some(script.into());
        self
    }

    /// 设置检测间隔
    pub fn with_check_interval(mut self, interval: Duration) -> Self {
        self.check_interval = interval;
        self
    }

    /// 设置需要提取的 Cookie
    pub fn with_extract_cookies(mut self, cookies: Vec<String>) -> Self {
        self.extract_cookies = Some(cookies);
        self
    }

    /// 添加初始 Cookie
    pub fn with_cookie(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.initial_cookies.insert(name.into(), value.into());
        self
    }

    /// 设置注入脚本
    pub fn with_inject_script(mut self, script: impl Into<String>) -> Self {
        self.inject_script = Some(script.into());
        self
    }

    /// 设置完成脚本
    pub fn with_finish_script(mut self, script: impl Into<String>) -> Self {
        self.finish_script = Some(script.into());
        self
    }
}
