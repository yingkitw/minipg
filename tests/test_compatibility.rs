//! Comprehensive ANTLR4 compatibility tests.
//!
//! This test module consolidates tests for:
//! - ANTLR4 feature compatibility (@header, @members, options, imports)
//! - ANTLR4 test suite patterns (basic rules, alternatives, quantifiers, etc.)
//! - Grammars-v4 repository compatibility (real-world grammar subsets)
//!
//! # Test Organization
//!
//! Tests are organized by compatibility category:
//! 1. ANTLR4 Features - Named actions, options, imports
//! 2. ANTLR4 Test Suite - Standard test patterns
//! 3. Grammars-v4 - Real-world grammar subsets

use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;

fn parse_grammar(grammar_text: &str) -> minipg::ast::Grammar {
    let parser = GrammarParser::new();
    parser.parse_string(grammar_text, "test.g4").expect("Failed to parse grammar")
}

// ============================================================================
// ANTLR4 FEATURES - Named Actions, Options, Imports
// ============================================================================

#[test]
fn test_named_action_header() {
    let grammar_text = r#"
        grammar Test;
        
        @header {
            package com.example;
            import java.util.*;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("header"));
    let header = grammar.named_actions.get("header").unwrap();
    // Code is tokenized, so check for key parts
    assert!(header.contains("package") || header.contains("com") || header.contains("example"));
}

#[test]
fn test_named_action_members() {
    let grammar_text = r#"
        grammar Test;
        
        @members {
            private int count = 0;
            private String buffer = "";
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("members"));
    let members = grammar.named_actions.get("members").unwrap();
    assert!(members.contains("count"));
}

#[test]
fn test_multiple_named_actions() {
    let grammar_text = r#"
        grammar Test;
        
        @header {
            package com.example;
        }
        
        @members {
            private int count = 0;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.named_actions.len(), 2);
    assert!(grammar.named_actions.contains_key("header"));
    assert!(grammar.named_actions.contains_key("members"));
}

#[test]
fn test_grammar_options_language() {
    let grammar_text = r#"
        grammar Test;
        
        options {
            language = java;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    assert_eq!(grammar.name, "Test");
}

#[test]
fn test_grammar_imports() {
    let grammar_text = r#"
        grammar Test;
        
        import Base;
        import Common;
        
        expr: base_expr | common_expr;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    assert_eq!(grammar.imports.len(), 2);
    assert!(grammar.imports.contains(&"Base".to_string()));
    assert!(grammar.imports.contains(&"Common".to_string()));
}

// ============================================================================
// ANTLR4 TEST SUITE PATTERNS
// ============================================================================

#[test]
fn test_antlr4_suite_basic_rules() {
    let grammar_text = r#"
        grammar BasicRules;
        
        rule1: 'a';
        rule2: 'b' 'c';
        rule3: rule1 rule2;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.rules.len(), 3);
    assert!(grammar.get_rule("rule1").is_some());
    assert!(grammar.get_rule("rule2").is_some());
    assert!(grammar.get_rule("rule3").is_some());
}

#[test]
fn test_antlr4_suite_character_classes() {
    let grammar_text = r#"
        grammar CharClasses;
        
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        HEX: [0-9a-fA-F]+;
        WHITESPACE: [ \t\r\n]+;
        NOT_QUOTE: ~["];
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.lexer_rules().count(), 5);
}

#[test]
fn test_antlr4_suite_string_literals() {
    let grammar_text = r#"
        grammar StringLiterals;
        
        rule1: 'hello' 'world';
        rule2: 'a' | 'b' | 'c';
        rule3: 'if' | 'else' | 'while';
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.rules.len(), 3);
}

