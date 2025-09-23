//! Parser demo for TJLang

use tjlang_parser::parse;
use codespan::Files;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

fn main() {
    let mut files = Files::new();
    let file_id = files.add("demo.tj", include_str!("demo.tj"));
    
    println!("🔍 Parsing TJLang demo...\n");
    
    match parse(include_str!("demo.tj"), file_id) {
        Ok((program, diagnostics)) => {
            println!("✅ Parsing successful!");
            println!("📊 Program units: {}", program.units.len());
            
            for (i, unit) in program.units.iter().enumerate() {
                println!("\n--- Unit {} ---", i + 1);
                match unit {
                    tjlang_ast::ProgramUnit::Module(module) => {
                        println!("Module: {}", module.name);
                    },
                    tjlang_ast::ProgramUnit::Import(import) => {
                        match import {
                            tjlang_ast::ImportDecl::Simple { module, alias, .. } => {
                                println!("Import: {} as {:?}", 
                                    module.parts.join("."), 
                                    alias.as_ref().unwrap_or(&"<none>".to_string())
                                );
                            },
                            tjlang_ast::ImportDecl::Selective { module, items, .. } => {
                                println!("Import: {}.{{ {} }}", 
                                    module.parts.join("."), 
                                    items.join(", ")
                                );
                            },
                        }
                    },
                    tjlang_ast::ProgramUnit::Export(export) => {
                        println!("Export: {:?}", export.item);
                    },
                    tjlang_ast::ProgramUnit::Declaration(decl) => {
                        match decl {
                            tjlang_ast::Declaration::Function(func) => {
                                println!("Function: {}(", func.name);
                                for param in &func.params {
                                    println!("  {}: {:?}", param.name, param.param_type);
                                }
                                println!(") -> {:?}", func.return_type);
                            },
                            tjlang_ast::Declaration::Variable(var) => {
                                println!("Variable: {}: {:?} = {:?}", 
                                    var.name, var.var_type, var.value);
                            },
                            tjlang_ast::Declaration::Enum(enum_decl) => {
                                println!("Enum: {}<{}>", 
                                    enum_decl.name, 
                                    enum_decl.type_params.join(", ")
                                );
                                for variant in &enum_decl.variants {
                                    println!("  {}(", variant.name);
                                    for field in &variant.fields {
                                        println!("    {:?}", field);
                                    }
                                    println!("  )");
                                }
                            },
                            tjlang_ast::Declaration::Struct(struct_decl) => {
                                println!("Struct: {}", struct_decl.name);
                                for field in &struct_decl.fields {
                                    println!("  {}: {:?}", field.name, field.field_type);
                                }
                            },
                            tjlang_ast::Declaration::Interface(interface_decl) => {
                                println!("Interface: {}", interface_decl.name);
                                for method in &interface_decl.methods {
                                    println!("  {}() -> {:?}", method.name, method.return_type);
                                }
                            },
                            _ => println!("Other declaration: {:?}", decl),
                        }
                    },
                }
            }
            
            if !diagnostics.is_empty() {
                println!("\n⚠️  Warnings/Errors:");
                let writer = StandardStream::stderr(ColorChoice::Always);
                let config = codespan_reporting::term::Config::default();
                
                for diagnostic in diagnostics.diagnostics() {
                    term::emit(&mut writer.lock(), &config, &files, diagnostic).unwrap();
                }
            }
        },
        Err(diagnostics) => {
            println!("❌ Parsing failed!");
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();
            
            for diagnostic in diagnostics.diagnostics() {
                term::emit(&mut writer.lock(), &config, &files, diagnostic).unwrap();
            }
        }
    }
}
