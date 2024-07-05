extern crate swc_common;
extern crate swc_ecma_parser;
use std::path::Path;
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::ModuleItem;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use tstz::typescript::get_value;

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm
        .load_file(Path::new("./examples/typescript/boomerang.ts"))
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

    // let mut operations = vec![];
    let mut type_env = vec![];

    for stmt in module.body {
        if let ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(swc_ecma_ast::Decl::Fn(fn_decl))) = stmt {
            if fn_decl.ident.sym == "smartContract" {
                for param in fn_decl.function.params.iter() {
                    type_env.push(get_value(param.pat.to_owned().expect_ident()));
                }
                dbg!(&type_env);
            }
        }
    }
}