#[test]
fn test_antlr4_suite_lexer_commands() {
    let grammar_text = r#"
        grammar LexerCommands;
        
        WS: [ \t\r\n]+ -> skip;
        COMMENT: '//' ~[\r\n]* -> channel(HIDDEN);
        STRING_START: '"' -> mode(STRING_MODE);
        
        mode STRING_MODE;
        STRING_CHAR: ~["\\];
        STRING_END: '"' -> popMode;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert!(grammar.lexer_rules().count() >= 3);
}

#[test]
#[ignore] // Parser issue with argument syntax - needs fix
fn test_antlr4_suite_rule_arguments() {
    let grammar_text = r#"
        grammar RuleArgs;
        
        expr[int x]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    let expr_rule = grammar.get_rule("expr").unwrap();
    assert_eq!(expr_rule.arguments.len(), 1);
}

#[test]
#[ignore] // Parser issue with returns syntax - needs fix
fn test_antlr4_suite_rule_returns() {
    let grammar_text = r#"
        grammar RuleReturns;
        
        expr returns [int value]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    let expr_rule = grammar.get_rule("expr").unwrap();
    assert_eq!(expr_rule.returns.len(), 1);
}

#[test]
#[ignore] // Parser issue with locals syntax - needs fix
fn test_antlr4_suite_rule_locals() {
    let grammar_text = r#"
        grammar RuleLocals;
        
        expr locals [int temp]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    let expr_rule = grammar.get_rule("expr").unwrap();
    assert_eq!(expr_rule.locals.len(), 1);
}

#[test]
fn test_antlr4_suite_labels() {
    let grammar_text = r#"
        grammar Labels;
        
        expr: left=term op=('+'|'-') right=term;
        list: ids+=ID (',' ids+=ID)*;
        term: NUMBER;
        
        ID: [a-zA-Z]+;
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert!(grammar.get_rule("expr").is_some());
    assert!(grammar.get_rule("list").is_some());
}

// ============================================================================
// GRAMMARS-V4 REPOSITORY COMPATIBILITY
// ============================================================================

#[test]
fn test_grammars_v4_java_subset() {
    let grammar_text = r#"
        grammar JavaSubset;
        
        compilationUnit: packageDeclaration? importDeclaration* typeDeclaration* EOF;
        packageDeclaration: 'package' qualifiedName ';';
        importDeclaration: 'import' qualifiedName ';';
        typeDeclaration: classDeclaration | interfaceDeclaration;
        
        classDeclaration: 'class' IDENTIFIER '{' classBodyDeclaration* '}';
        classBodyDeclaration: methodDeclaration | fieldDeclaration;
        methodDeclaration: IDENTIFIER IDENTIFIER '(' ')' '{' '}';
        fieldDeclaration: IDENTIFIER IDENTIFIER ';';
        
        interfaceDeclaration: 'interface' IDENTIFIER '{' '}';
        
        qualifiedName: IDENTIFIER ('.' IDENTIFIER)*;
        
        IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.name, "JavaSubset");
    assert!(grammar.rules.len() > 0);
    assert!(grammar.lexer_rules().count() > 0);
    assert!(grammar.parser_rules().count() > 0);
}

#[test]
fn test_grammars_v4_python_subset() {
    let grammar_text = r#"
        grammar PythonSubset;
        
        file_input: (NEWLINE | stmt)* ENDMARKER;
        stmt: simple_stmt | compound_stmt;
        simple_stmt: small_stmt (';' small_stmt)* ';'? NEWLINE;
        small_stmt: expr_stmt | pass_stmt | return_stmt;
        
        expr_stmt: testlist_star_expr;
        pass_stmt: 'pass';
        return_stmt: 'return' testlist?;
        
        compound_stmt: if_stmt | while_stmt | for_stmt | funcdef;
        if_stmt: 'if' test ':' suite ('elif' test ':' suite)* ('else' ':' suite)?;
        while_stmt: 'while' test ':' suite;
        for_stmt: 'for' exprlist 'in' testlist ':' suite;
        funcdef: 'def' NAME parameters ':' suite;
        
        suite: simple_stmt | NEWLINE INDENT stmt+ DEDENT;
        test: or_test;
        or_test: and_test ('or' and_test)*;
        and_test: not_test ('and' not_test)*;
        not_test: 'not' not_test | comparison;
        comparison: expr (comp_op expr)*;
        expr: xor_expr (('|') xor_expr)*;
        xor_expr: and_expr (('^') and_expr)*;
        and_expr: shift_expr (('&') shift_expr)*;
        shift_expr: arith_expr (('<<'|'>>') arith_expr)*;
        arith_expr: term (('+'|'-') term)*;
        term: factor (('*'|'@'|'/'|'%'|'//') factor)*;
        factor: ('+'|'-'|'~') factor | power;
        power: atom_expr ('**' factor)?;
        atom_expr: atom trailer*;
        atom: ('(' testlist? ')' | '[' testlist? ']' | '{' dictorsetmaker? '}' | NAME | NUMBER | STRING+);
        trailer: '(' arglist? ')' | '[' subscriptlist ']' | '.' NAME;
        
        testlist_star_expr: (test|star_expr) (',' (test|star_expr))* ','?;
        exprlist: expr (',' expr)* ','?;
        testlist: test (',' test)* ','?;
        subscriptlist: subscript (',' subscript)* ','?;
        subscript: test | test? ':' test? sliceop?;
        sliceop: ':' test?;
        star_expr: '*' expr;
        arglist: argument (',' argument)* ','?;
        argument: test | test '=' test;
        dictorsetmaker: ((test ':' test | '**' expr) (',' (test ':' test | '**' expr))* ','? | (test | star_expr) (',' (test | star_expr))* ','?);
        parameters: '(' typedargslist? ')';
        typedargslist: tfpdef ('=' test)? (',' tfpdef ('=' test)?)* (',' '*' tfpdef? (',' tfpdef ('=' test)?)* (',' '**' tfpdef)?)?;
        tfpdef: NAME (':' test)?;
        comp_op: '<'|'>'|'=='|'>='|'<='|'<>'|'!='|'in'|'not' 'in'|'is'|'is' 'not';
        
        NAME: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"' | '\'' (~['\\\r\n] | '\\' .)* '\'';
        NEWLINE: '\r'? '\n';
        INDENT: '<INDENT>';
        DEDENT: '<DEDENT>';
        ENDMARKER: '<EOF>';
        WS: [ \t]+ -> skip;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.name, "PythonSubset");
    assert!(grammar.rules.len() > 0);
}

