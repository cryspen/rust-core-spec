use crate::prelude::*;

/// This visitor unwraps unary calls to a given function. This is
/// useful for the `eval` function.
#[derive(Clone, Debug)]
pub(crate) struct DropUnaryCallsTo(pub(crate) syn::Ident);

impl VisitMut for DropUnaryCallsTo {
    fn visit_expr_mut(&mut self, e: &mut syn::Expr) {
        if let syn::Expr::Call(syn::ExprCall { func, args, .. }) = e {
            if let Some(sub_e) = destruct_punctuated_1(args) {
                if destruct_expr_as_ident(func) == Some(self.0.clone()) {
                    *e = sub_e;
                }
            }
        }
        visit_mut::visit_expr_mut(self, e);
    }
}
