mod drop_unary_calls_to;
mod prelude;
mod trim_signatures;
mod utils;

use drop_unary_calls_to::DropUnaryCallsTo;
use prelude::*;
use trim_signatures::trim_generics;

use structmeta::StructMeta;

/// Options the `partial_compute` macro can take
#[derive(Clone, StructMeta, Debug)]
struct Options {
    pub(crate) variables: Bracketed<TerminatedPunctuated<syn::Ident, syn::Token![,]>>,
    /// Shall we keep the existing, full, un-computed function? (default: false)
    pub(crate) keep_full_function: bool,
    /// How the meta-function should be named? (default: original name)
    pub(crate) name: Option<syn::Ident>,
    /// `<eval_function>(..)` nodes are computed (default: `eval`)
    pub(crate) eval_function: Option<syn::Ident>,
}

impl Options {
    pub(crate) fn variables(&self) -> Vec<syn::Ident> {
        self.variables.0 .0.iter().cloned().collect()
    }
    pub(crate) fn has_variable(&self, ident: &syn::Ident) -> bool {
        self.variables.0 .0.iter().any(|i| i == ident)
    }
    pub(crate) fn eval_function(&self) -> syn::Ident {
        self.eval_function.clone().unwrap_or(parse_quote! {eval})
    }
}

/// Represents a `let <lhs> = <rhs>;`
#[derive(Clone, Debug)]
struct Binding {
    lhs: syn::Pat,
    rhs: syn::Expr,
}

/// Represent a hole in an expression
#[derive(Clone, Debug)]
struct Hole {
    id: usize,
    env: Vec<Binding>,
    expr: syn::Expr,
}

impl Binding {
    fn as_expr(&self) -> TokenStream {
        let Binding { lhs, rhs } = self;
        quote! {let #lhs = #rhs}
    }
}
impl Hole {
    fn as_expr(&self) -> TokenStream {
        let env = self.env.iter().map(Binding::as_expr);
        let expr = &self.expr;
        quote! {#(#env;)* #expr}
    }
}

const HOLE_NAME_PREFIX: &str = "___PARTIAL_COMPUTE_HOLE_BEGIN_";
const HOLE_NAME_SUFFIX: &str = "_PARTIAL_COMPUTE_HOLE_END___";

/// Take an expression and a set of variables VARS.
/// Each submode `eval(<E>)` or `<V>` (V \in VARS) will be evaluated
#[derive(Clone, Debug)]
struct PartialCompute {
    options: Options,
    bindings: Vec<Binding>,
    holes: Vec<Hole>,
    fn_sig: Option<syn::Signature>,
}

impl PartialCompute {
    fn new(options: &Options) -> Self {
        PartialCompute {
            options: options.clone(),
            bindings: Vec::new(),
            holes: Vec::new(),
            fn_sig: None,
        }
    }
    fn hole(&mut self, expr: &mut syn::Expr) {
        let id = self.holes.len();
        self.holes.push(Hole {
            id,
            env: self.bindings.clone(),
            expr: expr.clone(),
        });
        let name = format!("{HOLE_NAME_PREFIX}{}{HOLE_NAME_SUFFIX}", id);
        let ident = Ident::new(&name, expr.span());
        *expr = syn::parse_quote! {#ident};
    }
    fn find_hole(&self, id: usize) -> Option<Hole> {
        self.holes.iter().find(|hole| hole.id == id).cloned()
    }
}

impl VisitMut for PartialCompute {
    fn visit_stmt_mut(&mut self, stmt: &mut syn::Stmt) {
        if let syn::Stmt::Local(syn::Local {
            pat,
            init: Some(init),
            ..
        }) = stmt
        {
            self.bindings.push(Binding {
                lhs: pat.clone(),
                rhs: *init.expr.clone(),
            })
        }
        visit_mut::visit_stmt_mut(self, stmt);
    }

    fn visit_expr_mut(&mut self, e: &mut syn::Expr) {
        if let Some(var) = destruct_expr_as_ident(e) {
            if self.options.has_variable(&var) {
                self.hole(e)
            }
        } else if let syn::Expr::Call(syn::ExprCall { func, args, .. }) = e {
            if let Some(sub_e) = destruct_punctuated_1(args) {
                if destruct_expr_as_ident(func) == Some(self.options.eval_function()) {
                    *e = sub_e;
                    self.hole(e);
                }
            }
        } else if let syn::Expr::Macro(r#macro) = e {
            use syn::parse::Parser;
            if let Ok(mut expr) = syn::Expr::parse.parse(r#macro.mac.tokens.clone().into()) {
                self.visit_expr_mut(&mut expr);
                r#macro.mac.tokens = expr.to_token_stream();
            }
        }
        visit_mut::visit_expr_mut(self, e);
    }

