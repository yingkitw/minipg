//! Command implementations.

use crate::cli::{Cli, Commands};
use anyhow::{Context, Result};
use minipg_analysis::SemanticAnalyzer;
use minipg_codegen::CodeGenerator;
use minipg_core::{
    types::CodeGenConfig, GrammarParser,
    SemanticAnalyzer as SemanticAnalyzerTrait,
};
use minipg_parser::GrammarParser as Parser;
use std::fs;
use std::path::Path;
use tracing::{error, info};

pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Generate {
            input,
            output,
            target_language,
            package,
            visitor,
            listener,
        } => generate_command(input, output, target_language, package, visitor, listener),
        Commands::Validate { input } => validate_command(input),
        Commands::Info { input } => info_command(input),
    }
}

fn generate_command(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    target_language: String,
    package: Option<String>,
    visitor: bool,
    listener: bool,
) -> Result<()> {
    let input = input.as_ref();
    let output = output.as_ref();

    info!("Generating parser from: {}", input.display());

    // Parse grammar
    let parser = Parser::new();
    let grammar = parser
        .parse_file(input)
        .context("Failed to parse grammar file")?;

    info!("Parsed grammar: {}", grammar.name);

    // Analyze grammar
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer
        .analyze(&grammar)
        .context("Failed to analyze grammar")?;

    // Report diagnostics
    for diagnostic in &analysis.diagnostics {
        match diagnostic.severity {
            minipg_core::DiagnosticSeverity::Error => error!("{}", diagnostic),
            minipg_core::DiagnosticSeverity::Warning => {
                tracing::warn!("{}", diagnostic)
            }
            minipg_core::DiagnosticSeverity::Info => info!("{}", diagnostic),
        }
    }

    if analysis.has_errors() {
        anyhow::bail!("Grammar has errors, aborting code generation");
    }

    // Generate code
    let config = CodeGenConfig {
        target_language,
        output_directory: output.to_string_lossy().to_string(),
        package_name: package,
        generate_listener: listener,
        generate_visitor: visitor,
    };

    let generator = CodeGenerator::new(config);
    let code = generator
        .generate_from_analysis(&analysis)
        .context("Failed to generate code")?;

    // Write output
    fs::create_dir_all(output).context("Failed to create output directory")?;
    let output_file = output.join(format!("{}_parser.rs", grammar.name.to_lowercase()));
    fs::write(&output_file, code).context("Failed to write output file")?;

    info!("Generated parser: {}", output_file.display());
    Ok(())
}

fn validate_command(input: impl AsRef<Path>) -> Result<()> {
    let input = input.as_ref();
    info!("Validating grammar: {}", input.display());

    // Parse grammar
    let parser = Parser::new();
    let grammar = parser
        .parse_file(input)
        .context("Failed to parse grammar file")?;

    // Analyze grammar
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer
        .analyze(&grammar)
        .context("Failed to analyze grammar")?;

    // Report diagnostics
    let mut has_errors = false;
    for diagnostic in &analysis.diagnostics {
        match diagnostic.severity {
            minipg_core::DiagnosticSeverity::Error => {
                error!("{}", diagnostic);
                has_errors = true;
            }
            minipg_core::DiagnosticSeverity::Warning => {
                tracing::warn!("{}", diagnostic)
            }
            minipg_core::DiagnosticSeverity::Info => info!("{}", diagnostic),
        }
    }

    if has_errors {
        anyhow::bail!("Grammar validation failed");
    } else {
        info!("Grammar is valid");
    }

    Ok(())
}

fn info_command(input: impl AsRef<Path>) -> Result<()> {
    let input = input.as_ref();
    info!("Analyzing grammar: {}", input.display());

    // Parse grammar
    let parser = Parser::new();
    let grammar = parser
        .parse_file(input)
        .context("Failed to parse grammar file")?;

    // Print grammar information
    println!("Grammar: {}", grammar.name);
    println!("Type: {:?}", grammar.grammar_type);
    println!("Rules: {}", grammar.rules.len());
    println!("  Parser rules: {}", grammar.parser_rules().count());
    println!("  Lexer rules: {}", grammar.lexer_rules().count());

    if !grammar.imports.is_empty() {
        println!("Imports:");
        for import in &grammar.imports {
            println!("  - {}", import);
        }
    }

    if !grammar.options.is_empty() {
        println!("Options:");
        for (key, value) in &grammar.options {
            println!("  {} = {}", key, value);
        }
    }

    Ok(())
}
