#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate itertools;
extern crate serde;
extern crate serde_json;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::*;
use syntax::ext::base::*;
use syntax::ext::build::AstBuilder;
use rustc_plugin::Registry;
use syntax::parse::token::intern;
use syntax::parse::token::{gensym, gensym_ident};
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;
use syntax::codemap::DUMMY_SP;
use syntax::codemap::Spanned;
use syntax::attr::mk_attr_id;
use syntax::parse::token::InternedString;
use syntax::abi::Abi::Rust;
use syntax::parse::token::Token;
use syntax::parse::token::Lit::Str_;
use itertools::Itertools;

#[macro_export]
macro_rules! assert_in_out_file {
    ($path:expr, $in_type:ty, $binary:expr) => {{
        use std::io::{Read, Write};

        let in_path = concat!($path, "-in.json");
        let out_path = concat!($path, "-out.json");

        println!("{} {}", in_path, out_path);

        let mut in_file = ::std::fs::File::open(in_path).unwrap();
        let mut out_file = ::std::fs::File::open(out_path).unwrap();

        let executable = format!("target/debug/{}", $binary);
        let process = ::std::process::Command::new(executable)
                              .stdin(::std::process::Stdio::piped())
                              .stdout(::std::process::Stdio::piped())
                              .stderr(::std::process::Stdio::piped())
                              .spawn()
                              .expect("could not run executable");

        let mut bytes = vec![];
        in_file.read_to_end(&mut bytes).unwrap();

        let input = String::from_utf8_lossy(&bytes);
        let trimmed_input = input.trim();
        if let Err(err) = $crate::from_str::<$in_type>(&trimmed_input) {
            panic!("INVALID\n{}\n{}", err, trimmed_input);
        }

        process.stdin.unwrap().write_all(&bytes).unwrap();

        let mut expected = String::new();
        out_file.read_to_string(&mut expected).unwrap();

        let mut actual = String::new();
        process.stdout.unwrap().read_to_string(&mut actual).unwrap();

        let mut error = String::new();
        process.stderr.unwrap().read_to_string(&mut error).unwrap();

        // TODO: Use general compare json function.
        let actual = $crate::remove_whitespace(&actual);
        let expected = $crate::remove_whitespace(&expected);

        println!("{}", error);
        assert_eq!(expected, actual);
    }}
}

pub fn remove_whitespace(string: &str) -> String {
    string.split_whitespace().join("")
}

pub fn from_str<W: serde::de::Deserialize>(bytes: &str) -> serde_json::error::Result<W> {
    serde_json::from_str::<W>(bytes)
}

pub fn expand_each_test(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult> {
    let directory_path = match args[0] {
        TokenTree::Token(_, token::Literal(token::Str_(n), _)) => n.as_str(),
        _ => {
            cx.span_err(sp, "argument should be a static str");
            return DummyResult::any(sp);
        }
    };

    let mut tests = vec![];

    for entry in ::std::fs::read_dir(&*directory_path).unwrap() {
        let filename = entry.unwrap().file_name();
        let filename = filename.to_str().unwrap();
        if filename.contains("-in.json") {
            let name = filename.trim_right_matches("-in.json");
            let fullname = format!("{}/{}", directory_path, name);
            let mut newtt = args.to_owned();
            newtt[0] = TokenTree::Token(
                DUMMY_SP,
                Token::Literal(
                    Str_(gensym(&fullname)),
                    None
                )
            );
            tests.push(make_test_function(cx, &fullname, newtt));
        }
    }

    MacEager::items(SmallVector::many(tests))
}

pub fn make_test_function(cx: &mut ExtCtxt, name: &str, tts: Vec<TokenTree>) -> P<Item> {
    let attr = Spanned {
        span: DUMMY_SP,
        node: Attribute_ {
            id: mk_attr_id(),
            style: AttrStyle::Outer,
            value: P(Spanned {
                span: DUMMY_SP,
                node: MetaWord(InternedString::new("test")),
            }),
            is_sugared_doc: false,
        }
    };

    let body_expr = Expr {
        id: DUMMY_NODE_ID,
        node: ExprMac(Spanned {
            node: Mac_ {
                path: cx.path_ident(DUMMY_SP, Ident::with_empty_ctxt(intern("assert_in_out_file"))),
                tts: tts,
                ctxt: SyntaxContext(0),
            },
            span: DUMMY_SP,
        }),
        span: DUMMY_SP,
        attrs: None,
    };

    let body = Spanned {
        node: StmtSemi(P(body_expr), DUMMY_NODE_ID),
        span: DUMMY_SP,
    };

    let function = ItemFn(
        P(FnDecl {
            inputs: vec![],
            output: DefaultReturn(DUMMY_SP),
            variadic: false,
        }),
        Unsafety::Normal,
        Constness::NotConst,
        Rust,
        Generics {
            lifetimes: vec![],
            ty_params: P::empty(),
            where_clause: WhereClause {
                id: DUMMY_NODE_ID,
                predicates: vec![],
            },
        },
        P(Block {
            stmts: vec![P(body)],
            expr: None,
            id: DUMMY_NODE_ID,
            rules: DefaultBlock,
            span: DUMMY_SP,
        }));

    cx.item(DUMMY_SP, gensym_ident(name), vec![attr], function)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("each_test", expand_each_test);
}
