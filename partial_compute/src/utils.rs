use crate::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct TerminatedPunctuated<T: Parse, S: Parse>(pub(crate) Punctuated<T, S>);
#[derive(Clone, Debug)]
pub(crate) struct Bracketed<T>(pub(crate) T);

pub(crate) fn destruct_punctuated_1<T: Clone, S>(x: &Punctuated<T, S>) -> Option<T> {
    (x.len() == 1).then(|| x.first().unwrap().clone())
}

pub(crate) fn destruct_path_as_ident(e: &syn::Path) -> Option<syn::Ident> {
    destruct_punctuated_1(&e.segments).map(|s| s.ident)
}

pub(crate) fn destruct_expr_as_ident(e: &syn::Expr) -> Option<syn::Ident> {
    match e {
        syn::Expr::Path(syn::ExprPath {
            qself: None, path, ..
        }) => destruct_path_as_ident(path),
        _ => None,
    }
}

pub(crate) fn destruct_pat_as_ident(p: &syn::Pat) -> Option<syn::Ident> {
    match p {
        syn::Pat::Ident(syn::PatIdent {
            by_ref: None,
            mutability: None,
            ident,
            subpat: None,
            ..
        }) => Some(ident.clone()),
        _ => None,
    }
}

const _: () = {
    impl<T: Parse, S: Parse> Parse for TerminatedPunctuated<T, S> {
        fn parse(input: parse::ParseStream) -> parse::Result<Self> {
            Ok(TerminatedPunctuated(Punctuated::<T, S>::parse_terminated(
                input,
            )?))
        }
    }

    impl<T: Parse> Parse for Bracketed<T> {
        fn parse(input: parse::ParseStream) -> parse::Result<Self> {
            let content;
            syn::bracketed!(content in input);
            Ok(Bracketed(T::parse(&content)?))
        }
    }
};

pub(crate) fn string_literal(s: &str) -> TokenStream {
    syn::LitStr::new(s, Span::call_site()).to_token_stream()
}
