use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Media type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Video,
    Audio,
    Book,
    Manga,
}

/// HTTP method enumeration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

/// Response type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Html,
    Json,
    Xml,
    Text,
}

/// Selector extract type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExtractType {
    Text,
    Html,
    #[serde(rename = "attr:href")]
    AttrHref,
    #[serde(rename = "attr:src")]
    AttrSrc,
    #[serde(rename = "attr:data-src")]
    AttrDataSrc,
    #[serde(rename = "attr:data-url")]
    AttrDataUrl,
    #[serde(untagged)]
    Custom(String),
}

/// String operation type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StringOperation {
    Prepend,
    Append,
    Replace,
    Split,
    Trim,
    Template,
}

/// Loop operation type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LoopOperation {
    ForEach,
    While,
    Map,
}

/// Transform operation type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransformOperation {
    Map,
    Filter,
    Flatten,
    First,
    Last,
    Unique,
}

/// Crypto operation type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CryptoOperation {
    Encode,
    Decode,
    Hash,
    Encrypt,
    Decrypt,
}

/// Crypto algorithm
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CryptoAlgorithm {
    Base64,
    Hex,
    Md5,
    Sha1,
    Sha256,
    Aes,
    Rsa,
}

/// Cache backend type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CacheBackend {
    Memory,
    Sqlite,
}

/// Script engine type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ScriptEngine {
    Rhai,
    #[serde(rename = "javascript")]
    JavaScript,
    Python,
}

/// WebView action type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WebViewAction {
    Wait,
    Login,
}

/// WebView extract type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WebViewExtract {
    Cookies,
    Html,
    Storage,
}

/// Cast target type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CastType {
    String,
    Int,
    Float,
    Bool,
    Date,
    Array,
    Json,
}

/// Error action type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ErrorAction {
    Throw,
    Skip,
    Default,
}

/// Step type in pipeline
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Step {
    Selector {
        query: String,
        extract: ExtractType,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    Jsonpath {
        query: String,
    },
    Regex {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        group: Option<usize>,
    },
    Script {
        call: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        args: Option<serde_json::Value>,
    },
    String {
        operation: StringOperation,
        #[serde(skip_serializing_if = "Option::is_none")]
        prefix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        template: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delimiter: Option<String>,
    },
    HttpRequest {
        url_template: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        method: Option<HttpMethod>,
        #[serde(skip_serializing_if = "Option::is_none")]
        post_data_template: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_type: Option<ResponseType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<serde_json::Map<String, serde_json::Value>>,
    },
    Conditional {
        condition: String,
        if_true: Vec<Step>,
        #[serde(skip_serializing_if = "Option::is_none")]
        if_false: Option<Vec<Step>>,
    },
    Loop {
        operation: LoopOperation,
        #[serde(skip_serializing_if = "Option::is_none")]
        pipeline: Option<Vec<Step>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        condition: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_iterations: Option<usize>,
    },
    Transform {
        operation: TransformOperation,
        #[serde(skip_serializing_if = "Option::is_none")]
        pipeline: Option<Vec<Step>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        condition: Option<String>,
    },
    Validate {
        rule: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_action: Option<ErrorAction>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default_value: Option<serde_json::Value>,
    },
    Cast {
        to: CastType,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_action: Option<ErrorAction>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default_value: Option<serde_json::Value>,
    },
    Crypto {
        operation: CryptoOperation,
        #[serde(skip_serializing_if = "Option::is_none")]
        algorithm: Option<CryptoAlgorithm>,
        #[serde(skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        iv: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        padding: Option<String>,
    },
    Constant {
        value: serde_json::Value,
    },
    CacheKey {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<u32>,
    },
    CacheRetrieve {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
    },
    CacheClear {
        #[serde(skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
    },
    WebView {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        action: Option<WebViewAction>,
        #[serde(skip_serializing_if = "Option::is_none")]
        wait_for: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        extract: Option<WebViewExtract>,
    },
}

pub type Pipeline = Vec<Step>;

/// Metadata section
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Meta {
    pub name: String,
    pub author: String,
    pub version: String,
    pub spec_version: String,
    pub domain: String,
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_spec_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// HTTP configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HttpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_times: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_script: Option<CookieScript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interceptor: Option<Interceptor>,
}

/// Cookie script configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CookieScript {
    pub call: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl: Option<u32>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthConfig {
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        #[serde(skip_serializing_if = "Option::is_none")]
        token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_script: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_ttl: Option<u32>,
    },
}

/// HTTP interceptor configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Interceptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_response: Option<String>,
}

/// Scripting configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScriptingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<ScriptEngine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CacheConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<CacheBackend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_preset_scopes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<CacheScope>>,
}

/// Cache scope configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CacheScope {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Discovery entry
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DiscoverEntry {
    pub entry_url_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
}

/// Category definition
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Category {
    pub name: String,
    pub id: String,
}

/// Search entry
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchEntry {
    pub entry_url_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
}

/// Recommendation entry
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecommendationEntry {
    pub entry_url_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
}

/// Ranking entry with types and periods
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RankingEntry {
    pub entry_url_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<RankingType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods: Option<Vec<RankingPeriod>>,
}

/// Ranking type definition
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RankingType {
    pub name: String,
    pub id: String,
}

/// Ranking period definition
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RankingPeriod {
    pub name: String,
    pub id: String,
}

/// List parse rules
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListParse {
    pub item_selector: Step,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Detail parse rules
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DetailParse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Parse rules
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParseRules {
    pub list: ListParse,
    pub detail: DetailParse,
}

/// Root rule file structure
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RuleFile {
    pub meta: Meta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripting: Option<ScriptingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<CacheConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<DiscoverEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<SearchEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<RecommendationEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking: Option<RankingEntry>,
    pub parse: ParseRules,
}
