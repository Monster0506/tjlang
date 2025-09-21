//! Enhanced TJLang Lexer Test Runner
//! 
//! This program tests the lexer against organized test files in valid/invalid folders
//! with nested categories for different language features.

use std::fs;
use std::path::{Path, PathBuf};
use codespan::Files;
use tjlang_lexer::lex;

#[derive(Debug, Clone)]
struct TestResult {
    path: String,
    tokens: usize,
    diagnostics: usize,
    has_errors: bool,
    expected_errors: bool,
    passed: bool,
    category: String,
}

#[derive(Debug)]
struct TestSummary {
    total_files: usize,
    total_tokens: usize,
    total_diagnostics: usize,
    passed: usize,
    failed: usize,
    by_category: std::collections::HashMap<String, CategorySummary>,
}

#[derive(Debug)]
struct CategorySummary {
    files: usize,
    passed: usize,
    failed: usize,
    tokens: usize,
    diagnostics: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 TJLang Enhanced Test Runner\n");
    
    let test_dir = "tests";
    let mut results = Vec::new();
    
    // Test valid files
    println!("📁 Testing Valid Files");
    println!("{}", "=".repeat(50));
    let valid_results = test_directory(&test_dir, "valid", true)?;
    results.extend(valid_results);
    
    println!("\n📁 Testing Invalid Files");
    println!("{}", "=".repeat(50));
    let invalid_results = test_directory(&test_dir, "invalid", false)?;
    results.extend(invalid_results);
    
    // Generate summary
    let summary = generate_summary(&results);
    print_summary(&summary);
    
    if summary.failed == 0 {
        println!("\n🎉 All tests passed!");
        Ok(())
    } else {
        println!("\n💥 Some tests failed!");
        Err("Test failures".into())
    }
}

fn test_directory(base_dir: &str, category: &str, expect_valid: bool) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    let category_path = Path::new(base_dir).join(category);
    
    if !category_path.exists() {
        println!("⚠️  Category '{}' not found, skipping...", category);
        return Ok(results);
    }
    
    // Recursively find all .tj files
    let tj_files = find_tj_files(&category_path)?;
    
    for file_path in tj_files {
        let relative_path = file_path.strip_prefix(base_dir).unwrap_or(&file_path);
        let path_str = relative_path.to_string_lossy().replace('\\', "/");
        
        // Determine subcategory
        let subcategory = determine_subcategory(&path_str, category);
        
        println!("📝 Testing: {}", path_str);
        
        let content = fs::read_to_string(&file_path)?;
        let mut files = Files::new();
        let file_id = files.add(&path_str, &content);
        
        // Lex the file
        let (tokens, diagnostics) = lex(&content, file_id);
        
        // Determine if test passed
        let has_errors = tokens.iter().any(|t| matches!(t.kind, tjlang_lexer::TokenKind::Error));
        let expected_errors = !expect_valid;
        let passed = if expected_errors {
            !diagnostics.is_empty() || has_errors
        } else {
            !has_errors && diagnostics.is_empty()
        };
        
        let result = TestResult {
            path: path_str,
            tokens: tokens.len(),
            diagnostics: diagnostics.len(),
            has_errors,
            expected_errors,
            passed,
            category: subcategory,
        };
        
        // Display detailed results
        if !diagnostics.is_empty() {
            println!("  🚨 Diagnostics: {}", diagnostics.len());
            for diagnostic in diagnostics.iter() {
                println!("    - {}", diagnostic);
            }
        } else {
            println!("  ✅ No diagnostics");
        }
        
        println!("  📊 Tokens: {}, Status: {}", 
                tokens.len(), 
                if result.passed { "✅ PASS" } else { "❌ FAIL" });
        
        results.push(result);
    }
    
    Ok(results)
}

fn find_tj_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut tj_files = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Recursively search subdirectories
            let mut sub_files = find_tj_files(&path)?;
            tj_files.append(&mut sub_files);
        } else if path.extension().and_then(|s| s.to_str()) == Some("tj") {
            tj_files.push(path);
        }
    }
    
    tj_files.sort();
    Ok(tj_files)
}

fn determine_subcategory(path: &str, base_category: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() > 2 {
        format!("{}/{}", base_category, parts[2])
    } else {
        base_category.to_string()
    }
}

fn generate_summary(results: &[TestResult]) -> TestSummary {
    let mut by_category = std::collections::HashMap::new();
    
    for result in results {
        let category = by_category.entry(result.category.clone()).or_insert(CategorySummary {
            files: 0,
            passed: 0,
            failed: 0,
            tokens: 0,
            diagnostics: 0,
        });
        
        category.files += 1;
        category.tokens += result.tokens;
        category.diagnostics += result.diagnostics;
        
        if result.passed {
            category.passed += 1;
        } else {
            category.failed += 1;
        }
    }
    
    let total_files = results.len();
    let total_tokens = results.iter().map(|r| r.tokens).sum();
    let total_diagnostics = results.iter().map(|r| r.diagnostics).sum();
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.iter().filter(|r| !r.passed).count();
    
    TestSummary {
        total_files,
        total_tokens,
        total_diagnostics,
        passed,
        failed,
        by_category,
    }
}

fn print_summary(summary: &TestSummary) {
    println!("\n📊 Test Summary");
    println!("{}", "=".repeat(50));
    println!("Total files: {}", summary.total_files);
    println!("Total tokens: {}", summary.total_tokens);
    println!("Total diagnostics: {}", summary.total_diagnostics);
    println!("Passed: {}", summary.passed);
    println!("Failed: {}", summary.failed);
    
    println!("\n📁 Results by Category");
    println!("{}", "=".repeat(50));
    
    let mut categories: Vec<_> = summary.by_category.iter().collect();
    categories.sort_by_key(|(name, _)| *name);
    
    for (category, stats) in categories {
        let status = if stats.failed == 0 { "✅" } else { "❌" };
        println!("{} {}: {}/{} passed ({} tokens, {} diagnostics)", 
                status, category, stats.passed, stats.files, stats.tokens, stats.diagnostics);
    }
}
