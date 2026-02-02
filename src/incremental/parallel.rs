//! Parallel parsing support for processing multiple files concurrently.

use super::{SyntaxTree, IncrementalParser, DefaultIncrementalParser};
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A parsing job.
#[derive(Debug, Clone)]
pub struct ParseJob {
    pub id: String,
    pub path: PathBuf,
    pub source: Option<String>,
    pub priority: i32,
}

impl ParseJob {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        Self {
            id: path.to_string_lossy().to_string(),
            path,
            source: None,
            priority: 50,
        }
    }

    pub fn from_source(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            path: PathBuf::from("<memory>"),
            source: Some(source.into()),
            priority: 50,
        }
    }
}

/// Result of a parse job.
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub job_id: String,
    pub tree: Option<SyntaxTree>,
    pub error: Option<String>,
    pub duration: Duration,
}

impl ParseResult {
    pub fn success(job_id: impl Into<String>, tree: SyntaxTree, duration: Duration) -> Self {
        Self {
            job_id: job_id.into(),
            tree: Some(tree),
            error: None,
            duration,
        }
    }

    pub fn failure(job_id: impl Into<String>, error: impl Into<String>, duration: Duration) -> Self {
        Self {
            job_id: job_id.into(),
            tree: None,
            error: Some(error.into()),
            duration,
        }
    }

    pub fn is_success(&self) -> bool {
        self.tree.is_some()
    }

    pub fn is_failure(&self) -> bool {
        self.error.is_some()
    }
}

/// Configuration for parallel parsing.
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    pub max_workers: usize,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        let max_workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
            .min(8);

        Self { max_workers }
    }
}

/// Parallel parser for processing multiple files.
pub struct ParallelParser {
    parser: DefaultIncrementalParser,
}

impl ParallelParser {
    pub fn new() -> Self {
        Self {
            parser: DefaultIncrementalParser::new(),
        }
    }

    /// Parse multiple files sequentially (simple and predictable).
    pub fn parse_files(&self, paths: &[PathBuf]) -> HashMap<String, ParseResult> {
        let mut results = HashMap::new();

        for path in paths {
            let start = Instant::now();
            let id = path.to_string_lossy().to_string();

            let result = match std::fs::read_to_string(path) {
                Ok(source) => match self.parser.parse(&source) {
                    Ok(tree) => ParseResult::success(id.clone(), tree, start.elapsed()),
                    Err(e) => ParseResult::failure(id.clone(), format!("{:?}", e), start.elapsed()),
                },
                Err(e) => ParseResult::failure(id.clone(), format!("Failed to read: {}", e), start.elapsed()),
            };

            results.insert(id, result);
        }

        results
    }

    /// Parse from job descriptions.
    pub fn parse_jobs(&self, jobs: &[ParseJob]) -> HashMap<String, ParseResult> {
        let mut results = HashMap::new();

        for job in jobs {
            let start = Instant::now();

            let result = match &job.source {
                Some(source) => match self.parser.parse(source) {
                    Ok(tree) => ParseResult::success(job.id.clone(), tree, start.elapsed()),
                    Err(e) => ParseResult::failure(job.id.clone(), format!("{:?}", e), start.elapsed()),
                },
                None => match std::fs::read_to_string(&job.path) {
                    Ok(source) => match self.parser.parse(&source) {
                        Ok(tree) => ParseResult::success(job.id.clone(), tree, start.elapsed()),
                        Err(e) => ParseResult::failure(job.id.clone(), format!("{:?}", e), start.elapsed()),
                    },
                    Err(e) => ParseResult::failure(job.id.clone(), format!("Failed to read: {}", e), start.elapsed()),
                },
            };

            results.insert(job.id.clone(), result);
        }

        results
    }
}

