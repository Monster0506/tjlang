//! TJLang Pattern Matching Engine
//! 
//! Advanced pattern matching with destructuring, guards, and exhaustiveness checking.

use std::collections::HashMap;
use crate::values::Value;
use tjlang_ast::{Pattern, Expression, Type, SourceSpan};

/// Pattern matching result
#[derive(Debug, Clone)]
pub enum MatchResult {
    /// Pattern matched successfully
    Matched {
        bindings: HashMap<String, Value>,
        guard_passed: bool,
    },
    /// Pattern did not match
    NotMatched,
    /// Pattern matched but guard failed
    GuardFailed,
}

/// Pattern matching engine
pub struct PatternMatcher {
    /// Current bindings from pattern matching
    bindings: HashMap<String, Value>,
    
    /// Pattern compilation cache
    pattern_cache: HashMap<String, CompiledPattern>,
}

/// Compiled pattern for efficient matching
#[derive(Debug, Clone)]
struct CompiledPattern {
    pub pattern: Pattern,
    pub bindings: Vec<String>,
    pub guards: Vec<Expression>,
    pub exhaustiveness_checked: bool,
}

impl PatternMatcher {
    /// Create a new pattern matcher
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            pattern_cache: HashMap::new(),
        }
    }
    
    /// Match a value against a pattern
    pub fn match_pattern(&mut self, value: &Value, pattern: &Pattern) -> MatchResult {
        self.bindings.clear();
        
        match self.match_pattern_internal(value, pattern) {
            MatchResult::Matched { bindings, guard_passed } => {
                self.bindings = bindings;
                MatchResult::Matched { bindings: self.bindings.clone(), guard_passed }
            },
            result => result,
        }
    }
    
    /// Internal pattern matching logic
    fn match_pattern_internal(&mut self, value: &Value, pattern: &Pattern) -> MatchResult {
        match (value, pattern) {
            // Literal patterns
            (Value::Int(a), Pattern::Literal(tjlang_ast::Literal::Int(b))) => {
                if a == b {
                    MatchResult::Matched { bindings: HashMap::new(), guard_passed: true }
                } else {
                    MatchResult::NotMatched
                }
            },
            (Value::Float(a), Pattern::Literal(tjlang_ast::Literal::Float(b))) => {
                if (a - b).abs() < f64::EPSILON {
                    MatchResult::Matched { bindings: HashMap::new(), guard_passed: true }
                } else {
                    MatchResult::NotMatched
                }
            },
            (Value::Bool(a), Pattern::Literal(tjlang_ast::Literal::Bool(b))) => {
                if a == b {
                    MatchResult::Matched { bindings: HashMap::new(), guard_passed: true }
                } else {
                    MatchResult::NotMatched
                }
            },
            (Value::String(a), Pattern::Literal(tjlang_ast::Literal::String(b))) => {
                if a == b {
                    MatchResult::Matched { bindings: HashMap::new(), guard_passed: true }
                } else {
                    MatchResult::NotMatched
                }
            },
            (Value::None, Pattern::Literal(tjlang_ast::Literal::None)) => {
                MatchResult::Matched { bindings: HashMap::new(), guard_passed: true }
            },
            
            // Variable patterns
            (value, Pattern::Variable { name, .. }) => {
                let mut bindings = HashMap::new();
                bindings.insert(name.clone(), value.clone());
                MatchResult::Matched { bindings, guard_passed: true }
            },
            
            // Wildcard patterns
            (_, Pattern::Wildcard(_)) => {
                MatchResult::Matched { bindings: HashMap::new(), guard_passed: true }
            },
            
            // Constructor patterns
            (Value::Enum { name: enum_name, variant: variant_name, fields: value_fields }, 
             Pattern::Constructor { name: pattern_name, fields: pattern_fields, .. }) => {
                if enum_name == pattern_name && variant_name == pattern_name {
                    if value_fields.len() == pattern_fields.len() {
                        let mut bindings = HashMap::new();
                        for (i, pattern_field) in pattern_fields.iter().enumerate() {
                            if let Some(value_field) = value_fields.get(i) {
                                match self.match_pattern_internal(value_field, pattern_field) {
                                    MatchResult::Matched { bindings: field_bindings, guard_passed } => {
                                        if !guard_passed {
                                            return MatchResult::GuardFailed;
                                        }
                                        bindings.extend(field_bindings);
                                    },
                                    MatchResult::NotMatched => return MatchResult::NotMatched,
                                    MatchResult::GuardFailed => return MatchResult::GuardFailed,
                                }
                            } else {
                                return MatchResult::NotMatched;
                            }
                        }
                        MatchResult::Matched { bindings, guard_passed: true }
                    } else {
                        MatchResult::NotMatched
                    }
                } else {
                    MatchResult::NotMatched
                }
            },
            
            // Struct patterns
            (Value::Struct { name: struct_name, fields: value_fields }, 
             Pattern::Struct { name: pattern_name, fields: pattern_fields, .. }) => {
                if struct_name == pattern_name {
                    let mut bindings = HashMap::new();
                    for (field_name, pattern_field) in pattern_fields {
                        if let Some(value_field) = value_fields.get(field_name) {
                            match self.match_pattern_internal(value_field, pattern_field) {
                                MatchResult::Matched { bindings: field_bindings, guard_passed } => {
                                    if !guard_passed {
                                        return MatchResult::GuardFailed;
                                    }
                                    bindings.extend(field_bindings);
                                },
                                MatchResult::NotMatched => return MatchResult::NotMatched,
                                MatchResult::GuardFailed => return MatchResult::GuardFailed,
                            }
                        } else {
                            return MatchResult::NotMatched;
                        }
                    }
                    MatchResult::Matched { bindings, guard_passed: true }
                } else {
                    MatchResult::NotMatched
                }
            },
            
            // Tuple patterns
            (Value::Tuple(value_elements), Pattern::Tuple { patterns: pattern_elements, .. }) => {
                if value_elements.len() == pattern_elements.len() {
                    let mut bindings = HashMap::new();
                    for (value_elem, pattern_elem) in value_elements.iter().zip(pattern_elements.iter()) {
                        match self.match_pattern_internal(value_elem, pattern_elem) {
                            MatchResult::Matched { bindings: elem_bindings, guard_passed } => {
                                if !guard_passed {
                                    return MatchResult::GuardFailed;
                                }
                                bindings.extend(elem_bindings);
                            },
                            MatchResult::NotMatched => return MatchResult::NotMatched,
                            MatchResult::GuardFailed => return MatchResult::GuardFailed,
                        }
                    }
                    MatchResult::Matched { bindings, guard_passed: true }
                } else {
                    MatchResult::NotMatched
                }
            },
            
            // Trait check patterns
            (value, Pattern::TraitCheck { trait_name, .. }) => {
                // TODO: Implement trait checking
                // For now, always match
                MatchResult::Matched { bindings: HashMap::new(), guard_passed: true }
            },
            
            // Default case - no match
            _ => MatchResult::NotMatched,
        }
    }
    
    /// Check if a pattern is exhaustive for a given type
    pub fn check_exhaustiveness(&self, patterns: &[Pattern], target_type: &Type) -> bool {
        // TODO: Implement exhaustiveness checking
        // This is a complex algorithm that needs to analyze the type structure
        // and ensure all possible cases are covered
        true
    }
    
    /// Compile a pattern for efficient matching
    pub fn compile_pattern(&mut self, name: String, pattern: Pattern) -> CompiledPattern {
        let bindings = self.extract_bindings(&pattern);
        let guards = Vec::new(); // TODO: Extract guards from pattern
        
        CompiledPattern {
            pattern,
            bindings,
            guards,
            exhaustiveness_checked: false,
        }
    }
    
    /// Extract variable bindings from a pattern
    fn extract_bindings(&self, pattern: &Pattern) -> Vec<String> {
        match pattern {
            Pattern::Variable { name, .. } => vec![name.clone()],
            Pattern::Constructor { fields, .. } => {
                fields.iter().flat_map(|f| self.extract_bindings(f)).collect()
            },
            Pattern::Struct { fields, .. } => {
                fields.iter().flat_map(|(_, f)| self.extract_bindings(f)).collect()
            },
            Pattern::Tuple { patterns, .. } => {
                patterns.iter().flat_map(|p| self.extract_bindings(p)).collect()
            },
            Pattern::TraitCheck { .. } => Vec::new(),
            Pattern::Wildcard(_) => Vec::new(),
            Pattern::Literal(_) => Vec::new(),
        }
    }
    
    /// Get current bindings
    pub fn get_bindings(&self) -> &HashMap<String, Value> {
        &self.bindings
    }
    
    /// Clear bindings
    pub fn clear_bindings(&mut self) {
        self.bindings.clear();
    }
    
    /// Match multiple patterns in sequence
    pub fn match_sequence(&mut self, value: &Value, patterns: &[Pattern]) -> Vec<MatchResult> {
        patterns.iter().map(|pattern| self.match_pattern(value, pattern)).collect()
    }
    
    /// Find the first matching pattern
    pub fn find_first_match(&mut self, value: &Value, patterns: &[Pattern]) -> Option<(usize, MatchResult)> {
        for (i, pattern) in patterns.iter().enumerate() {
            let result = self.match_pattern(value, pattern);
            match result {
                MatchResult::Matched { .. } => return Some((i, result)),
                _ => continue,
            }
        }
        None
    }
    
    /// Check if a value matches any of the patterns
    pub fn matches_any(&mut self, value: &Value, patterns: &[Pattern]) -> bool {
        patterns.iter().any(|pattern| {
            matches!(self.match_pattern(value, pattern), MatchResult::Matched { .. })
        })
    }
    
    /// Get pattern complexity (for optimization)
    pub fn get_pattern_complexity(&self, pattern: &Pattern) -> usize {
        match pattern {
            Pattern::Literal(_) => 1,
            Pattern::Variable { .. } => 1,
            Pattern::Wildcard(_) => 1,
            Pattern::Constructor { fields, .. } => 1 + fields.iter().map(|f| self.get_pattern_complexity(f)).sum::<usize>(),
            Pattern::Struct { fields, .. } => 1 + fields.iter().map(|(_, f)| self.get_pattern_complexity(f)).sum::<usize>(),
            Pattern::Tuple { patterns, .. } => 1 + patterns.iter().map(|p| self.get_pattern_complexity(p)).sum::<usize>(),
            Pattern::TraitCheck { .. } => 2,
        }
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}
