//! Comprehensive tests against all grammars from grammars-v4 repository.
//!
//! This test suite validates minipg's compatibility with real-world ANTLR4 grammars
//! from the official grammars-v4 repository: https://github.com/antlr/grammars-v4
//!
//! The tests can run in two modes:
//! 1. Online mode: Clones/downloads grammars-v4 repository and tests all grammars
//! 2. Offline mode: Tests against locally cached grammars (if available)
//!
//! To run these tests:
//!   cargo test --test test_grammars_v4_all -- --nocapture
//!
//! To skip downloading (use cached grammars):
//!   GRAMMARS_V4_SKIP_DOWNLOAD=1 cargo test --test test_grammars_v4_all

use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const GRAMMARS_V4_REPO: &str = "https://github.com/antlr/grammars-v4.git";
const GRAMMARS_V4_DIR: &str = "grammars-v4-cache";
const GRAMMARS_V2_DIR: &str = "grammars-v2-cache";


/// Get the path to the grammars-v4 repository (cloned or cached)
fn get_grammars_v4_path() -> PathBuf {
    PathBuf::from(GRAMMARS_V4_DIR)
}

/// Get the path to the grammars-v2 repository (cloned or cached)
fn get_grammars_v2_path() -> PathBuf {
    PathBuf::from(GRAMMARS_V2_DIR)
}

/// Clone or update the grammars-v4 repository
fn ensure_grammars_v4_repo() -> Result<PathBuf, String> {
    let path = get_grammars_v4_path();
    
    // Check if we should skip download
    if std::env::var("GRAMMARS_V4_SKIP_DOWNLOAD").is_ok() {
        if path.exists() {
            return Ok(path);
        } else {
            return Err("GRAMMARS_V4_SKIP_DOWNLOAD is set but no cached grammars found".to_string());
        }
    }
    
    // Clone if doesn't exist
    if !path.exists() {
        println!("Cloning grammars-v4 repository...");
        let output = Command::new("git")
            .args(&["clone", "--depth", "1", GRAMMARS_V4_REPO, GRAMMARS_V4_DIR])
            .output()
            .map_err(|e| format!("Failed to run git: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to clone grammars-v4: {}", stderr));
        }
        println!("Successfully cloned grammars-v4 repository");
    } else {
        // Update if exists
        println!("Updating grammars-v4 repository...");
        let output = Command::new("git")
            .args(&["-C", GRAMMARS_V4_DIR, "pull"])
            .output()
            .map_err(|e| format!("Failed to run git: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Warning: Failed to update grammars-v4: {}", stderr);
        }
    }
    
    Ok(path)
}

/// Find all .g4 files in a directory recursively
fn find_g4_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    
    if !dir.exists() {
        return Ok(files);
    }
    
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip .git directory
            if path.file_name().and_then(|n| n.to_str()) == Some(".git") {
                continue;
            }
            // Recursively search subdirectories
            files.extend(find_g4_files(&path)?);
        } else if path.extension().and_then(|e| e.to_str()) == Some("g4") {
            files.push(path);
        }
    }
    
    files.sort();
    Ok(files)
}

/// Test parsing a single grammar file
fn test_grammar_file(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    
    let parser = GrammarParser::new();
    parser.parse_string(&content, path.file_name().unwrap().to_str().unwrap())
        .map_err(|e| format!("Parse error: {:?}", e))?;
    
    Ok(())
}

/// Test statistics
#[derive(Default)]
struct TestStats {
    total: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
    errors: Vec<(String, String)>,
}

impl TestStats {
    fn print_summary(&self) {
        println!("\n{}", "=".repeat(80));
        println!("GRAMMARS-V4 TEST SUMMARY");
        println!("{}", "=".repeat(80));
        println!("Total grammars tested: {}", self.total);
        println!("Passed: {} ({:.1}%)", self.passed, 
                 (self.passed as f64 / self.total as f64 * 100.0));
        println!("Failed: {} ({:.1}%)", self.failed,
                 (self.failed as f64 / self.total as f64 * 100.0));
        println!("Skipped: {}", self.skipped);
        
        if !self.errors.is_empty() {
            println!("\nFailed grammars:");
            for (grammar, error) in &self.errors {
                println!("  - {}: {}", grammar, error);
            }
        }
        println!("{}", "=".repeat(80));
    }
}

