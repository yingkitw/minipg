/// Comprehensive performance benchmarks for minipg
/// 
/// Run with: cargo bench --bench performance_bench
/// 
/// These benchmarks measure:
/// - Parsing performance across different grammar sizes
/// - Code generation performance for all target languages
/// - Memory allocation patterns
/// - Scalability with grammar complexity

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;
use minipg::ast::{Grammar, Rule};
use minipg::codegen::{
    RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator,
    TypeScriptCodeGenerator, GoCodeGenerator, JavaCodeGenerator,
    CCodeGenerator, CppCodeGenerator
};
use minipg::core::{CodeGenerator, types::{CodeGenConfig, GrammarType}};

/// Generate a grammar with specified number of rules
fn create_grammar_with_rules(rule_count: usize) -> Grammar {
    let mut grammar = Grammar::new("PerfTest".to_string(), GrammarType::Parser);
    
    for i in 0..rule_count {
        grammar.add_rule(Rule::parser_rule(format!("rule{}", i)));
    }
    
    for i in 0..10 {
        grammar.add_rule(Rule::lexer_rule(format!("TOKEN{}", i)));
    }
    
    grammar
}

/// Generate a large grammar text for parsing benchmarks
fn generate_large_grammar_text(rule_count: usize) -> String {
    let mut grammar = String::from("grammar parser PerfTest;\n\n");
    
    for i in 0..rule_count {
        grammar.push_str(&format!("rule{}: TOKEN{} ;\n", i, i % 10));
    }
    
    for i in 0..10 {
        grammar.push_str(&format!("TOKEN{}: 'token{}' ;\n", i, i));
    }
    
    grammar
}

fn bench_parsing_varying_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_varying_size");
    let parser = GrammarParser::new();
    
    for rule_count in [10, 50, 100, 500, 1000].iter() {
        let grammar_text = generate_large_grammar_text(*rule_count);
        let size = grammar_text.len();
        
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(rule_count),
            rule_count,
            |b, _| {
                b.iter(|| {
                    parser.parse_string(black_box(&grammar_text), "perf.g4").unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn bench_codegen_all_languages(c: &mut Criterion) {
    let grammar = create_grammar_with_rules(100);
    let config = CodeGenConfig::default();
    
    let mut group = c.benchmark_group("codegen_all_languages");
    
    // Rust
    {
        let generator = RustCodeGenerator::new();
        group.bench_function("rust", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    // Python
    {
        let generator = PythonCodeGenerator::new();
        group.bench_function("python", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    // JavaScript
    {
        let generator = JavaScriptCodeGenerator::new();
        group.bench_function("javascript", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    // TypeScript
    {
        let generator = TypeScriptCodeGenerator::new();
        group.bench_function("typescript", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    // Go
    {
        let generator = GoCodeGenerator::new();
        group.bench_function("go", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    // Java
    {
        let generator = JavaCodeGenerator::new();
        group.bench_function("java", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    // C
    {
        let generator = CCodeGenerator::new();
        group.bench_function("c", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    // C++
    {
        let generator = CppCodeGenerator::new();
        group.bench_function("cpp", |b| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config)).unwrap()
            });
        });
    }
    
    group.finish();
}

fn bench_codegen_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("codegen_scaling");
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    
    for rule_count in [10, 50, 100, 500, 1000, 2000].iter() {
        let grammar = create_grammar_with_rules(*rule_count);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(rule_count),
            rule_count,
            |b, _| {
                b.iter(|| {
                    generator.generate(black_box(&grammar), black_box(&config)).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn bench_parse_complex_structures(c: &mut Criterion) {
    let parser = GrammarParser::new();
    
    let complex_grammar = r#"
        grammar parser Complex;
        
        expr: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | '(' expr ')' | '-' factor;
        
        stmt: expr ';' | ifStmt | whileStmt | block | returnStmt;
        ifStmt: 'if' '(' expr ')' stmt ('else' stmt)?;
        whileStmt: 'while' '(' expr ')' stmt;
        block: '{' stmt* '}';
        returnStmt: 'return' expr? ';';
        
        program: stmt* EOF;
        
        NUMBER: DIGIT+ ('.' DIGIT+)?;
        DIGIT: '0'..'9';
        WS: (' ' | '\t' | '\r' | '\n')+ -> skip;
    "#;
    
    c.bench_function("parse_complex_grammar", |b| {
        b.iter(|| {
            parser.parse_string(black_box(complex_grammar), "complex.g4").unwrap()
        });
    });
}

fn bench_parse_real_world_grammars(c: &mut Criterion) {
    let parser = GrammarParser::new();
    let mut group = c.benchmark_group("parse_real_world");
    
    let grammars = vec![
        ("calculator", include_str!("../examples/calculator.g4")),
        ("json", include_str!("../examples/json.g4")),
        ("sql", include_str!("../examples/SQL.g4")),
    ];
    
    for (name, grammar_text) in grammars {
        group.bench_function(name, |b| {
            b.iter(|| {
                parser.parse_string(black_box(grammar_text), &format!("{}.g4", name)).unwrap()
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parsing_varying_size,
    bench_codegen_all_languages,
    bench_codegen_scaling,
    bench_parse_complex_structures,
    bench_parse_real_world_grammars
);
criterion_main!(benches);

