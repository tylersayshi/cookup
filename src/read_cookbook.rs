use std::fs;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::{Expr, Module, ModuleDecl, TsSatisfiesExpr};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

use crate::utils::{get_storage_path, SavedRecipe};

fn extract_default_export_list(module: &Module) -> Option<Vec<SavedRecipe>> {
    for item in &module.body {
        if let swc_ecma_ast::ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(expr)) = item {
            // Check if the export is a "satisfies" expression (type assertion)
            let expr = match &*expr.expr {
                Expr::TsSatisfies(TsSatisfiesExpr { expr, .. }) => &**expr, // Unwrap `satisfies` type assertion
                other => other,
            };

            // Now check if it is an array literal
            if let Expr::Array(arr) = expr {
                let list = arr
                    .elems
                    .iter()
                    .filter_map(|elem| {
                        if let Some(Expr::Object(obj)) = elem.as_ref().map(|e| &*e.expr) {
                            let mut name = String::new();
                            let mut created_at = String::new();
                            let mut ingredients = String::new();
                            let mut instructions = String::new();

                            for prop in &obj.props {
                                if let swc_ecma_ast::PropOrSpread::Prop(boxed_prop) = prop {
                                    if let swc_ecma_ast::Prop::KeyValue(kv) = &**boxed_prop {
                                        if let Some(key) = kv.key.as_ident() {
                                            match &*kv.value {
                                                Expr::Lit(swc_ecma_ast::Lit::Str(lit)) => {
                                                    // Handle regular string
                                                    match &*key.sym {
                                                        "name" => name = lit.value.to_string(),
                                                        "created_at" => {
                                                            created_at = lit.value.to_string()
                                                        }
                                                        "ingredients" => {
                                                            ingredients = lit.value.to_string()
                                                        }
                                                        "instructions" => {
                                                            instructions = lit.value.to_string()
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                Expr::Tpl(tpl) => {
                                                    // Handle template literals
                                                    let combined_string = tpl
                                                        .quasis
                                                        .iter()
                                                        .map(|quasi| {
                                                            quasi
                                                                .cooked
                                                                .as_ref()
                                                                .map(|c| c.to_string())
                                                                .unwrap_or_default()
                                                        })
                                                        .collect::<String>();

                                                    match &*key.sym {
                                                        "ingredients" => {
                                                            ingredients = combined_string
                                                        }
                                                        "instructions" => {
                                                            instructions = combined_string
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }

                            if !name.is_empty()
                                && !ingredients.is_empty()
                                && !instructions.is_empty()
                                && !created_at.is_empty()
                            {
                                Some(SavedRecipe {
                                    name,
                                    created_at,
                                    ingredients,
                                    instructions,
                                })
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                return Some(list);
            }
        }
    }
    None
}

pub fn read_cookbook() -> Vec<SavedRecipe> {
    let cm: Lrc<SourceMap> = Default::default();

    let file_path = get_storage_path().unwrap();
    let code = fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

    let fm = cm.new_source_file(FileName::Real(file_path.into()).into(), code);

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module().expect("Failed to parse module");

    extract_default_export_list(&module).unwrap_or_default()
}
