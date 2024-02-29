use crate::prelude::*;
pub(crate) use syn::visit::Visit;

#[derive(Debug, Clone, Default)]
struct IdentCollector {
    idents: HashSet<syn::Ident>,
}

impl<'a> Visit<'a> for IdentCollector {
    fn visit_path(&mut self, path: &'a syn::Path) {
        if let Some(ident) = super::destruct_path_as_ident(path) {
            self.idents.insert(ident);
        };
        syn::visit::visit_path(self, path);
    }
}

pub(crate) fn trim_generics(sig: &mut syn::Signature) {
    let idents = {
        let mut visitor = IdentCollector::default();
        let mut sig = sig.clone();
        sig.generics.params = Punctuated::new();
        sig.generics.where_clause = None;
        visitor.visit_signature(&sig);
        visitor.idents
    };

    let gparams = sig.generics.params.clone();
    sig.generics.params = Punctuated::new();
    for gparam in &gparams {
        match gparam {
            syn::GenericParam::Const(syn::ConstParam { ident, .. })
            | syn::GenericParam::Type(syn::TypeParam { ident, .. })
                if idents.contains(&ident) =>
            {
                sig.generics.params.push(gparam.clone())
            }
            _ => (),
        }
    }
}