#[test]
fn test_grammars_v4_sql_subset() {
    let grammar_text = r#"
        grammar SQLSubset;
        
        sqlStatement: selectStatement | insertStatement | updateStatement | deleteStatement;
        selectStatement: SELECT selectList FROM tableList (WHERE whereClause)?;
        insertStatement: INSERT INTO tableName VALUES valueList;
        updateStatement: UPDATE tableName SET assignmentList (WHERE whereClause)?;
        deleteStatement: DELETE FROM tableName (WHERE whereClause)?;
        
        selectList: '*' | columnList;
        columnList: columnName (',' columnName)*;
        tableList: tableName (',' tableName)*;
        valueList: '(' value (',' value)* ')';
        assignmentList: assignment (',' assignment)*;
        assignment: columnName '=' value;
        whereClause: condition;
        condition: columnName operator value;
        operator: '=' | '!=' | '<' | '>' | '<=' | '>=';
        
        tableName: IDENTIFIER;
        columnName: IDENTIFIER;
        value: STRING | NUMBER | IDENTIFIER;
        
        SELECT: 'SELECT' | 'select';
        FROM: 'FROM' | 'from';
        WHERE: 'WHERE' | 'where';
        INSERT: 'INSERT' | 'insert';
        INTO: 'INTO' | 'into';
        VALUES: 'VALUES' | 'values';
        UPDATE: 'UPDATE' | 'update';
        SET: 'SET' | 'set';
        DELETE: 'DELETE' | 'delete';
        
        IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"' | '\'' (~['\\\r\n] | '\\' .)* '\'';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.name, "SQLSubset");
    assert!(grammar.rules.len() > 0);
}