/// Test all grammars from grammars-v4 repository
#[test]
#[ignore] // Ignore by default - requires git and network access. Run with: cargo test --test test_grammars_v4_all test_all_grammars_v4 -- --ignored
fn test_all_grammars_v4() {
    let repo_path = match ensure_grammars_v4_repo() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping test: {}", e);
            eprintln!("Set GRAMMARS_V4_SKIP_DOWNLOAD=1 to use cached grammars");
            return;
        }
    };
    
    println!("Finding all .g4 files in {}...", repo_path.display());
    let g4_files = match find_g4_files(&repo_path) {
        Ok(files) => files,
        Err(e) => {
            panic!("Failed to find .g4 files: {}", e);
        }
    };
    
    println!("Found {} grammar files", g4_files.len());
    
    let mut stats = TestStats::default();
    stats.total = g4_files.len();
    
    for (idx, file_path) in g4_files.iter().enumerate() {
        let relative_path = file_path.strip_prefix(&repo_path)
            .unwrap_or(file_path)
            .display()
            .to_string();
        
        print!("[{}/{}] Testing {}... ", idx + 1, stats.total, relative_path);
        
        match test_grammar_file(file_path) {
            Ok(()) => {
                println!("✓");
                stats.passed += 1;
            }
            Err(e) => {
                println!("✗");
                stats.failed += 1;
                stats.errors.push((relative_path, e));
            }
        }
    }
    
    stats.print_summary();
    
    // Fail the test if too many grammars failed
    let failure_rate = stats.failed as f64 / stats.total as f64;
    if failure_rate > 0.1 {
        panic!("Too many grammars failed ({:.1}% failure rate)", failure_rate * 100.0);
    }
}

/// Test a subset of popular grammars (faster, for CI)
#[test]
#[ignore]
fn test_popular_grammars_v4() {
    let popular_grammars = vec![
        "java/java9/Java9.g4",
        "python/python3/Python3.g4",
        "cpp/cpp14/CPP14.g4",
        "csharp/csharp/CSharp.g4",
        "javascript/javascript/JavaScriptParser.g4",
        "typescript/typescript/TypeScriptParser.g4",
        "sql/tsql/TSqlParser.g4",
        "graphql/graphql/GraphQL.g4",
        "json/json/JSON.g4",
        "xml/xml/XMLParser.g4",
    ];
    
    let repo_path = match ensure_grammars_v4_repo() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping test: {}", e);
            return;
        }
    };
    
    let mut stats = TestStats::default();
    stats.total = popular_grammars.len();
    
    println!("Testing {} popular grammars...", stats.total);
    
    for grammar_path in popular_grammars {
        let full_path = repo_path.join(grammar_path);
        
        if !full_path.exists() {
            println!("Skipping {} (not found)", grammar_path);
            stats.skipped += 1;
            continue;
        }
        
        print!("Testing {}... ", grammar_path);
        
        match test_grammar_file(&full_path) {
            Ok(()) => {
                println!("✓");
                stats.passed += 1;
            }
            Err(e) => {
                println!("✗");
                println!("  Error: {}", e);
                stats.failed += 1;
                stats.errors.push((grammar_path.to_string(), e));
            }
        }
    }
    
    stats.print_summary();
    
    if stats.failed > 0 {
        panic!("Some popular grammars failed");
    }
}