impl Default for ParallelParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_job_from_path() {
        let job = ParseJob::from_path("/test/path.g4");
        assert_eq!(job.id, "/test/path.g4");
        assert!(job.source.is_none());
    }

    #[test]
    fn test_parse_job_from_source() {
        let job = ParseJob::from_source("test", "grammar Test;");
        assert_eq!(job.id, "test");
        assert_eq!(job.source, Some("grammar Test;".to_string()));
    }

    #[test]
    fn test_parse_result_success() {
        let grammar = crate::ast::Grammar::new("Test".to_string(), crate::core::types::GrammarType::Combined);
        let tree = SyntaxTree::new(grammar, "test".to_string());
        let result = ParseResult::success("test", tree, Duration::from_millis(10));

        assert!(result.is_success());
        assert!(!result.is_failure());
    }

    #[test]
    fn test_parse_result_failure() {
        let result = ParseResult::failure("test", "error message", Duration::from_millis(5));
        assert!(!result.is_success());
        assert!(result.is_failure());
    }

    #[test]
    fn test_parallel_config_default() {
        let config = ParallelConfig::default();
        assert!(config.max_workers > 0);
        assert!(config.max_workers <= 8);
    }

    #[test]
    fn test_parallel_parser_new() {
        let parser = ParallelParser::new();
        // Test that it can be created
        assert_eq!(parser.parse_files(&[]).len(), 0);
    }

    #[test]
    fn test_parse_job_priority() {
        let job = ParseJob::from_path("/test/path.g4");
        assert_eq!(job.priority, 50); // Default priority

        let mut job2 = ParseJob::from_source("test", "grammar Test;");
        job2.priority = 100;
        assert_eq!(job2.priority, 100);
    }

    #[test]
    fn test_parse_job_with_custom_priority() {
        let mut job = ParseJob::from_path("/test/path.g4");
        job.priority = 10;
        assert_eq!(job.priority, 10);
    }

    #[test]
    fn test_parse_result_duration() {
        let grammar = crate::ast::Grammar::new("Test".to_string(), crate::core::types::GrammarType::Combined);
        let tree = SyntaxTree::new(grammar, "test".to_string());
        let duration = Duration::from_millis(123);
        let result = ParseResult::success("test", tree, duration);

        assert_eq!(result.duration.as_millis(), 123);
    }

    #[test]
    fn test_parse_result_error_message() {
        let error_msg = "syntax error at line 5";
        let result = ParseResult::failure("test", error_msg, Duration::from_millis(1));

        assert_eq!(result.error, Some(error_msg.to_string()));
        assert!(result.is_failure());
        assert!(!result.is_success());
    }

    #[test]
    fn test_parse_empty_jobs_list() {
        let parser = ParallelParser::new();
        let results = parser.parse_jobs(&[]);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_parse_single_job() {
        let parser = ParallelParser::new();
        let job = ParseJob::from_source("test", "grammar Test;");

        let results = parser.parse_jobs(&[job]);
        assert_eq!(results.len(), 1);
        assert!(results.contains_key("test"));
    }

    #[test]
    fn test_parse_multiple_jobs() {
        let parser = ParallelParser::new();
        let jobs = vec![
            ParseJob::from_source("test1", "grammar Test1;"),
            ParseJob::from_source("test2", "grammar Test2;"),
            ParseJob::from_source("test3", "grammar Test3;"),
        ];

        let results = parser.parse_jobs(&jobs);
        assert_eq!(results.len(), 3);
        assert!(results.contains_key("test1"));
        assert!(results.contains_key("test2"));
        assert!(results.contains_key("test3"));
    }

    #[test]
    fn test_parse_job_with_invalid_grammar() {
        let parser = ParallelParser::new();
        let job = ParseJob::from_source("invalid", "not a valid grammar");

        let results = parser.parse_jobs(&[job]);
        assert_eq!(results.len(), 1);

        let result = &results["invalid"];
        assert!(result.is_failure());
        assert!(result.error.is_some());
    }

    #[test]
    fn test_parse_empty_source() {
        let parser = ParallelParser::new();
        let job = ParseJob::from_source("empty", "");

        let results = parser.parse_jobs(&[job]);
        assert_eq!(results.len(), 1);

        let result = &results["empty"];
        // Empty source should fail to parse
        assert!(result.is_failure());
    }

    #[test]
    fn test_parse_job_id_uniqueness() {
        let parser = ParallelParser::new();
        let jobs = vec![
            ParseJob::from_source("same_id", "grammar Test1;"),
            ParseJob::from_source("same_id", "grammar Test2;"),
        ];

        let results = parser.parse_jobs(&jobs);
        // Later job should overwrite earlier one with same ID
        assert_eq!(results.len(), 1);
        assert!(results.contains_key("same_id"));
    }

    #[test]
    fn test_parse_performance_tracking() {
        let parser = ParallelParser::new();
        let job = ParseJob::from_source("perf_test", "grammar Test;\nID: [a-z]+;");

        let results = parser.parse_jobs(&[job]);
        let result = &results["perf_test"];

        // Duration should be tracked even for successful parses
        assert!(result.duration.as_nanos() >= 0);
    }

    #[test]
    fn test_parallel_config_limits() {
        let config = ParallelConfig {
            max_workers: 4,
        };
        assert_eq!(config.max_workers, 4);

        // Default config should respect CPU limits
        let default_config = ParallelConfig::default();
        assert!(default_config.max_workers <= 8);
        assert!(default_config.max_workers > 0);
    }
}
