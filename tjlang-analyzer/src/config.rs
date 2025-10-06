//! Rule configuration system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the analysis pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Enabled rules
    pub enabled_rules: HashMap<String, bool>,

    /// Rule-specific settings
    pub rule_settings: HashMap<String, RuleSettings>,

    /// Global analysis settings
    pub global_settings: GlobalSettings,
}

/// Global analysis settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    /// Maximum number of diagnostics to report
    pub max_diagnostics: Option<usize>,

    /// Stop analysis on first error
    pub stop_on_error: bool,

    /// Enable parallel rule execution
    pub parallel_execution: bool,

    /// Analysis timeout in seconds
    pub timeout_seconds: Option<u64>,
}

/// Settings for individual rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSettings {
    /// Rule severity level
    pub severity: RuleSeverity,

    /// Rule-specific configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Severity levels for rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleSeverity {
    Error,
    Warning,
    Info,
    Disabled,
}

/// Naming convention configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingConfig {
    pub variable_case: CaseStyle,
    pub function_case: CaseStyle,
    pub type_case: CaseStyle,
    pub constant_case: CaseStyle,
    pub min_length: usize,
    pub max_length: usize,
}

/// Case style for naming conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaseStyle {
    SnakeCase,
    CamelCase,
    ScreamingCamelCase,
    PascalCase,
    ScreamingSnakeCase,
    KebabCase,
}

/// Complexity analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityConfig {
    pub max_function_length: usize,
    pub max_cyclomatic_complexity: usize,
    pub max_parameter_count: usize,
    pub max_nesting_depth: usize,
}

/// Formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingConfig {
    pub indent_size: usize,
    pub use_tabs: bool,
    pub max_line_length: usize,
    pub trailing_whitespace: bool,
    pub final_newline: bool,
}

/// Dead code detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadCodeConfig {
    pub detect_unused_variables: bool,
    pub detect_unused_parameters: bool,
    pub detect_unreachable_code: bool,
    pub detect_empty_functions: bool,
}

/// Performance analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub detect_inefficient_loops: bool,
    pub detect_memory_allocations: bool,
    pub detect_string_concatenation: bool,
    pub max_recursion_depth: usize,
}

/// Security analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub detect_null_pointers: bool,
    pub detect_buffer_overflows: bool,
    pub detect_unsafe_operations: bool,
    pub detect_hardcoded_credentials: bool,
}

impl Default for RuleConfig {
    fn default() -> Self {
        let mut enabled_rules = HashMap::new();
        let mut rule_settings = HashMap::new();

        // Enable only lenient rules by default (runtime error prevention)
        let critical_rules = vec![
            // Critical runtime error prevention rules
            "LiteralIndexBoundsRule",
            "LiteralDivisionByZeroRule", 
            "UndefinedVariableRule",
            "UndefinedFunctionRule",
            "ParameterTypeValidationRule",
            
            // Granular module validation rules (prevent runtime errors)
            "ModuleEmptyNameRule",
            "ModuleInvalidCharactersRule", 
            "ModuleReservedNameRule",
            
            // Granular type checking rules (prevent runtime errors)
            "VariableTypeCheckRule",
            "FunctionTypeCheckRule",
            "ExpressionTypeCheckRule",
            "MemberAccessTypeCheckRule",
        ];

        for rule in critical_rules {
            enabled_rules.insert(rule.to_string(), true);
            
            // Set appropriate severity levels (all lenient rules are critical)
            let severity = RuleSeverity::Error;
            
            // Set rule-specific configurations (none needed for lenient rules)
            let config = HashMap::new();
            
            rule_settings.insert(
                rule.to_string(),
                RuleSettings {
                    severity,
                    config,
                },
            );
        }

        Self {
            enabled_rules,
            rule_settings,
            global_settings: GlobalSettings::default(),
        }
    }
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            max_diagnostics: Some(1000),
            stop_on_error: false,
            parallel_execution: true,
            timeout_seconds: Some(30),
        }
    }
}

impl RuleConfig {
    /// Check if a rule is enabled
    pub fn is_rule_enabled(&self, rule_name: &str) -> bool {
        self.enabled_rules.get(rule_name).copied().unwrap_or(false)
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, rule_name: &str) {
        self.enabled_rules.insert(rule_name.to_string(), true);
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, rule_name: &str) {
        self.enabled_rules.insert(rule_name.to_string(), false);
    }

    /// Get rule settings
    pub fn get_rule_settings(&self, rule_name: &str) -> Option<&RuleSettings> {
        self.rule_settings.get(rule_name)
    }

    /// Set rule severity
    pub fn set_rule_severity(&mut self, rule_name: &str, severity: RuleSeverity) {
        if let Some(settings) = self.rule_settings.get_mut(rule_name) {
            settings.severity = severity;
        } else {
            self.rule_settings.insert(
                rule_name.to_string(),
                RuleSettings {
                    severity,
                    config: HashMap::new(),
                },
            );
        }
    }

    /// Get rule configuration value
    pub fn get_config_value<T>(&self, rule_name: &str, key: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let value = self.rule_settings.get(rule_name)?.config.get(key)?.clone();
        serde_json::from_value(value).ok()
    }

    /// Set rule configuration value
    pub fn set_config_value<T>(&mut self, rule_name: &str, key: &str, value: T)
    where
        T: Serialize,
    {
        let settings = self
            .rule_settings
            .entry(rule_name.to_string())
            .or_insert_with(|| RuleSettings {
                severity: RuleSeverity::Warning,
                config: HashMap::new(),
            });

        if let Ok(json_value) = serde_json::to_value(value) {
            settings.config.insert(key.to_string(), json_value);
        }
    }
}
