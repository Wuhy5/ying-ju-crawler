//! 规则验证模块
//!
//! 提供对 CrawlerRule 的完整验证，包括：
//! - 组件循环引用检测
//! - 未定义组件/脚本模块检测
//! - 模板语法验证
//! - 字段映射验证

use std::collections::{HashMap, HashSet};

use crate::{
    error::{CrawlerError, ValidationErrors, ValidationResult},
    schema::{Component, CrawlerRule, FlowTrait, Pipeline, Step, StepTrait},
};

/// 规则验证Trait
pub trait RuleValidate {
    /// 验证规则的完整性和正确性
    fn validate(&self) -> ValidationResult<()>;
}

impl RuleValidate for CrawlerRule {
    fn validate(&self) -> ValidationResult<()> {
        let mut validator = RuleValidator::new(self);
        validator.validate_all()
    }
}

/// 规则验证器
struct RuleValidator<'a> {
    rule: &'a CrawlerRule,
    errors: ValidationErrors,
    /// 已定义的组件名
    defined_components: HashSet<String>,
    /// 已定义的脚本模块名
    defined_script_modules: HashSet<String>,
}

impl<'a> RuleValidator<'a> {
    fn new(rule: &'a CrawlerRule) -> Self {
        let defined_components = rule
            .components
            .as_ref()
            .map(|c| c.keys().cloned().collect())
            .unwrap_or_default();

        let defined_script_modules = rule
            .scripting
            .as_ref()
            .map(|s| s.modules.keys().cloned().collect())
            .unwrap_or_default();

        Self {
            rule,
            errors: ValidationErrors::new(),
            defined_components,
            defined_script_modules,
        }
    }

    fn validate_all(&mut self) -> ValidationResult<()> {
        // 1. 验证组件循环引用
        self.validate_component_cycles();

        // 2. 验证所有流程
        if let Some(ref login_flow) = self.rule.login {
            self.validate_flow("login", login_flow.pipeline());
        }
        if let Some(ref list_flow) = self.rule.list {
            self.validate_flow("list", list_flow.pipeline());
        }
        self.validate_flow("detail", self.rule.detail.pipeline());
        self.validate_flow("search", self.rule.search.pipeline());

        // 3. 验证组件
        if let Some(ref components) = self.rule.components {
            for (name, component) in components {
                self.validate_component(name, component);
            }
        }

        self.errors.clone().into_result()
    }

    /// 验证组件循环引用
    fn validate_component_cycles(&mut self) {
        if let Some(ref components) = self.rule.components {
            for component_name in components.keys() {
                let mut visited = HashSet::new();
                let mut path = Vec::new();
                if self.detect_cycle(component_name, components, &mut visited, &mut path) {
                    self.errors.push(CrawlerError::CircularReference {
                        path: path.join(" -> "),
                    });
                }
            }
        }
    }

    /// 检测组件调用循环
    fn detect_cycle(
        &self,
        component_name: &str,
        components: &HashMap<String, Component>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if visited.contains(component_name) {
            path.push(component_name.to_string());
            return true;
        }

        visited.insert(component_name.to_string());
        path.push(component_name.to_string());

        if let Some(component) = components.get(component_name) {
            for step in &component.pipeline {
                if let Step::Call(call) = step
                    && self.detect_cycle(&call.component, components, visited, path)
                {
                    return true;
                }
            }
        }

        path.pop();
        visited.remove(component_name);
        false
    }

    /// 验证流程
    fn validate_flow(&mut self, flow_name: &str, pipeline: &Pipeline) {
        self.validate_pipeline(&format!("{}.pipeline", flow_name), pipeline);
    }

    /// 验证组件
    fn validate_component(&mut self, name: &str, component: &Component) {
        self.validate_pipeline(
            &format!("components.{}.pipeline", name),
            &component.pipeline,
        );
    }

    /// 验证管道
    fn validate_pipeline(&mut self, path: &str, pipeline: &Pipeline) {
        // 验证每个步骤的特定规则
        for (index, step) in pipeline.iter().enumerate() {
            self.validate_step(&format!("{}[{}]", path, index), step);
        }
    }

    /// 验证单个步骤
    fn validate_step(&mut self, path: &str, step: &Step) {
        match step {
            Step::Call(call) => {
                // 验证组件是否存在
                if !self.defined_components.contains(&call.component) {
                    self.errors.push(CrawlerError::UndefinedComponent {
                        component: call.component.clone(),
                    });
                }
            }
            Step::Script(script) => {
                // 验证脚本模块是否存在
                if let Some(module) = script.call.split('.').next()
                    && !self.defined_script_modules.contains(module)
                {
                    self.errors.push(CrawlerError::UndefinedScriptModule {
                        module: module.to_string(),
                    });
                }
            }
            Step::LoopForEach(loop_step) => {
                // 递归验证子管道
                self.validate_pipeline(&format!("{}.pipeline", path), &loop_step.pipeline);
            }
            _ => {
                // 其他步骤使用默认验证
                let mut step_errors = ValidationErrors::new();
                step.validate(&mut step_errors);
                self.errors.extend(step_errors);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::Meta,
        pipeline::StepCall,
        schema::{DetailFlow, MediaType, SearchFlow},
    };

    fn create_minimal_rule() -> CrawlerRule {
        CrawlerRule {
            meta: Meta {
                name: "Test".to_string(),
                author: "Test".to_string(),
                version: "1.0.0".to_string(),
                spec_version: "1.0.0".to_string(),
                domain: "example.com".to_string(),
                media_type: MediaType::Video,
                description: None,
                encoding: None,
                icon_url: None,
            },
            http: None,
            limits: None,
            scripting: None,
            components: None,
            login: None,
            list: None,
            detail: DetailFlow {
                description: None,
                pipeline: vec![],
            },
            search: SearchFlow {
                description: None,
                pagination: None,
                filters: None,
                pipeline: vec![],
            },
        }
    }

    #[test]
    fn test_validate_empty_rule() {
        let rule = create_minimal_rule();
        assert!(rule.validate().is_ok());
    }

    #[test]
    fn test_validate_undefined_component() {
        let mut rule = create_minimal_rule();
        rule.detail.pipeline.push(Step::Call(StepCall {
            component: "undefined_component".to_string(),
            with: None,
            output: "result".to_string(),
        }));

        let result = rule.validate();
        assert!(matches!(
            result,
            Err(CrawlerError::UndefinedComponent { .. })
        ));
    }

    #[test]
    fn test_validate_circular_reference() {
        let mut rule = create_minimal_rule();

        // 创建循环引用: A -> B -> A
        let mut components = HashMap::new();
        components.insert(
            "A".to_string(),
            Component {
                description: None,
                inputs: None,
                pipeline: vec![Step::Call(StepCall {
                    component: "B".to_string(),
                    with: None,
                    output: "result".to_string(),
                })],
            },
        );
        components.insert(
            "B".to_string(),
            Component {
                description: None,
                inputs: None,
                pipeline: vec![Step::Call(StepCall {
                    component: "A".to_string(),
                    with: None,
                    output: "result".to_string(),
                })],
            },
        );
        rule.components = Some(components);

        let result = rule.validate();
        assert!(result.is_err());
        // 验证错误中包含循环引用
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("循环引用") || matches!(e, CrawlerError::MultipleErrors { .. }),
                "Expected circular reference error, got: {}",
                error_msg
            );
        }
    }
}
