#![doc = include_str!("../README.md")]

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
    /// The "dyn" keyword.
    KDyn = "dyn";
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
        ReceiverAndSelf(ReceiverAndSelf),
        NamedParam(NamedParam),
    }

    struct ReceiverAndSelf {
        _and: And,
        _self: KSelf,
    }

    struct NamedParam {
        ident: Ident,
        _colon: Colon,
        typ: Type,
    }

    struct SimpleType {
        // TODO: add path params
        ident: Ident,
    }

    enum Type {
        WithGenerics(WithGenerics),
        Simple(SimpleType),
    }

    struct WithGenerics {
        typ: SimpleType,
        _lt: Lt,
        params: Vec<GenericParam>,
        _gt: Gt,
    }

    enum GenericParam {
        DynType(DynType),
        Ident(Ident),
    }

    struct DynType {
        _dyn: KDyn,
        params: Box<Type>,
    }
}

impl core::fmt::Display for Function {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "fn {}(", self.name)?;
        write!(f, "{}", self.params.content)?;
        write!(f, ")")?;
        if let Some(ret) = &self.ret {
            write!(f, " -> {}", ret.second)?;
        }
        write!(f, " {{ ... }}")
    }
}

impl core::fmt::Display for Params {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        for param in self.params.0.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", param.value)?;
            first = false;
        }
        Ok(())
    }
}

impl core::fmt::Display for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Param::ReceiverAndSelf(_) => write!(f, "&self"),
            Param::NamedParam(p) => write!(f, "{}: {}", p.ident, p.typ),
        }
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Type::Simple(simple) => write!(f, "{}", simple.ident),
            Type::WithGenerics(with_generics) => {
                write!(f, "{}", with_generics.typ.ident)?;
                if !with_generics.params.is_empty() {
                    write!(f, "<")?;
                    let mut first = true;
                    for param in &with_generics.params {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", param)?;
                        first = false;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
        }
    }
}

impl core::fmt::Display for GenericParam {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GenericParam::Ident(ident) => write!(f, "{}", ident),
            GenericParam::DynType(dyn_type) => write!(f, "{}", dyn_type),
        }
    }
}

impl core::fmt::Display for DynType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "dyn {}", self.params)
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
        write!(&mut code, "{}", f.params.content).unwrap();
        write!(&mut code, ")").unwrap();

        // Handle return type
        if let Some(ret) = &f.ret {
            write!(&mut code, " -> {}", ret.second).unwrap();
        }

        write!(&mut code, ";").unwrap();
    }
    write!(&mut code, "}}").unwrap();

    let mut output_stream = code.into_token_stream();
    output_stream.extend(TokenStream::from(item_clone));
    output_stream.into()
}
