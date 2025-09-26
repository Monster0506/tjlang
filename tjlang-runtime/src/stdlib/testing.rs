//! TESTING Module - Testing framework
//!
//! Provides comprehensive testing functionality including:
//! - Test case creation and execution
//! - Assertions and expectations
//! - Test reporting
//! - Mock objects
//! - Test fixtures

use crate::values::Value;

/// TESTING module for testing operations
pub struct TESTING;

impl TESTING {
    /// Create a new test suite
    pub fn new_suite(name: &str) -> TestSuite {
        TestSuite::new(name)
    }
    
    /// Run a test
    pub fn run_test<F>(name: &str, test_fn: F) -> TestResult 
    where F: Fn() -> Result<(), String> {
        let start = std::time::Instant::now();
        match test_fn() {
            Ok(()) => TestResult::Passed {
                name: name.to_string(),
                duration: start.elapsed(),
            },
            Err(error) => TestResult::Failed {
                name: name.to_string(),
                error,
                duration: start.elapsed(),
            },
        }
    }
    
    /// Assert that a condition is true
    pub fn assert_true(condition: bool, message: &str) -> Result<(), String> {
        if condition {
            Ok(())
        } else {
            Err(format!("Assertion failed: {}", message))
        }
    }
    
    /// Assert that a condition is false
    pub fn assert_false(condition: bool, message: &str) -> Result<(), String> {
        if !condition {
            Ok(())
        } else {
            Err(format!("Assertion failed: {}", message))
        }
    }
    
    /// Assert that two values are equal
    pub fn assert_equal<T: PartialEq + std::fmt::Debug>(actual: &T, expected: &T, message: &str) -> Result<(), String> {
        if actual == expected {
            Ok(())
        } else {
            Err(format!("Assertion failed: {} - Expected {:?}, got {:?}", message, expected, actual))
        }
    }
    
    /// Assert that two values are not equal
    pub fn assert_not_equal<T: PartialEq + std::fmt::Debug>(actual: &T, expected: &T, message: &str) -> Result<(), String> {
        if actual != expected {
            Ok(())
        } else {
            Err(format!("Assertion failed: {} - Values should not be equal: {:?}", message, actual))
        }
    }
    
    /// Assert that a value is within a range
    pub fn assert_in_range<T: PartialOrd + std::fmt::Debug>(value: &T, min: &T, max: &T, message: &str) -> Result<(), String> {
        if value >= min && value <= max {
            Ok(())
        } else {
            Err(format!("Assertion failed: {} - Value {:?} not in range [{:?}, {:?}]", message, value, min, max))
        }
    }
    
    /// Assert that a string contains a substring
    pub fn assert_contains(actual: &str, expected: &str, message: &str) -> Result<(), String> {
        if actual.contains(expected) {
            Ok(())
        } else {
            Err(format!("Assertion failed: {} - String '{}' does not contain '{}'", message, actual, expected))
        }
    }
    
    /// Assert that a function panics
    pub fn assert_panics<F>(f: F, message: &str) -> Result<(), String> 
    where F: FnOnce() + std::panic::UnwindSafe {
        match std::panic::catch_unwind(f) {
            Ok(_) => Err(format!("Assertion failed: {} - Function should have panicked", message)),
            Err(_) => Ok(()),
        }
    }
}

/// Test suite for organizing tests
pub struct TestSuite {
    name: String,
    tests: Vec<Test>,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tests: Vec::new(),
        }
    }
    
    pub fn add_test<F>(&mut self, name: &str, test_fn: F) 
    where F: Fn() -> Result<(), String> + 'static {
        self.tests.push(Test {
            name: name.to_string(),
            test_fn: Box::new(test_fn),
        });
    }
    
    pub fn run(&self) -> TestSuiteResult {
        let mut results = Vec::new();
        let mut passed = 0;
        let mut failed = 0;
        
        for test in &self.tests {
            let result = TESTING::run_test(&test.name, &test.test_fn);
            match &result {
                TestResult::Passed { .. } => passed += 1,
                TestResult::Failed { .. } => failed += 1,
            }
            results.push(result);
        }
        
        TestSuiteResult {
            name: self.name.clone(),
            results,
            passed,
            failed,
        }
    }
}

/// Individual test
struct Test {
    name: String,
    test_fn: Box<dyn Fn() -> Result<(), String>>,
}

/// Test result
#[derive(Debug)]
pub enum TestResult {
    Passed {
        name: String,
        duration: std::time::Duration,
    },
    Failed {
        name: String,
        error: String,
        duration: std::time::Duration,
    },
}

/// Test suite result
#[derive(Debug)]
pub struct TestSuiteResult {
    pub name: String,
    pub results: Vec<TestResult>,
    pub passed: usize,
    pub failed: usize,
}

impl TestSuiteResult {
    pub fn total(&self) -> usize {
        self.passed + self.failed
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            self.passed as f64 / self.total() as f64
        }
    }
    
    pub fn print_summary(&self) {
        println!("Test Suite: {}", self.name);
        println!("Total: {}, Passed: {}, Failed: {}", self.total(), self.passed, self.failed);
        println!("Success Rate: {:.1}%", self.success_rate() * 100.0);
        
        for result in &self.results {
            match result {
                TestResult::Passed { name, duration } => {
                    println!("  ✓ {} ({:.2}ms)", name, duration.as_secs_f64() * 1000.0);
                },
                TestResult::Failed { name, error, duration } => {
                    println!("  ✗ {} ({:.2}ms) - {}", name, duration.as_secs_f64() * 1000.0, error);
                },
            }
        }
    }
}
