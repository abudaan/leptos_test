use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Field, Fields, Path, Type};

#[proc_macro_derive(New, attributes(new))]
pub fn derive_new(item: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = parse_macro_input!(item as DeriveInput);

    let new_ident = format_ident!("New{ident}");

    let new_attrs = attrs
        .into_iter()
        .filter_map(|Attribute { path, tokens, .. }| {
            if path.get_ident() == Some(&format_ident!("new")) {
                match tokens
                    .into_iter()
                    .next()
                    .expect("invalid syntax, use `new(skip)`")
                {
                    TokenTree::Group(group) => Some(group.stream()),
                    _ => None,
                }
            } else {
                None
            }
        })
        .map(|tokens| Attribute {
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            bracket_token: Default::default(),
            path: Path {
                leading_colon: None,
                segments: Default::default(),
            },
            tokens,
        });

    let fields = match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(_) => data.fields,
            // TODO: Implement
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(
                "deriving `New` is currently only supported for structs with named fields"
            ),
        },
        Data::Enum(_) | Data::Union(_) => {
            unimplemented!("deriving `New` is only supported for structs")
        }
    };

    // Only include fields that aren't annotated with `#[new(skip)]`.
    let filtered_fields = fields
        .into_iter()
        .filter(|Field { attrs, .. }| !is_skipped(attrs))
        .map(replace_new);

    (quote! {
        #(#new_attrs)*
        #vis struct #new_ident #generics {
            #(#filtered_fields,)*
        }
    })
    .into()
}

fn replace_new(field: Field) -> Field {
    let mut new_attrs = Vec::new();
    let replace = field
        .attrs
        .into_iter()
        .filter_map(|attr| {
            if attr.path.get_ident() == Some(&format_ident!("new")) {
                Some(attr.tokens)
            } else {
                new_attrs.push(attr);
                None
            }
        })
        .find_map(|tokens| match tokens.clone().into_iter().next() {
            Some(TokenTree::Group(group)) => {
                let mut iter = group.stream().into_iter();
                if let Some(proc_macro2::TokenTree::Ident(ident)) = iter.next() {
                    if ident != "use" {
                        return None;
                    }
                }
                if let Some(proc_macro2::TokenTree::Group(g)) = iter.next() {
                    Some(g)
                } else {
                    None
                }
            }
            _ => None,
        });

    if let Some(replace) = replace {
        let new_type = match field.ty {
            Type::Path(_) => {
                let tokens = replace.stream().into_token_stream();
                Type::Path(syn::parse2(tokens).unwrap())
            }
            _ => todo!("Not implemented"),
        };
        return Field {
            ty: new_type,
            attrs: new_attrs,
            ..field
        };
    }
    Field {
        attrs: new_attrs,
        ..field
    }
}

fn is_skipped(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter_map(|Attribute { path, tokens, .. }| {
            if path.get_ident() == Some(&format_ident!("new")) {
                Some(tokens)
            } else {
                None
            }
        })
        .any(|tokens| match tokens.clone().into_iter().next() {
            Some(TokenTree::Group(group)) => group.stream().into_iter().any(|args| match args {
                TokenTree::Ident(arg) => arg == "skip",
                _ => false,
            }),
            _ => false,
        })
}
