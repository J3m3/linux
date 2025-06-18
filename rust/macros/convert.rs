// SPDX-License-Identifier: GPL-2.0

use proc_macro::{Delimiter, Ident, Span, TokenStream, TokenTree};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    // Skip until the `enum` keyword, including the `enum` itself.
    for tt in tokens.by_ref() {
        if matches!(tt, TokenTree::Ident(ident) if ident.to_string() == "enum") {
            break;
        }
    }

    let Some(TokenTree::Ident(enum_ident)) = tokens.next() else {
        return "::core::compile_error!(\"`#[derive(FromPrimitive)]` can only \
                be applied to an enum\");"
            .parse::<TokenStream>()
            .unwrap();
    };

    let mut errs = TokenStream::new();

    if matches!(tokens.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '<') {
        errs.extend(
            "::core::compile_error!(\"`#[derive(FromPrimitive)]` \
                    does not support enums with generic parameters\");"
                .parse::<TokenStream>()
                .unwrap(),
        );
    }

    let variants_group = tokens
        .find_map(|tt| match tt {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => Some(g),
            _ => None,
        })
        .expect("Missing main body of an enum");

    let mut variant_idents: Vec<Ident> = vec![];
    let mut variant_tokens = variants_group.stream().into_iter().peekable();
    while let Some(tt) = variant_tokens.next() {
        // Skip attributes like #[...] if present.
        if matches!(&tt, TokenTree::Punct(p) if p.as_char() == '#') {
            // Skip the attribute group.
            variant_tokens.next();
            continue;
        }

        let TokenTree::Ident(ident) = tt else {
            panic!("Missing enum variant identifier");
        };

        // Reject tuple-like or struct-like variants.
        if let Some(TokenTree::Group(g)) = variant_tokens.peek() {
            let variant_kind = match g.delimiter() {
                Delimiter::Brace => "struct-like",
                Delimiter::Parenthesis => "tuple-like",
                _ => panic!("Invalid enum variant syntax"),
            };
            errs.extend(
                format!(
                    "::core::compile_error!(\"`#[derive(FromPrimitive)]` does not \
                    support {variant_kind} variant `{enum_ident}::{ident}`; \
                    only unit variants are allowed\");"
                )
                .parse::<TokenStream>()
                .unwrap(),
            );
        }

        variant_idents.push(ident);

        // Skip over the rest of the variant definition (e.g., explicit discriminants).
        // We only collect the variant identifiers; their values are later obtained
        // via `as` casting.
        for tt in variant_tokens.by_ref() {
            if matches!(&tt, TokenTree::Punct(p) if p.as_char() == ',') {
                break;
            }
        }
    }

    if !errs.is_empty() {
        return errs;
    }

    // Implement only for `i64` and `u64`; other types delegate to these via
    // default implementations.
    let types = ["i64", "u64"];
    let methods: TokenStream = types
        .into_iter()
        .map(|ty| {
            let ty_ident = Ident::new(ty, Span::mixed_site());
            let fn_name = Ident::new(&format!("from_{ty}"), Span::mixed_site());
            let fn_body = build_fn_body(&variant_idents, &ty_ident);
            quote! {
                #[inline]
                fn #fn_name(n: #ty_ident) -> ::core::option::Option<Self> {
                    #fn_body
                }
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl FromPrimitive for #enum_ident {
            #methods
        }
    }
}

fn build_fn_body(variant_idents: &[Ident], ty: &Ident) -> TokenStream {
    let mut fn_body = TokenStream::new();

    for ident in variant_idents {
        fn_body.extend(quote! {
            if n == Self::#ident as #ty {
                ::core::option::Option::Some(Self::#ident)
            } else
        });
    }
    fn_body.extend(quote! {
        { ::core::option::Option::None }
    });

    fn_body
}
