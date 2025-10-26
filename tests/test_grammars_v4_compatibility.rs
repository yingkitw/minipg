//! Tests for grammars-v4 repository compatibility.
//! 
//! This test suite verifies that minipg can parse and generate code for
//! real-world grammars from the ANTLR4 grammars-v4 repository.

use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;

/// Test parsing a simplified Java grammar (subset of grammars-v4/java/java9)
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
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "java.g4").expect("Failed to parse Java grammar");
    
    // Verify grammar structure
    assert_eq!(grammar.name, "JavaSubset");
    assert!(grammar.rules.len() > 0);
    assert!(grammar.lexer_rules().count() > 0);
    assert!(grammar.parser_rules().count() > 0);
}

/// Test parsing a simplified Python grammar (subset of grammars-v4/python/python3)
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
        INDENT: '    ';
        DEDENT: '';
        ENDMARKER: EOF;
        WS: [ \t]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "python.g4").expect("Failed to parse Python grammar");
    
    // Verify grammar structure
    assert_eq!(grammar.name, "PythonSubset");
    assert!(grammar.rules.len() > 0);
}

/// Test parsing a simplified SQL grammar (subset of grammars-v4/sql/tsql)
#[test]
fn test_grammars_v4_sql_subset() {
    let grammar_text = r#"
        grammar SQLSubset;
        
        root: batch EOF;
        batch: go_statement*;
        go_statement: sql_statement;
        
        sql_statement: select_statement | insert_statement | update_statement | delete_statement;
        
        select_statement: 'SELECT' select_list from_clause? where_clause? group_by_clause? order_by_clause?;
        select_list: select_item (',' select_item)*;
        select_item: expression (AS? ID)?;
        from_clause: 'FROM' table_reference;
        where_clause: 'WHERE' expression;
        group_by_clause: 'GROUP' 'BY' expression (',' expression)*;
        order_by_clause: 'ORDER' 'BY' order_by_item (',' order_by_item)*;
        order_by_item: expression ('ASC' | 'DESC')?;
        
        insert_statement: 'INSERT' 'INTO' table_name ('(' column_name_list ')')? 'VALUES' '(' expression_list ')';
        update_statement: 'UPDATE' table_name 'SET' assignment (',' assignment)* where_clause?;
        delete_statement: 'DELETE' 'FROM' table_name where_clause?;
        
        assignment: column_name '=' expression;
        table_reference: table_name (AS? ID)?;
        table_name: ID;
        column_name: ID;
        column_name_list: column_name (',' column_name)*;
        expression_list: expression (',' expression)*;
        
        expression: or_expression;
        or_expression: and_expression ('OR' and_expression)*;
        and_expression: comparison_expression ('AND' comparison_expression)*;
        comparison_expression: additive_expression (comparison_operator additive_expression)?;
        additive_expression: multiplicative_expression (('+' | '-') multiplicative_expression)*;
        multiplicative_expression: unary_expression (('*' | '/' | '%') unary_expression)*;
        unary_expression: ('+' | '-')? primary_expression;
        primary_expression: ID | NUMBER | STRING | '(' expression ')' | function_call;
        function_call: ID '(' expression_list? ')';
        
        comparison_operator: '=' | '<' | '>' | '<=' | '>=' | '<>' | '!=';
        
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        STRING: '\'' (~['\\\r\n] | '\\' .)* '\'';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "sql.g4").expect("Failed to parse SQL grammar");
    
    // Verify grammar structure
    assert_eq!(grammar.name, "SQLSubset");
    assert!(grammar.rules.len() > 0);
}

/// Test parsing a simplified C grammar (subset of grammars-v4/c/c)
#[test]
fn test_grammars_v4_c_subset() {
    let grammar_text = r#"
        grammar CSubset;
        
        translationUnit: externalDeclaration*;
        externalDeclaration: functionDefinition | declaration;
        
        functionDefinition: declarationSpecifiers declarator compoundStatement;
        declaration: declarationSpecifiers initDeclaratorList? ';';
        declarationSpecifiers: typeSpecifier;
        typeSpecifier: 'void' | 'char' | 'short' | 'int' | 'long' | 'float' | 'double' | 'signed' | 'unsigned';
        
        initDeclaratorList: initDeclarator (',' initDeclarator)*;
        initDeclarator: declarator ('=' initializer)?;
        declarator: pointer? directDeclarator;
        directDeclarator: ID | '(' declarator ')' | directDeclarator '[' constantExpression? ']' | directDeclarator '(' parameterTypeList? ')';
        pointer: '*' pointer?;
        parameterTypeList: parameterDeclaration (',' parameterDeclaration)*;
        parameterDeclaration: declarationSpecifiers declarator?;
        
        initializer: assignmentExpression | '{' initializerList? '}';
        initializerList: initializer (',' initializer)*;
        
        compoundStatement: '{' blockItem* '}';
        blockItem: declaration | statement;
        statement: labeledStatement | compoundStatement | expressionStatement | selectionStatement | iterationStatement | jumpStatement;
        
        labeledStatement: ID ':' statement;
        expressionStatement: expression? ';';
        selectionStatement: 'if' '(' expression ')' statement ('else' statement)?;
        iterationStatement: 'while' '(' expression ')' statement | 'for' '(' expression? ';' expression? ';' expression? ')' statement;
        jumpStatement: 'return' expression? ';';
        
        expression: assignmentExpression;
        assignmentExpression: conditionalExpression ('=' assignmentExpression)?;
        conditionalExpression: logicalOrExpression ('?' expression ':' conditionalExpression)?;
        logicalOrExpression: logicalAndExpression ('||' logicalAndExpression)*;
        logicalAndExpression: equalityExpression ('&&' equalityExpression)*;
        equalityExpression: relationalExpression (('==' | '!=') relationalExpression)*;
        relationalExpression: additiveExpression (('<' | '>' | '<=' | '>=') additiveExpression)*;
        additiveExpression: multiplicativeExpression (('+' | '-') multiplicativeExpression)*;
        multiplicativeExpression: unaryExpression (('*' | '/' | '%') unaryExpression)*;
        unaryExpression: ('&' | '*' | '+' | '-' | '~' | '!')? postfixExpression;
        postfixExpression: primaryExpression ('[' expression ']' | '(' argumentExpressionList? ')' | '.' ID | '->' ID)*;
        primaryExpression: ID | CONSTANT | STRING | '(' expression ')';
        argumentExpressionList: assignmentExpression (',' assignmentExpression)*;
        constantExpression: conditionalExpression;
        
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        CONSTANT: [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "c.g4").expect("Failed to parse C grammar");
    
    // Verify grammar structure
    assert_eq!(grammar.name, "CSubset");
    assert!(grammar.rules.len() > 0);
}

/// Test parsing a simplified JavaScript grammar (subset of grammars-v4/javascript/javascript)
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
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "javascript.g4").expect("Failed to parse JavaScript grammar");
    
    // Verify grammar structure
    assert_eq!(grammar.name, "JavaScriptSubset");
    assert!(grammar.rules.len() > 0);
}

