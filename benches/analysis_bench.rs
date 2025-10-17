use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minipg_analysis::SemanticAnalyzer;
use minipg_ast::{Alternative, Element, Grammar, Rule};
use minipg_core::{types::GrammarType, SemanticAnalyzer as SemanticAnalyzerTrait};

fn create_grammar(rule_count: usize) -> Grammar {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    for i in 0..rule_count {
        let mut rule = Rule::parser_rule(format!("rule{}", i));
        let mut alt = Alternative::new();
        
        if i > 0 {
            alt.add_element(Element::rule_ref(format!("rule{}", i - 1)));
        } else {
            alt.add_element(Element::string_literal("x".to_string()));
        }
        
        rule.add_alternative(alt);
        grammar.add_rule(rule);
    }
    
    grammar
}

fn bench_semantic_analysis(c: &mut Criterion) {
    let grammar = create_grammar(20);
    let analyzer = SemanticAnalyzer::new();
    
    c.bench_function("semantic_analysis_20_rules", |b| {
        b.iter(|| {
            analyzer.analyze(black_box(&grammar)).unwrap()
        });
    });
}

fn bench_reachability_analysis(c: &mut Criterion) {
    let grammar = create_grammar(50);
    let analyzer = SemanticAnalyzer::new();
    
    c.bench_function("reachability_analysis_50_rules", |b| {
        b.iter(|| {
            analyzer.analyze(black_box(&grammar)).unwrap()
        });
    });
}

criterion_group!(benches, bench_semantic_analysis, bench_reachability_analysis);
criterion_main!(benches);
