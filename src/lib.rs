//! # autotrait
//!
//! A Rust proc-macro crate that provides the `autotrait` attribute.
//! This allows automatically implementing traits based on a base implementation.

use unsynn::*;

keyword! {
    /// The "pub" keyword.
    KPub = "pub";
    /// The "impl" keyword.
    KImpl = "impl";
    /// The "for" keyword.
    KFor = "for";
    /// The "fn" keyword.
    KFn = "fn";
    /// The "self" keyword.
    KSelf = "self";
}

operator! {
    /// The "->" operator.
    RightArrow = "->";

    /// The "&" operator.
    And = "&";
}

unsynn! {
    struct ImplBlock {
        _impl: KImpl,
        trait_name: Ident,
        _for: KFor,
        typ_name: Ident,
        body: BraceGroupContaining<Vec<Function>>,
    }

    struct Function {
        _fn: KFn,
        name: Ident,
        params: ParenthesisGroupContaining<Params>,
        ret: Option<Cons<RightArrow, Type>>,
        body: BraceGroup,
    }

    struct Params {
        params: CommaDelimitedVec<Param>,
    }

    enum Param {
        AndSelf(Cons<And, KSelf>),
        IdentColon(Cons<Ident, Colon, Type>),
    }

    struct Type {
        name: Ident,
    }
}

#[proc_macro_attribute]
pub fn autotrait(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_clone = item.clone();

    let token_stream = TokenStream::from(item);
    let mut i = token_stream.to_token_iter();
    let b = i.parse::<ImplBlock>().unwrap();

    use std::fmt::Write;
    let mut code = String::new();
    write!(&mut code, "trait {} {{", b.trait_name).unwrap();
    // Add function declarations to the trait based on functions in the implementation
    for f in &b.body.content {
        write!(&mut code, "fn {}(", f.name).unwrap();
        // Handle parameters
        let mut param_strs = Vec::new();
        let params = &f.params.content.params.0;
        for param_delimited in params {
            let param = &param_delimited.value;
            match param {
                Param::AndSelf(_) => param_strs.push("&self".to_string()),
                Param::IdentColon(param) => {
                    let name = &param.first;
                    let typ = &param.third;
                    param_strs.push(format!("{}: {}", name, typ.name));
                }
            }
        }
        write!(&mut code, "{}", param_strs.join(", ")).unwrap();
        write!(&mut code, ")").unwrap();

        // Handle return type
        if let Some(ret) = &f.ret {
            write!(&mut code, " -> {}", ret.second.name).unwrap();
        }

        write!(&mut code, ";").unwrap();
    }
    write!(&mut code, "}}").unwrap();

    let mut output_stream = code.into_token_stream();
    output_stream.extend(TokenStream::from(item_clone));
    output_stream.into()
}