/// Test parsing a simplified GraphQL grammar (subset of grammars-v4/graphql)
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
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "graphql.g4").expect("Failed to parse GraphQL grammar");
    
    // Verify grammar structure
    assert_eq!(grammar.name, "GraphQLSubset");
    assert!(grammar.rules.len() > 0);
}

/// Test code generation for all grammars-v4 examples
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
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "expr.g4").expect("Failed to parse");
    
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
        
        // Verify code was generated
        assert!(!code.is_empty(), "Generated code for {} is empty", lang);
        assert!(code.len() > 100, "Generated code for {} is too short", lang);
    }
}

/// Test ANTLR4 compatibility features with grammars-v4 style grammars
#[test]
fn test_grammars_v4_antlr4_features() {
    let grammar_text = r#"
        grammar AdvancedFeatures;
        
        options {
            language = java;
            tokenVocab = CommonTokens;
        }
        
        @header {
            package com.example.parser;
            import java.util.*;
        }
        
        @members {
            private int depth = 0;
        }
        
        program: statement+ EOF;
        statement: assignment | expression;
        assignment: ID '=' expression;
        expression: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | ID | '(' expression ')';
        
        ID: [a-zA-Z_]+;
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "advanced.g4").expect("Failed to parse");
    
    // Verify ANTLR4 features
    assert_eq!(grammar.options.get("language").map(|s| s.as_str()), Some("java"));
    assert_eq!(grammar.options.get("tokenVocab").map(|s| s.as_str()), Some("CommonTokens"));
    assert_eq!(grammar.named_actions.len(), 2);
    assert!(grammar.named_actions.contains_key("header"));
    assert!(grammar.named_actions.contains_key("members"));
}

/// Test complex nested structures (common in real grammars)
#[test]
fn test_grammars_v4_complex_nesting() {
    let grammar_text = r#"
        grammar ComplexNesting;
        
        root: element+;
        element: simpleElement | complexElement;
        simpleElement: IDENTIFIER;
        complexElement: '{' element* '}' | '[' element* ']' | '(' element* ')';
        
        IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "complex.g4").expect("Failed to parse");
    
    // Verify grammar handles recursion
    assert!(grammar.rules.len() > 0);
    let root_rule = grammar.get_rule("root").expect("root rule not found");
    assert!(root_rule.alternatives.len() > 0);
}

/// Test left-recursive grammar patterns (common in expression grammars)
#[test]
fn test_grammars_v4_left_recursion_detection() {
    let grammar_text = r#"
        grammar LeftRecursion;
        
        expr: expr '+' term | term;
        term: term '*' factor | factor;
        factor: NUMBER | '(' expr ')';
        
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "leftrecurse.g4").expect("Failed to parse");
    
    // Verify grammar was parsed (left recursion detection is in semantic analysis)
    assert_eq!(grammar.name, "LeftRecursion");
    assert!(grammar.rules.len() > 0);
}

/// Test Unicode and special characters in grammars
#[test]
fn test_grammars_v4_unicode_support() {
    let grammar_text = r#"
        grammar UnicodeSupport;
        
        program: statement+;
        statement: IDENTIFIER | NUMBER | STRING;
        
        IDENTIFIER: [a-zA-Z_\u0080-\uFFFF][a-zA-Z0-9_\u0080-\uFFFF]*;
        NUMBER: [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "unicode.g4").expect("Failed to parse");
    
    // Verify Unicode support
    assert_eq!(grammar.name, "UnicodeSupport");
}
