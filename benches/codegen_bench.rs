use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minipg_ast::{Grammar, Rule};
use minipg_codegen::{JavaScriptCodeGenerator, PythonCodeGenerator, RustCodeGenerator};
use minipg_core::{types::{CodeGenConfig, GrammarType}, CodeGenerator};

fn create_grammar(rule_count: usize) -> Grammar {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    for i in 0..rule_count {
        grammar.add_rule(Rule::parser_rule(format!("rule{}", i)));
    }
    
    for i in 0..5 {
        grammar.add_rule(Rule::lexer_rule(format!("TOKEN{}", i)));
    }
    
    grammar
}

fn bench_rust_codegen(c: &mut Criterion) {
    let grammar = create_grammar(20);
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    
    c.bench_function("rust_codegen_20_rules", |b| {
        b.iter(|| {
            generator.generate(black_box(&grammar), black_box(&config)).unwrap()
        });
    });
}

fn bench_python_codegen(c: &mut Criterion) {
    let grammar = create_grammar(20);
    let generator = PythonCodeGenerator::new();
    let config = CodeGenConfig::default();
    
    c.bench_function("python_codegen_20_rules", |b| {
        b.iter(|| {
            generator.generate(black_box(&grammar), black_box(&config)).unwrap()
        });
    });
}

fn bench_javascript_codegen(c: &mut Criterion) {
    let grammar = create_grammar(20);
    let generator = JavaScriptCodeGenerator::new();
    let config = CodeGenConfig::default();
    
    c.bench_function("javascript_codegen_20_rules", |b| {
        b.iter(|| {
            generator.generate(black_box(&grammar), black_box(&config)).unwrap()
        });
    });
}

criterion_group!(
    benches,
    bench_rust_codegen,
    bench_python_codegen,
    bench_javascript_codegen
);
criterion_main!(benches);
