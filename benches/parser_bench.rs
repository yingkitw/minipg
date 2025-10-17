use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use minipg_core::GrammarParser;
use minipg_parser::GrammarParser as Parser;

fn simple_grammar() -> &'static str {
    r#"
grammar parser Calculator;

expr: term;
term: factor;
factor: NUMBER;

NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
"#
}

fn complex_grammar() -> &'static str {
    r#"
grammar parser Complex;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';
stmt: expr ';' | block;
block: '{' stmt* '}';
program: stmt*;

NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
WS: SPACE+;
SPACE: ' ' | '\t' | '\r' | '\n';
"#
}

fn bench_parse_simple(c: &mut Criterion) {
    let parser = Parser::new();
    let source = simple_grammar();
    
    c.bench_function("parse_simple_grammar", |b| {
        b.iter(|| {
            parser.parse_string(black_box(source), "bench.g4").unwrap()
        });
    });
}

fn bench_parse_complex(c: &mut Criterion) {
    let parser = Parser::new();
    let source = complex_grammar();
    
    c.bench_function("parse_complex_grammar", |b| {
        b.iter(|| {
            parser.parse_string(black_box(source), "bench.g4").unwrap()
        });
    });
}

fn bench_parse_varying_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_varying_size");
    let parser = Parser::new();
    
    for rule_count in [5, 10, 20, 50].iter() {
        let mut grammar = String::from("grammar parser Test;\n\n");
        
        for i in 0..*rule_count {
            grammar.push_str(&format!("rule{}: 'x';\n", i));
        }
        
        group.bench_with_input(
            BenchmarkId::from_parameter(rule_count),
            rule_count,
            |b, _| {
                b.iter(|| {
                    parser.parse_string(black_box(&grammar), "bench.g4").unwrap()
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_simple,
    bench_parse_complex,
    bench_parse_varying_size
);
criterion_main!(benches);
