extern crate swc_common;
extern crate swc_ecma_parser;
use std::path::Path;
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::ModuleItem;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

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

    for stmt in module.body {
        if let ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(swc_ecma_ast::Decl::Fn(fn_decl))) = stmt {
            if fn_decl.ident.sym == "smartContract" {
                dbg!(fn_decl);
            }
        }
    }
}
