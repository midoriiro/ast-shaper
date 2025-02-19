use crate::utils::create_ident;
use crate::utils::path::Path;
use crate::utils::punctuated::PunctuatedExt;
use quote::ToTokens;
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Block, ExprAssign, ExprBlock, ExprCall, ExprClosure, ExprField, ExprIf, ExprLet, ExprLit, ExprMacro, ExprMethodCall, ExprPath, ExprReference, ExprStruct, FieldValue, Lit, LitStr, Local, LocalInit, Macro, MacroDelimiter, Member, Pat, PatIdent, PatTupleStruct, PatType, ReturnType, Stmt, StmtMacro, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Expr(syn::Expr),
    Stmt(Stmt),
    Path(Path),
}

impl Expr {
    pub fn to_expr(&self) -> syn::Expr {
        match self {
            Expr::Expr(value) => value.clone(),
            Expr::Stmt(value) => {
                match value {
                    Stmt::Expr(value, _) => value.clone(),
                    Stmt::Macro(value) => {
                        syn::Expr::Macro(ExprMacro {
                            attrs: value.attrs.clone(),
                            mac: value.mac.clone(),
                        })
                    }
                    _ => panic!("Statement is not an expression")
                }
            }
            Expr::Path(value) => syn::Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: value.to_syn_path(),
            }),
        }
    }
    
    pub fn to_stmt(&self) -> Stmt {
        match self {
            Expr::Stmt(value) => value.clone(),
            _ => panic!("Expression is not a statement")
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprMethodChainCall {
    Start {
        receiver: Expr,
        method: Path,
        arguments: Vec<Expr>
    },
    Chained {
        method: Path,
        arguments: Vec<Expr>
    }
}

pub struct Statement;

impl Statement {
    pub fn access_field(receiver: Path, member: Path) -> Stmt {
        Stmt::Expr(
            syn::Expr::Field(ExprField {
                attrs: vec![],
                base: Box::new(syn::Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: receiver.to_syn_path(),
                })),
                dot_token: Default::default(),
                member: Member::Named(member.last().unwrap().ident.clone()),
            }),
            None,
        )
    }

    pub fn access_field_as_ref(receiver: Path, member: Path) -> Stmt {
        Stmt::Expr(
            syn::Expr::Field(ExprField {
                attrs: vec![],
                base: Box::new(syn::Expr::Reference(ExprReference {
                    attrs: vec![],
                    and_token: Default::default(),
                    mutability: None,
                    expr: Box::new(syn::Expr::Path(ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: receiver.to_syn_path(),
                    })),
                })),
                dot_token: Default::default(),
                member: Member::Named(member.last().unwrap().ident.clone()),
            }),
            None,
        )
    }
    
    pub fn assign_field(
        receiver: Path,
        member: Path,
        right: Expr
    ) -> Stmt {
        Stmt::Expr(
            syn::Expr::Assign(ExprAssign {
                attrs: vec![],
                left: Box::new(Expr::Stmt(Self::access_field(receiver, member)).to_expr()),
                eq_token: Default::default(),
                right: Box::new(right.to_expr()),
            }),
            Some(Default::default())
        )
    }
    
    pub fn implicit_return(expr: Expr) -> Stmt {
        Stmt::Expr(
            expr.to_expr(),
            None
        )
    }

    pub fn let_some_condition(condition: Expr, value: Path, then: Vec<Stmt>, or: Option<Expr>) -> Stmt {
        Stmt::Expr(
            syn::Expr::If(ExprIf {
                attrs: vec![],
                if_token: Default::default(),
                cond: Box::new(syn::Expr::Let(ExprLet {
                    attrs: vec![],
                    let_token: Default::default(),
                    pat: Box::new(Pat::TupleStruct(PatTupleStruct {
                        attrs: vec![],
                        qself: None,
                        path: Path::new("Some").to_syn_path(),
                        paren_token: Default::default(),
                        elems: Punctuated::single(Pat::Ident(PatIdent {
                            attrs: vec![],
                            by_ref: None,
                            mutability: None,
                            ident: create_ident(value.to_string()),
                            subpat: None,
                        })),
                    })),
                    eq_token: Default::default(),
                    expr: Box::new(condition.to_expr()),
                })),
                then_branch: Block {
                    brace_token: Default::default(),
                    stmts: then,
                },
                else_branch: match or {
                    Some(value) => Some((Default::default(), Box::new(value.to_expr()))),
                    None => None
                },
            }),
            None
        )
    }
    
    pub fn let_none_condition(path: Expr, then: Vec<Stmt>) -> Stmt {
        Stmt::Expr(
            syn::Expr::If(ExprIf {
                attrs: vec![],
                if_token: Default::default(),
                cond: Box::new(syn::Expr::Let(ExprLet {
                    attrs: vec![],
                    let_token: Default::default(),
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident: create_ident("None"),
                        subpat: None,
                    })),
                    eq_token: Default::default(),
                    expr: Box::new(path.to_expr()),
                })),
                then_branch: Block {
                    brace_token: Default::default(),
                    stmts: then,
                },
                else_branch: None,
            }),
            None
        )
    }
    
    pub fn call(path: Path, arguments: Vec<Expr>) -> Stmt {
        let arguments = arguments.iter()
            .map(|argument| argument.to_expr())
            .collect::<Punctuated<syn::Expr, Token![,]>>();
        Stmt::Expr(
            syn::Expr::Call(ExprCall {
                attrs: vec![],
                func: Box::new(syn::Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: path.to_syn_path(),
                })),
                paren_token: Default::default(),
                args: Punctuated::from_iter(arguments),
            }),
            Some(Default::default())
        )
    }
    
    pub fn method_call(
        receiver: Expr,
        method: Path,
        arguments: Vec<Expr>
    ) -> Stmt {
        let arguments = arguments.iter()
            .map(|argument| argument.to_expr())
            .collect::<Punctuated<syn::Expr, Token![,]>>();
        Stmt::Expr(
            syn::Expr::MethodCall(ExprMethodCall {
                attrs: vec![],
                receiver: Box::new(receiver.to_expr()),
                dot_token: Default::default(),
                method: method.last().unwrap().ident.clone(),
                turbofish: None,
                paren_token: Default::default(),
                args: arguments,
            }),
            Some(Default::default())
        )
    }

    pub fn method_chain_call(
        calls: Vec<ExprMethodChainCall>
    ) -> Stmt {
        if calls.len() <= 1 {
            panic!("Chaining calls require to have at least two calls");
        }
        let mut last_call = None;
        for call in calls {
            if let None = last_call {
                match call {
                    ExprMethodChainCall::Start { receiver, method, arguments } => {
                        last_call = Some(Expr::Stmt(Statement::method_call(
                            receiver,
                            method,
                            arguments,
                        )));
                        continue
                    }
                    _ => panic!("Expected 'Start' call")
                }
            }
            let receiver = last_call.unwrap();
            let call = match call {
                ExprMethodChainCall::Chained { method, arguments } => {
                    Statement::method_call(
                        receiver,
                        method,
                        arguments
                    )
                }
                _ => panic!("Expected 'Chained' call")
            };  
            last_call = Some(Expr::Stmt(call));
        }
        last_call.unwrap().to_stmt()
    }
    
    pub fn struct_literal(path: Path, fields: HashMap<String, Expr>) -> Stmt {
        let fields = fields.iter()
            .map(|(ident, value)| {
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(create_ident(ident)),
                    colon_token: Some(Default::default()),
                    expr: value.to_expr(),
                }
            })
            .collect::<Punctuated<FieldValue, Token![,]>>();
        Stmt::Expr(
            syn::Expr::Struct(ExprStruct {
                attrs: vec![],
                qself: None,
                path: path.to_syn_path(),
                brace_token: Default::default(),
                fields,
                dot2_token: None,
                rest: None,
            }),
            Some(Default::default())
        )
    }
    
    pub fn assign(left: Expr, right: Expr) -> Stmt {
        Stmt::Expr(
            syn::Expr::Assign(ExprAssign {
                attrs: vec![],
                left: Box::new(left.to_expr()),
                eq_token: Default::default(),
                right: Box::new(right.to_expr()),
            }),
            Some(Default::default())
        )
    }

    pub fn let_assign(name: Path, right: Expr) -> Stmt {
        Stmt::Local(Local {
            attrs: vec![],
            let_token: Default::default(),
            pat: Pat::Ident(PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: create_ident(name.to_string()),
                subpat: None,
            }),
            init: Some(LocalInit {
                eq_token: Default::default(),
                expr: Box::new(right.to_expr()),
                diverge: None,
            }),
            semi_token: Default::default(),
        })
    }
    
    pub fn macro_invocation(name: Path, delimiter: MacroDelimiter, arguments: Vec<Expr>) -> Stmt {
        let arguments: Punctuated<syn::Expr, Token![,]> = Punctuated::from_iter(arguments.iter().map(|argument| argument.to_expr()));
        Stmt::Expr(
            syn::Expr::Block(ExprBlock {
                attrs: vec![],
                label: None,
                block: Block {
                    brace_token: Default::default(),
                    stmts: vec![
                        Stmt::Macro(StmtMacro {
                            attrs: vec![],
                            mac: Macro {
                                path: name.to_syn_path(),
                                bang_token: Default::default(),
                                delimiter,
                                tokens: arguments.to_token_stream(),
                            },
                            semi_token: Some(Default::default()),
                        })
                    ],
                },
            }),
            Some(Default::default())
        )
    }
    
    pub fn panic(format: String, mut arguments: Vec<Expr>) -> Stmt {
        let mut arguments_to_pass = vec![
            Expr::Expr(syn::Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Str(LitStr::new(format.as_str(), format.span())),
            })),
        ];
        arguments_to_pass.append(&mut arguments);
        Self::macro_invocation(
            Path::new("panic"),
            MacroDelimiter::Paren(Default::default()),
            arguments_to_pass
        )
    }
    
    pub fn block(statements: Vec<Stmt>) -> Stmt {
        Stmt::Expr(
            syn::Expr::Block(ExprBlock {
                attrs: vec![],
                label: None,
                block: Block {
                    brace_token: Default::default(),
                    stmts: statements,
                },
            }),
            Some(Default::default())
        )
    }

    pub fn path(path: Path) -> Stmt {
        Stmt::Expr(
            syn::Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: path.to_syn_path(),
            }),
            Some(Default::default())
        )
    }
    
    pub fn without_trailling_semi_colon(statement: Stmt) -> Stmt {
        let expression = match statement {
            Stmt::Expr(value, _) => value,
            _ => panic!("Expected expression statement")
        };
        Stmt::Expr(
            expression,
            None
        )
    }
    
    pub fn closure(
        arguments: 
        Vec<Pat>, 
        return_type: ReturnType, 
        body: Expr
    ) -> Stmt {
        let arguments = arguments.iter()
            .map(|argument| argument.clone())
            .collect::<Punctuated<Pat, Token![,]>>();
        Stmt::Expr(
            syn::Expr::Closure(ExprClosure {
                attrs: vec![],
                lifetimes: None,
                constness: None,
                movability: None,
                asyncness: None,
                capture: None,
                or1_token: Default::default(),
                inputs: arguments,
                or2_token: Default::default(),
                output: return_type,
                body: Box::new(body.to_expr()),
            }),
            Some(Default::default())
        )
    }
}