extern crate swc_common;
extern crate swc_ecma_parser;
use clap::Parser as CParser;
use std::path::{Path, PathBuf};
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::{
    CallExpr, Callee, Decl, Expr, ModuleItem, Stmt, VarDecl, VarDeclKind, VarDeclarator,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use tstz::{
    mlir::{Operation, OperationKind, Type, Value},
    typescript::get_value,
};

#[derive(CParser, Debug)]
#[command(name = "FilePath")]
struct Args {
    #[arg(short, long, value_name = "INPUT", required = true)]
    input: PathBuf,
    #[arg(short, long, value_name = "OUTPUT", required = false)]
    output: Option<PathBuf>,
}

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let args = Args::parse();
    let fm = cm
        .load_file(Path::new(&args.input))
        .expect("failed to load test.js");
    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Typescript(Default::default()),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module().unwrap();

    let mut operations = vec![];
    let mut type_env = vec![];

    for stmt in module.body {
        if let ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(swc_ecma_ast::Decl::Fn(fn_decl))) = stmt {
            if fn_decl.ident.sym == "smartContract" {
                for param in fn_decl.function.params.iter() {
                    type_env.push(get_value(param.pat.to_owned().expect_ident()));
                }

                for stmt in fn_decl.function.body.unwrap().stmts {
                    process_stmt(stmt, &mut type_env, &mut operations);
                }
            }
        }
    }

    let storage = type_env
        .iter()
        .find(|value| value.id == "%storage")
        .unwrap();
    let param = type_env.iter().find(|value| value.id == "%param").unwrap();

    let mut output = String::new();

    output.push_str("module {\n");
    output.push_str(&format!(
        "  func.func @smart_contract({}: {}, {}: {}) -> {} {{\n",
        param.id,
        param.ty,
        storage.id,
        storage.ty,
        Type::Pair {
            fst: Box::new(Type::List {
                elem: Box::new(Type::Operation)
            }),
            snd: Box::new(storage.ty.to_owned())
        }
    ));
    for op in operations {
        output.push_str(&format!("    {}\n", op));
    }
    output.push_str("  }\n");
    output.push('}');

    if let Some(output_path) = args.output {
        std::fs::write(output_path, output).expect("failed to write output");
    } else {
        println!("{}", output);
    }
}

fn process_stmt(
    stmt: swc_ecma_ast::Stmt,
    type_env: &mut Vec<Value>,
    operations: &mut Vec<Operation>,
) {
    match stmt {
        Stmt::Decl(Decl::Var(var_decl)) => {
            let VarDecl {
                span: _,
                kind,
                declare: _,
                decls,
            } = *var_decl;

            assert_eq!(kind, VarDeclKind::Const);
            assert_eq!(decls.len(), 1);
            let decl = &decls[0];
            let VarDeclarator {
                span: _,
                name,
                init,
                definite: _,
            } = decl;
            let value = get_value(name.to_owned().expect_ident());
            if let Some(expr) = init {
                let call_expr = match &**expr {
                    Expr::Call(call_expr) => call_expr,
                    _ => unreachable!(),
                };
                let CallExpr {
                    span: _,
                    callee,
                    args,
                    type_args: _,
                } = call_expr;

                if let Callee::Expr(callee_expr) = callee {
                    let callee_ident = match &**callee_expr {
                        Expr::Ident(ident) => ident,
                        _ => unreachable!(),
                    };
                    let name = callee_ident.sym.to_string();

                    let kind = OperationKind::from(name.as_str());
                    let args = args
                        .iter()
                        .map(|arg| {
                            let sym = arg.to_owned().expr.expect_ident().sym.to_string();
                            type_env
                                .iter()
                                .find(|v| v.id == format!("%{}", sym))
                                .unwrap()
                                .to_owned()
                        })
                        .collect::<Vec<_>>();

                    let results = vec![value.to_owned()];
                    operations.push(Operation {
                        kind,
                        args,
                        results,
                    });
                    type_env.push(value);
                }
            }
        }
        Stmt::Return(return_stms) => {
            let sym = return_stms.arg.unwrap().expect_ident().sym.to_string();
            let args = type_env
                .iter()
                .filter(|v| v.id == format!("%{}", sym))
                .map(|v| v.to_owned())
                .collect::<Vec<_>>();
            operations.push(Operation {
                kind: OperationKind::Return,
                args,
                results: vec![],
            });
        }
        Stmt::Expr(expr_stmt) => {
            let expr = expr_stmt.expr;
            let call_expr = match &*expr {
                Expr::Call(call_expr) => call_expr,
                _ => unreachable!(),
            };
            let CallExpr {
                span: _,
                callee,
                args,
                type_args: _,
            } = call_expr;

            if let Callee::Expr(callee_expr) = callee {
                let callee_ident = match &**callee_expr {
                    Expr::Ident(ident) => ident,
                    _ => unreachable!(),
                };
                let name = callee_ident.sym.to_string();

                if name == "assert" {
                    let args = args
                        .iter()
                        .map(|arg| {
                            let sym = arg.to_owned().expr.expect_ident().sym.to_string();
                            type_env
                                .iter()
                                .find(|v| v.id == format!("%{}", sym))
                                .unwrap()
                                .to_owned()
                        })
                        .collect::<Vec<_>>();
                    operations.push(Operation {
                        kind: OperationKind::Assert,
                        args,
                        results: vec![],
                    });
                } else {
                    todo!("unexpected function: {:?}", name);
                }
            }
        }
        _ => {
            unreachable!("unexpected statement: {:?}", stmt);
        }
    }
}
