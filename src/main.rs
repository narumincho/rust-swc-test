fn main() {
    println!("Hello, world!");
}

#[test]
fn code_to_tree() {
    let code = "
/** アカウント */
export type Account = {
    /** アカウントを識別するID */
    readonly id: string;
    /** アカウント名 */
    readonly name: string;
}
";
    let comments = swc_common::comments::SingleThreadedComments::default();
    let lexer = swc_ecma_parser::lexer::Lexer::new(
        swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
            tsx: false,
            decorators: false,
            dts: false,
            no_early_errors: true,
            disallow_ambiguous_jsx_like: false,
        }),
        swc_ecma_ast::EsVersion::Es2022,
        swc_ecma_parser::StringInput::new(
            code,
            swc_common::source_map::BytePos(0),
            swc_common::source_map::BytePos((code.as_bytes().len() - 1) as u32),
        ),
        Some(&comments),
    );
    let mut parser = swc_ecma_parser::Parser::new_from(lexer);
    let tree_result = parser.parse_typescript_module();
    let module = tree_result.expect("パースに失敗");
    println!("{:#?}", module);

    let result: Vec<TypeData> = module
        .body
        .iter()
        .filter_map(|module_item| match module_item {
            swc_ecma_ast::ModuleItem::ModuleDecl(module_decl) => match module_decl {
                swc_ecma_ast::ModuleDecl::ExportDecl(export_decl) => {
                    let comment = comments.with_leading(export_decl.span.lo, |comments| {
                        comments
                            .iter()
                            .map(|comment| comment.text.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    });
                    match &export_decl.decl {
                        swc_ecma_ast::Decl::TsTypeAlias(type_alias) => {
                            match &*type_alias.type_ann {
                                swc_ecma_ast::TsType::TsTypeLit(ts_type_list) => {
                                    Some(TypeData {
                                        name: type_alias.id.to_id().0.to_string(),
                                        comment,
                                        members: ts_type_list
                                            .members
                                            .iter()
                                            .filter_map(|member| {
                                                match member {
                                        swc_ecma_ast::TsTypeElement::TsPropertySignature(
                                            property_signature,
                                        ) => match &*property_signature.key {
                                            swc_ecma_ast::Expr::Ident(ident) => Some(MemberData {
                                                name: ident.sym.to_string(),
                                                comment: comments.with_leading(
                                                    property_signature.span.lo,
                                                    |comments| {
                                                        comments
                                                            .iter()
                                                            .map(|comment| comment.text.to_string())
                                                            .collect::<Vec<String>>()
                                                            .join("\n")
                                                    },
                                                ),
                                            }),
                                            _ => None,
                                        },
                                        _ => None,
                                    }
                                            })
                                            .collect(),
                                    })
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            swc_ecma_ast::ModuleItem::Stmt(_) => None,
        })
        .collect();

    println!("{:#?}", result);
}

#[derive(Debug)]
struct TypeData {
    name: String,
    comment: String,
    members: Vec<MemberData>,
}

#[derive(Debug)]
struct MemberData {
    name: String,
    comment: String,
}

#[test]
fn tree_to_code() {
    let comments = swc_common::comments::SingleThreadedComments::default();

    let account_comment_byte_pos = swc_common::source_map::BytePos(1);

    swc_common::comments::Comments::add_leading(
        &comments,
        account_comment_byte_pos,
        swc_common::comments::Comment {
            span: swc_common::DUMMY_SP,
            kind: swc_common::comments::CommentKind::Block,
            text: swc_atoms::Atom::from("アカウント"),
        },
    );

    let account_id_comment_byte_pos = swc_common::source_map::BytePos(2);

    swc_common::comments::Comments::add_leading(
        &comments,
        account_id_comment_byte_pos,
        swc_common::comments::Comment {
            span: swc_common::DUMMY_SP,
            kind: swc_common::comments::CommentKind::Block,
            text: swc_atoms::Atom::from("アカウントを識別するID"),
        },
    );

    let account_name_comment_byte_pos = swc_common::source_map::BytePos(3);

    swc_common::comments::Comments::add_leading(
        &comments,
        account_name_comment_byte_pos,
        swc_common::comments::Comment {
            span: swc_common::DUMMY_SP,
            kind: swc_common::comments::CommentKind::Block,
            text: swc_atoms::Atom::from("アカウント名"),
        },
    );

    let module = swc_ecma_ast::TsModuleBlock {
        span: swc_common::Span::default(),
        body: vec![swc_ecma_ast::ModuleItem::ModuleDecl(
            swc_ecma_ast::ModuleDecl::ExportDecl(swc_ecma_ast::ExportDecl {
                span: swc_common::Spanned::span(&account_comment_byte_pos),
                decl: swc_ecma_ast::Decl::TsTypeAlias(Box::new(swc_ecma_ast::TsTypeAliasDecl {
                    id: swc_ecma_ast::Ident::new(
                        string_cache::Atom::from("Account"),
                        swc_common::Span::default(),
                    ),
                    declare: false,
                    span: swc_common::Span::default(),
                    type_ann: Box::new(swc_ecma_ast::TsType::TsTypeLit(swc_ecma_ast::TsTypeLit {
                        span: swc_common::Span::default(),
                        members: vec![
                            swc_ecma_ast::TsTypeElement::TsPropertySignature(
                                swc_ecma_ast::TsPropertySignature {
                                    span: swc_common::Spanned::span(&account_id_comment_byte_pos),
                                    readonly: true,
                                    key: Box::new(swc_ecma_ast::Expr::Ident(swc_ecma_ast::Ident {
                                        span: swc_common::Span::default(),
                                        sym: string_cache::Atom::from("id"),
                                        optional: false,
                                    })),
                                    computed: false,
                                    optional: false,
                                    init: None,
                                    params: vec![],
                                    type_ann: Some(Box::new(swc_ecma_ast::TsTypeAnn {
                                        span: swc_common::Span::default(),
                                        type_ann: Box::new(swc_ecma_ast::TsType::TsKeywordType(
                                            swc_ecma_ast::TsKeywordType {
                                                span: swc_common::Span::default(),
                                                kind:
                                                    swc_ecma_ast::TsKeywordTypeKind::TsStringKeyword,
                                            },
                                        )),
                                    })),
                                    type_params: None,
                                },
                            ),
                            swc_ecma_ast::TsTypeElement::TsPropertySignature(
                                swc_ecma_ast::TsPropertySignature {
                                    span: swc_common::Spanned::span(&account_name_comment_byte_pos),
                                    readonly: true,
                                    key: Box::new(swc_ecma_ast::Expr::Ident(swc_ecma_ast::Ident {
                                        span: swc_common::Span::default(),
                                        sym: string_cache::Atom::from("name"),
                                        optional: false,
                                    })),
                                    computed: false,
                                    optional: false,
                                    init: None,
                                    params: vec![],
                                    type_ann: Some(Box::new(swc_ecma_ast::TsTypeAnn {
                                        span: swc_common::Span::default(),
                                        type_ann: Box::new(swc_ecma_ast::TsType::TsKeywordType(
                                            swc_ecma_ast::TsKeywordType {
                                                span: swc_common::Span::default(),
                                                kind:
                                                    swc_ecma_ast::TsKeywordTypeKind::TsStringKeyword,
                                            },
                                        )),
                                    })),
                                    type_params: None,
                                },
                            ),
                        ],
                    })),
                    type_params: None,
                })),
            }),
        )],
    };

    let cm = swc_common::sync::Lrc::<swc_common::SourceMap>::default();
    let mut buf = vec![];
    let writer = swc_ecma_codegen::text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None);

    let mut emitter = swc_ecma_codegen::Emitter {
        cfg: Default::default(),
        comments: Some(&comments),
        cm: cm.clone(),
        wr: writer,
    };

    swc_ecma_codegen::Node::emit_with(&module, &mut emitter).expect("コードの生成が失敗した");

    let code = String::from_utf8(buf).expect("UTF8として解釈できないコードを生成した");

    println!("{}", code);
}