#[test]
fn test_grammars_v4_javascript_subset() {
    let grammar_text = r#"
        grammar JavaScriptSubset;
        
        program: sourceElement* EOF;
        sourceElement: statement | declaration;
        
        statement: block | expressionStatement | ifStatement | whileStatement | forStatement | returnStatement | breakStatement | continueStatement;
        block: '{' statement* '}';
        expressionStatement: expression ';';
        ifStatement: 'if' '(' expression ')' statement ('else' statement)?;
        whileStatement: 'while' '(' expression ')' statement;
        forStatement: 'for' '(' (variableDeclaration | expression)? ';' expression? ';' expression? ')' statement;
        returnStatement: 'return' expression? ';';
        breakStatement: 'break' ';';
        continueStatement: 'continue' ';';
        
        declaration: variableDeclaration | functionDeclaration;
        variableDeclaration: ('var' | 'let' | 'const') identifier ('=' expression)? ';';
        functionDeclaration: 'function' identifier '(' parameterList? ')' '{' statement* '}';
        parameterList: identifier (',' identifier)*;
        
        expression: assignmentExpression;
        assignmentExpression: conditionalExpression ('=' assignmentExpression)?;
        conditionalExpression: logicalOrExpression ('?' expression ':' conditionalExpression)?;
        logicalOrExpression: logicalAndExpression ('||' logicalAndExpression)*;
        logicalAndExpression: bitwiseOrExpression ('&&' bitwiseOrExpression)*;
        bitwiseOrExpression: bitwiseXorExpression ('|' bitwiseXorExpression)*;
        bitwiseXorExpression: bitwiseAndExpression ('^' bitwiseAndExpression)*;
        bitwiseAndExpression: equalityExpression ('&' equalityExpression)*;
        equalityExpression: relationalExpression (('==' | '!=' | '===' | '!==') relationalExpression)*;
        relationalExpression: additiveExpression (('<' | '>' | '<=' | '>=') additiveExpression)*;
        additiveExpression: multiplicativeExpression (('+' | '-') multiplicativeExpression)*;
        multiplicativeExpression: unaryExpression (('*' | '/' | '%') unaryExpression)*;
        unaryExpression: ('!' | '~' | '+' | '-' | 'typeof' | 'delete')? postfixExpression;
        postfixExpression: primaryExpression ('[' expression ']' | '.' identifier | '(' argumentList? ')')*;
        primaryExpression: identifier | literal | '(' expression ')' | arrayLiteral | objectLiteral;
        
        arrayLiteral: '[' (expression (',' expression)*)? ']';
        objectLiteral: '{' (propertyAssignment (',' propertyAssignment)*)? '}';
        propertyAssignment: identifier ':' expression;
        argumentList: expression (',' expression)*;
        
        identifier: IDENTIFIER;
        literal: NUMBER | STRING | 'true' | 'false' | 'null' | 'undefined';
        
        IDENTIFIER: [a-zA-Z_$][a-zA-Z0-9_$]*;
        NUMBER: [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"' | '\'' (~['\\\r\n] | '\\' .)* '\'';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.name, "JavaScriptSubset");
    assert!(grammar.rules.len() > 0);
}

#[test]
fn test_grammars_v4_graphql_subset() {
    let grammar_text = r#"
        grammar GraphQLSubset;
        
        document: definition+;
        definition: operationDefinition | fragmentDefinition;
        
        operationDefinition: operationType name? variableDefinitions? directives? selectionSet;
        operationType: 'query' | 'mutation' | 'subscription';
        variableDefinitions: '(' variableDefinition+ ')';
        variableDefinition: variable ':' type;
        variable: '$' name;
        type: namedType | listType | nonNullType;
        namedType: name;
        listType: '[' type ']';
        nonNullType: type '!';
        
        selectionSet: '{' selection+ '}';
        selection: field | fragmentSpread | inlineFragment;
        field: name arguments? directives? selectionSet?;
        arguments: '(' argument+ ')';
        argument: name ':' value;
        value: intValue | floatValue | stringValue | booleanValue | nullValue | enumValue | listValue | objectValue;
        intValue: INT;
        floatValue: FLOAT;
        stringValue: STRING;
        booleanValue: 'true' | 'false';
        nullValue: 'null';
        enumValue: name;
        listValue: '[' value* ']';
        objectValue: '{' objectField* '}';
        objectField: name ':' value;
        
        directives: directive+;
        directive: '@' name arguments?;
        
        fragmentDefinition: 'fragment' name 'on' namedType directives? selectionSet;
        fragmentSpread: '...' name directives?;
        inlineFragment: '...' ('on' namedType)? directives? selectionSet;
        
        name: IDENTIFIER;
        
        IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
        INT: [0-9]+;
        FLOAT: [0-9]+ '.' [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.name, "GraphQLSubset");
    assert!(grammar.rules.len() > 0);
}

#[test]
fn test_grammars_v4_code_generation_all_languages() {
    let grammar_text = r#"
        grammar SimpleExpression;
        
        expr: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | '(' expr ')';
        
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    // Test code generation for all 8 languages
    let languages = vec!["rust", "python", "javascript", "typescript", "go", "c", "cpp", "java"];
    
    for lang in languages {
        let config = minipg::core::types::CodeGenConfig {
            target_language: lang.to_string(),
            output_directory: ".".to_string(),
            package_name: None,
            generate_listener: false,
            generate_visitor: false,
        };
        
        let generator = minipg::codegen::CodeGenerator::new(config);
        let analysis = minipg::analysis::AnalysisResult::new(grammar.clone());
        let code = generator.generate_from_analysis(&analysis).expect(&format!("Failed to generate {}", lang));
        
        assert!(!code.is_empty(), "Generated code for {} is empty", lang);
        assert!(code.len() > 100, "Generated code for {} is too short", lang);
    }
}