    fn visit_signature_mut(&mut self, sig: &mut syn::Signature) {
        let mut detected = true;
        let mut rsig = sig.clone();
        rsig.inputs = Punctuated::new();
        sig.inputs = sig
            .inputs
            .clone()
            .into_iter()
            .filter(|arg| match arg {
                syn::FnArg::Typed(pat)
                    if destruct_pat_as_ident(&pat.pat)
                        .map(|v| self.options.has_variable(&v))
                        .unwrap_or(false) =>
                {
                    detected = true;
                    rsig.inputs.push(arg.clone());
                    false
                }
                _ => true,
            })
            .collect();
        if detected {
            self.fn_sig = Some(rsig);
        }
    }
}

#[derive(Clone, Debug)]
enum Chunk {
    String(String),
    Rust(TokenStream),
}

impl Chunk {
    fn add_env(&mut self, env: &Vec<syn::Ident>) {
        if let Chunk::Rust(ts) = self {
            let env = env
                .into_iter()
                .map(|x| quote! {let #x = #x.clone();})
                .collect::<Vec<_>>();
            *ts = quote! {{#(#env)* #ts}};
        }
    }
}

impl ToTokens for Chunk {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Chunk::String(s) => syn::LitStr::new(s, Span::call_site()).to_tokens(tokens),
            Chunk::Rust(ts) => quote! {{#ts}.print_as_rust()}.to_tokens(tokens),
        }
    }
}

impl Chunk {
    /// Parse `f` a Rust function with holes into a `Chunk`s
    fn parse(f: &str, x: &PartialCompute) -> Option<Vec<Self>> {
        let vec = f
            .split(HOLE_NAME_PREFIX)
            .into_iter()
            .map(
                |chunk| match chunk.split(HOLE_NAME_SUFFIX).collect::<Vec<_>>()[..] {
                    [name, chunk] => {
                        let e = x.find_hole(name.parse().ok()?)?.as_expr();
                        // let e = quote! {#e.print_as_rust()};
                        Some(vec![Self::Rust(e), Self::String(chunk.to_string())])
                    }
                    [chunk] => Some(vec![Self::String(chunk.to_string())]),
                    _ => None,
                },
            )
            .collect::<Option<Vec<Vec<_>>>>()?;
        Some(vec.into_iter().flatten().collect())
    }

    fn as_format_string(chunks: &Vec<Chunk>) -> TokenStream {
        string_literal(
            &chunks
                .iter()
                .map(|chunk| match chunk {
                    Chunk::Rust(_) => "{}",
                    Chunk::String(_) => "{}",
                })
                .collect::<Vec<_>>()[..]
                .join(""),
        )
    }
}

#[proc_macro]
pub fn drop_eval_in_expr(expr: pm::TokenStream) -> pm::TokenStream {
    let mut expr = parse_macro_input!(expr as syn::Expr);
    DropUnaryCallsTo(parse_quote! {eval}).visit_expr_mut(&mut expr);
    quote! {#expr}.into()
}

#[proc_macro]
pub fn drop_eval_in_expr_as_str(expr: pm::TokenStream) -> pm::TokenStream {
    let expr: TokenStream = drop_eval_in_expr(expr).into();
    let dummy_function_sig: syn::Signature = syn::parse_str(DUMMY_FUNCTION_SIG).unwrap();
    let file = parse_quote! {#dummy_function_sig { #expr }};
    let string = prettyplease::unparse(&file);
    let string = string
        .trim()
        .strip_prefix(DUMMY_FUNCTION_SIG)
        .unwrap()
        .strip_prefix(" {")
        .unwrap()
        .strip_suffix("}")
        .unwrap()
        .trim();
    let string = string_literal(string);

    quote! {#string}.into()
}

const DUMMY_FUNCTION_SIG: &str = "fn __DUMMY_PARTIAL_COMPUTE_FN()";

#[proc_macro_error]
#[proc_macro_attribute]
pub fn partial_compute(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let options = parse_macro_input!(attr as Options);
    let mut fun = parse_macro_input!(item as syn::ItemFn);
    let mut original_fun = fun.clone();
    DropUnaryCallsTo(options.eval_function()).visit_item_fn_mut(&mut original_fun);

    let mut visitor = PartialCompute::new(&options);
    visitor.visit_item_fn_mut(&mut fun);
    trim_generics(&mut fun.sig);

    let naked = fun.sig.inputs.is_empty();
    let fun = if naked {
        format!("{DUMMY_FUNCTION_SIG}{}", fun.block.to_token_stream())
    } else {
        format!("{}", fun.to_token_stream())
    };
    let Some(mut chunks) = Chunk::parse(&fun, &visitor) else {
        abort!(Span::call_site(), "Could not parse hole expression");
    };
    {
        let variables = &options.variables();
        chunks.iter_mut().for_each(|chunk| chunk.add_env(variables));
    }

    let format_string_lit = Chunk::as_format_string(&chunks);
    let Some(mut fn_sig) = visitor.fn_sig else {
        abort!(Span::call_site(), "No function was found")
    };
    trim_generics(&mut fn_sig);
    fn_sig.output = parse_quote! {-> String};
    if let Some(name) = options.name {
        fn_sig.ident = name;
    }
    let extra = options.keep_full_function.then_some(original_fun);

    quote! {
        #extra

        #fn_sig {
        {
            let string = format!(#format_string_lit, #(#chunks),*);
            let item_fn = syn::parse_str(&string).unwrap();
            let file = syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![syn::Item::Fn(item_fn)],
            };
            let string = prettyplease::unparse(&file).trim().to_string();
            if #naked {
                string
                    .strip_prefix(#DUMMY_FUNCTION_SIG).unwrap()
                    .strip_prefix(" {").unwrap()
                    .strip_suffix("}").unwrap()
                    .trim()
                    .to_string()
            } else {
                string
            }
        }
    }}
    .into()
}
