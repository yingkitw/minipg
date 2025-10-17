//! Benchmarks for code generation performance.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use minipg_ast::{Alternative, Element, Grammar, Rule, RuleType};
use minipg_codegen::{RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator};
use minipg_core::{types::{CodeGenConfig, GrammarType}, CodeGenerator};

fn create_simple_grammar() -> Grammar {
    let mut grammar = Grammar::new("Simple".to_string(), GrammarType::Combined);
    
    // Add lexer rules
    let mut number = Rule::new("NUMBER".to_string(), RuleType::Lexer);
    let mut alt = Alternative::new();
    alt.add_element(Element::OneOrMore {
        element: Box::new(Element::CharRange { start: '0', end: '9' }),
    });
    number.add_alternative(alt);
    grammar.add_rule(number);
    
    // Add parser rule
    let mut expr = Rule::new("expr".to_string(), RuleType::Parser);
    let mut alt = Alternative::new();
    alt.add_element(Element::RuleRef { name: "NUMBER".to_string(), label: None });
    expr.add_alternative(alt);
    grammar.add_rule(expr);
    
    grammar
}

fn create_complex_grammar() -> Grammar {
    let mut grammar = Grammar::new("Complex".to_string(), GrammarType::Combined);
    
    // Add multiple lexer rules
    for i in 0..10 {
        let mut rule = Rule::new(format!("TOKEN{}", i), RuleType::Lexer);
        let mut alt = Alternative::new();
        alt.add_element(Element::CharRange { start: 'a', end: 'z' });
        rule.add_alternative(alt);
        grammar.add_rule(rule);
    }
    
    // Add multiple parser rules
    for i in 0..20 {
        let mut rule = Rule::new(format!("rule{}", i), RuleType::Parser);
        let mut alt = Alternative::new();
        alt.add_element(Element::RuleRef { 
            name: format!("TOKEN{}", i % 10), 
            label: None 
        });
        rule.add_alternative(alt);
        grammar.add_rule(rule);
    }
    
    grammar
}

fn bench_rust_codegen(c: &mut Criterion) {
    let simple = create_simple_grammar();
    let complex = create_complex_grammar();
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    
    let mut group = c.benchmark_group("rust_codegen");
    
    group.bench_function("simple_grammar", |b| {
        b.iter(|| {
            generator.generate(black_box(&simple), black_box(&config))
        });
    });
    
    group.bench_function("complex_grammar", |b| {
        b.iter(|| {
            generator.generate(black_box(&complex), black_box(&config))
        });
    });
    
    group.finish();
}

fn bench_python_codegen(c: &mut Criterion) {
    let simple = create_simple_grammar();
    let complex = create_complex_grammar();
    let config = CodeGenConfig::default();
    let generator = PythonCodeGenerator::new();
    
    let mut group = c.benchmark_group("python_codegen");
    
    group.bench_function("simple_grammar", |b| {
        b.iter(|| {
            generator.generate(black_box(&simple), black_box(&config))
        });
    });
    
    group.bench_function("complex_grammar", |b| {
        b.iter(|| {
            generator.generate(black_box(&complex), black_box(&config))
        });
    });
    
    group.finish();
}

fn bench_javascript_codegen(c: &mut Criterion) {
    let simple = create_simple_grammar();
    let config = CodeGenConfig::default();
    let generator = JavaScriptCodeGenerator::new();
    
    c.bench_function("javascript_codegen_simple", |b| {
        b.iter(|| {
            generator.generate(black_box(&simple), black_box(&config))
        });
    });
}

fn bench_typescript_codegen(c: &mut Criterion) {
    let simple = create_simple_grammar();
    let config = CodeGenConfig::default();
    let generator = TypeScriptCodeGenerator::new();
    
    c.bench_function("typescript_codegen_simple", |b| {
        b.iter(|| {
            generator.generate(black_box(&simple), black_box(&config))
        });
    });
}

fn bench_all_languages(c: &mut Criterion) {
    let grammar = create_simple_grammar();
    let config = CodeGenConfig::default();
    
    let mut group = c.benchmark_group("all_languages");
    
    group.bench_function("rust", |b| {
        let generator = RustCodeGenerator::new();
        b.iter(|| generator.generate(black_box(&grammar), black_box(&config)));
    });
    
    group.bench_function("python", |b| {
        let generator = PythonCodeGenerator::new();
        b.iter(|| generator.generate(black_box(&grammar), black_box(&config)));
    });
    
    group.bench_function("javascript", |b| {
        let generator = JavaScriptCodeGenerator::new();
        b.iter(|| generator.generate(black_box(&grammar), black_box(&config)));
    });
    
    group.bench_function("typescript", |b| {
        let generator = TypeScriptCodeGenerator::new();
        b.iter(|| generator.generate(black_box(&grammar), black_box(&config)));
    });
    
    group.finish();
}

fn bench_grammar_complexity(c: &mut Criterion) {
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    
    let mut group = c.benchmark_group("grammar_complexity");
    
    for size in [1, 5, 10, 20, 50].iter() {
        let mut grammar = Grammar::new(format!("Test{}", size), GrammarType::Combined);
        
        // Add rules
        for i in 0..*size {
            let mut rule = Rule::new(format!("rule{}", i), RuleType::Parser);
            let mut alt = Alternative::new();
            alt.add_element(Element::Terminal { 
                value: format!("TOKEN{}", i), 
                label: None 
            });
            rule.add_alternative(alt);
            grammar.add_rule(rule);
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                generator.generate(black_box(&grammar), black_box(&config))
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_rust_codegen,
    bench_python_codegen,
    bench_javascript_codegen,
    bench_typescript_codegen,
    bench_all_languages,
    bench_grammar_complexity
);
criterion_main!(benches);