/// Test grammars by language category
#[test]
#[ignore]
fn test_grammars_by_category() {
    let categories = vec![
        ("Programming Languages", vec!["java", "python", "cpp", "c", "csharp", "javascript", "typescript", "go", "rust", "swift"]),
        ("Data Formats", vec!["json", "xml", "yaml", "csv", "toml"]),
        ("Query Languages", vec!["sql", "sparql", "cypher", "graphql"]),
        ("Markup", vec!["html", "markdown", "css"]),
        ("Configuration", vec!["ini", "properties", "config"]),
    ];
    
    let repo_path = match ensure_grammars_v4_repo() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping test: {}", e);
            return;
        }
    };
    
    let mut overall_stats = TestStats::default();
    
    for (category_name, languages) in categories {
        println!("\n{}", "=".repeat(80));
        println!("Testing category: {}", category_name);
        println!("{}", "=".repeat(80));
        
        let mut category_stats = TestStats::default();
        
        for language in languages {
            let lang_dir = repo_path.join(language);
            if !lang_dir.exists() {
                continue;
            }
            
            let g4_files = match find_g4_files(&lang_dir) {
                Ok(files) => files,
                Err(_) => continue,
            };
            
            category_stats.total += g4_files.len();
            overall_stats.total += g4_files.len();
            
            for file_path in g4_files {
                let relative_path = file_path.strip_prefix(&repo_path)
                    .unwrap_or(&file_path)
                    .display()
                    .to_string();
                
                match test_grammar_file(&file_path) {
                    Ok(()) => {
                        category_stats.passed += 1;
                        overall_stats.passed += 1;
                    }
                    Err(e) => {
                        category_stats.failed += 1;
                        overall_stats.failed += 1;
                        category_stats.errors.push((relative_path, e));
                    }
                }
            }
        }
        
        println!("\nCategory results:");
        println!("  Total: {}", category_stats.total);
        println!("  Passed: {}", category_stats.passed);
        println!("  Failed: {}", category_stats.failed);
        
        if !category_stats.errors.is_empty() && category_stats.errors.len() <= 10 {
            println!("  Errors:");
            for (grammar, error) in &category_stats.errors {
                println!("    - {}: {}", grammar, error);
            }
        }
    }
    
    println!("\n{}", "=".repeat(80));
    println!("OVERALL RESULTS");
    println!("{}", "=".repeat(80));
    overall_stats.print_summary();
}

/// Test all grammars from grammars-v2-cache repository
/// 
/// Usage:
///   cargo test --test test_grammars_v4_all test_all_grammars_v2 -- --ignored
/// 
/// Or set GRAMMARS_CACHE_DIR environment variable to use a custom path:
///   GRAMMARS_CACHE_DIR=/path/to/grammars-v2-cache cargo test --test test_grammars_v4_all test_all_grammars_v2 -- --ignored
#[test]
#[ignore] // Ignore by default - requires cached grammars. Run with: cargo test --test test_grammars_v4_all test_all_grammars_v2 -- --ignored
fn test_all_grammars_v2() {
    // Check for environment variable override first
    let cache_path = if let Ok(dir) = std::env::var("GRAMMARS_CACHE_DIR") {
        PathBuf::from(dir)
    } else {
        get_grammars_v2_path()
    };
    
    if !cache_path.exists() {
        eprintln!("Skipping test: grammars-v2-cache directory not found at {}", cache_path.display());
        eprintln!("Please ensure grammars-v2-cache directory exists, or set GRAMMARS_CACHE_DIR environment variable");
        return;
    }
    
    println!("Finding all .g4 files in {}...", cache_path.display());
    let g4_files = match find_g4_files(&cache_path) {
        Ok(files) => files,
        Err(e) => {
            panic!("Failed to find .g4 files: {}", e);
        }
    };
    
    println!("Found {} grammar files", g4_files.len());
    
    if g4_files.is_empty() {
        eprintln!("No .g4 files found in {}", cache_path.display());
        return;
    }
    
    let mut stats = TestStats::default();
    stats.total = g4_files.len();
    
    for (idx, file_path) in g4_files.iter().enumerate() {
        let relative_path = file_path.strip_prefix(&cache_path)
            .unwrap_or(file_path)
            .display()
            .to_string();
        
        print!("[{}/{}] Testing {}... ", idx + 1, stats.total, relative_path);
        
        match test_grammar_file(file_path) {
            Ok(()) => {
                println!("✓");
                stats.passed += 1;
            }
            Err(e) => {
                println!("✗");
                stats.failed += 1;
                stats.errors.push((relative_path, e));
            }
        }
    }
    
    println!("\n{}", "=".repeat(80));
    println!("GRAMMARS-V2 TEST SUMMARY");
    println!("{}", "=".repeat(80));
    stats.print_summary();
    
    // Fail the test if too many grammars failed
    let failure_rate = stats.failed as f64 / stats.total as f64;
    if failure_rate > 0.1 {
        panic!("Too many grammars failed ({:.1}% failure rate)", failure_rate * 100.0);
    }
}

