#![doc = include_str!("../README.md")]

use quote::{TokenStreamExt as _, format_ident, quote};
use unsynn::*;

#[derive(Clone)]
struct LifetimeName(pub Ident);

impl quote::ToTokens for LifetimeName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let punct = TokenTree::Punct(Punct::new('\'', Spacing::Joint));
        let name = &self.0;
        tokens.append(punct);
        quote::ToTokens::to_tokens(&name, tokens);
    }
}

keyword! {
    /// The "pub" keyword.
    KPub = "pub";
    /// The "impl" keyword.
    KImpl = "impl";
    /// The "for" keyword.
    KFor = "for";
    /// The "async" keyword
    KAsync = "async";
    /// The "fn" keyword.
    KFn = "fn";
    /// The "self" keyword.
    KSelf = "self";
    /// The "dyn" keyword.
    KDyn = "dyn";
    /// The "mut" keyword.
    KMut = "mut";

    CapitalFn = "Fn";
    CapitalFnMut = "FnMut";
    CapitalFnOnce = "FnOnce";

    KUpperSend = "Send";
    KUpperSync = "Sync";
}

operator! {
    /// The "->" operator.
    RightArrow = "->";

    /// The "&" operator.
    And = "&";

    /// The "'" operator.
    SingleQuote = "'";

    /// The "#" operator.
    Pound = "#";

    /// The "::" operator.
    DoubleColon = "::";

    /// The "(" operator.
    LeftParen = "(";

    /// The ")" operator.
    RightParen = ")";

    /// The "!" operator
    Not = "!";
}

unsynn! {
    struct AttrBounds {
        bounds: CommaDelimitedVec<AttrBound>
    }

    enum AttrBound {
        NotSend(Cons<Not, KUpperSend>),
        NotSync(Cons<Not, KUpperSync>),
    }

    struct ImplBlock {
        attrs: Vec<Attr>,
        _impl: KImpl,
        trait_name: Ident,
        _for: KFor,
        typ_name: Ident,
        body: BraceGroupContaining<Vec<Function>>,
    }

    struct Function {
        attrs: Vec<Attr>,
        _async: Option<KAsync>,
        _fn: KFn,
        name: Ident,
        generics: Option<FunctionGenericParams>,
        params: ParenthesisGroupContaining<Params>,
        ret: Option<Cons<RightArrow, Type>>,
        body: BraceGroup,
    }

    struct FunctionGenericParams {
        _lt: Lt,
        params: CommaDelimitedVec<GenericParam>,
        _gt: Gt,
    }

    struct Attr {
        _hash: Pound,
        group: BracketGroup,
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
        lifetime: Option<Lifetime>,
        _mut: Option<KMut>,
        _self: KSelf,
    }

    struct NamedParam {
        _mut: Option<KMut>,
        ident: Ident,
        _colon: Colon,
        typ: Type,
    }

    struct SimpleType {
        ident: DelimitedVec<Ident, DoubleColon>,
    }

    enum Type {
        DynTrait(DynTrait),
        ImplTrait(ImplTrait),
        WithGenerics(WithGenerics),
        Slice(Slice),
        Reference(Reference),
        Tuple(TupleType),
        Fn(FnType),
        Simple(SimpleType),
    }

    struct DynTrait {
        _dyn: KDyn,
        traits: DelimitedVec<Box<TypeOrLifetime>, Plus>,
    }

    enum TypeOrLifetime {
        Lifetime(Lifetime),
        Type(Type),
    }

    struct ImplTrait {
        _impl: KImpl,
        traits: DelimitedVec<Box<Type>, Plus>,
    }

    struct FnType {
        _fn: FnTypeWord,
        params: ParenthesisGroupContaining<CommaDelimitedVec<Type>>,
        ret: Option<Cons<RightArrow, Box<Type>>>,
    }

    enum FnTypeWord {
        CapitalFn(CapitalFn),
        CapitalFnMut(CapitalFnMut),
        CapitalFnOnce(CapitalFnOnce),
    }

    struct TupleType {
        types: ParenthesisGroupContaining<CommaDelimitedVec<Type>>,
    }

    struct Slice {
        _and: And,
        lifetime: Option<Lifetime>,
        element_type: BracketGroupContaining<Box<Type>>,
    }

    struct Reference {
        _and: And,
        lifetime: Option<Lifetime>,
        _mut: Option<KMut>,
        typ: Box<Type>,
    }

    struct WithGenerics {
        typ: SimpleType,
        _lt: Lt,
        params: CommaDelimitedVec<GenericParam>,
        _gt: Gt,
    }

    enum GenericParam {
        Lifetime(Lifetime),
        Type(Box<Type>),
    }

    struct Lifetime {
        _lifetime: SingleQuote,
        ident: Ident,
    }
}

impl quote::ToTokens for Function {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        let generics = if let Some(generics) = &self.generics {
            let params = generics.params.0.iter().map(|param| &param.value);
            quote! { < #(#params),* > }
        } else {
            quote! {}
        };

        let params = &self.params.content;

        let return_type = if let Some(ret) = &self.ret {
            let ret_type = &ret.second;
            quote! { -> #ret_type }
        } else {
            quote! {}
        };

        quote::ToTokens::to_tokens(
            &quote! {
                fn #name #generics (#params) #return_type { ... }
            },
            tokens,
        );
    }
}

impl quote::ToTokens for Params {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let params = self.params.0.iter().map(|param| &param.value);
        quote::ToTokens::to_tokens(
            &quote! {
                #(#params),*
            },
            tokens,
        );
    }
}

impl quote::ToTokens for TypeOrLifetime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeOrLifetime::Type(typ) => quote::ToTokens::to_tokens(&typ, tokens),
            TypeOrLifetime::Lifetime(lifetime) => {
                LifetimeName(lifetime.ident.clone()).to_tokens(tokens);
            }
        }
    }
}

impl quote::ToTokens for Param {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Param::ReceiverAndSelf(r) => {
                let lifetime = if let Some(lifetime) = &r.lifetime {
                    let lifetime_token = LifetimeName(lifetime.ident.clone());
                    quote! { #lifetime_token }
                } else {
                    quote! {}
                };

                let mutability = if r._mut.is_some() {
                    quote! { mut }
                } else {
                    quote! {}
                };

                quote::ToTokens::to_tokens(&quote! { &#lifetime #mutability self }, tokens);
            }
            Param::NamedParam(p) => {
                let ident = &p.ident;
                let typ = &p.typ;
                quote::ToTokens::to_tokens(&quote! { #ident: #typ }, tokens);
            }
        }
    }
}

impl quote::ToTokens for SimpleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let idents = self.ident.0.iter().map(|ident| &ident.value);
        quote::ToTokens::to_tokens(
            &quote! {
                #(#idents)::*
            },
            tokens,
        );
    }
}

impl quote::ToTokens for DynTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let traits = self.traits.0.iter().map(|trait_type| &trait_type.value);
        quote::ToTokens::to_tokens(
            &quote! {
                dyn #(#traits)+*
            },
            tokens,
        );
    }
}

impl quote::ToTokens for ImplTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let traits = self.traits.0.iter().map(|trait_type| &trait_type.value);
        quote::ToTokens::to_tokens(
            &quote! {
                impl #(#traits)+*
            },
            tokens,
        );
    }
}

impl quote::ToTokens for Reference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lifetime = if let Some(lifetime) = &self.lifetime {
            let lifetime_token = LifetimeName(lifetime.ident.clone());
            quote! { #lifetime_token }
        } else {
            quote! {}
        };

        let mutability = if self._mut.is_some() {
            quote! { mut }
        } else {
            quote! {}
        };

        let typ = &self.typ;

        quote::ToTokens::to_tokens(
            &quote! {
                &#lifetime #mutability #typ
            },
            tokens,
        );
    }
}

impl quote::ToTokens for WithGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let typ = &self.typ;

        if self.params.0.is_empty() {
            quote::ToTokens::to_tokens(&typ, tokens);
        } else {
            let params = self.params.0.iter().map(|param| &param.value);
            quote::ToTokens::to_tokens(
                &quote! {
                    #typ < #(#params),* >
                },
                tokens,
            );
        }
    }
}

impl quote::ToTokens for TupleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let types = self.types.content.0.iter().map(|typ| &typ.value);
        quote::ToTokens::to_tokens(
            &quote! {
                ( #(#types),* )
            },
            tokens,
        );
    }
}

impl quote::ToTokens for Slice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lifetime = if let Some(lifetime) = &self.lifetime {
            let lifetime_token = LifetimeName(lifetime.ident.clone());
            quote! { #lifetime_token }
        } else {
            quote! {}
        };

        let element_type = &self.element_type.content;

        quote::ToTokens::to_tokens(
            &quote! {
                &#lifetime [#element_type]
            },
            tokens,
        );
    }
}

impl quote::ToTokens for FnType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fn_type = match &self._fn {
            FnTypeWord::CapitalFn(_) => format_ident!("Fn"),
            FnTypeWord::CapitalFnMut(_) => format_ident!("FnMut"),
            FnTypeWord::CapitalFnOnce(_) => format_ident!("FnOnce"),
        };

        let params = self.params.content.0.iter().map(|param| &param.value);

        let return_type = if let Some(ret) = &self.ret {
            let ret_type = &ret.second;
            quote! { -> #ret_type }
        } else {
            quote! {}
        };

        quote::ToTokens::to_tokens(
            &quote! {
                #fn_type ( #(#params),* ) #return_type
            },
            tokens,
        );
    }
}

impl quote::ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Type::Simple(simple) => quote::ToTokens::to_tokens(simple, tokens),
            Type::Reference(reference) => quote::ToTokens::to_tokens(reference, tokens),
            Type::WithGenerics(with_generics) => quote::ToTokens::to_tokens(with_generics, tokens),
            Type::Tuple(tuple) => quote::ToTokens::to_tokens(tuple, tokens),
            Type::Slice(slice) => quote::ToTokens::to_tokens(slice, tokens),
            Type::Fn(fn_type) => quote::ToTokens::to_tokens(fn_type, tokens),
            Type::DynTrait(dyn_trait) => quote::ToTokens::to_tokens(dyn_trait, tokens),
            Type::ImplTrait(impl_trait) => quote::ToTokens::to_tokens(impl_trait, tokens),
        }
    }
}

impl quote::ToTokens for GenericParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            GenericParam::Type(typ) => quote::ToTokens::to_tokens(&typ, tokens),
            GenericParam::Lifetime(lifetime) => {
                let lifetime_token = LifetimeName(lifetime.ident.clone());
                lifetime_token.to_tokens(tokens);
            }
        }
    }
}

#[proc_macro_attribute]
pub fn autotrait(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_clone = item.clone();

    let token_stream = TokenStream::from(item);
    let mut i = token_stream.to_token_iter();
    let b = i.parse::<ImplBlock>().unwrap();

    let attr_bounds = TokenStream::from(attr)
        .to_token_iter()
        .parse::<AttrBounds>()
        .expect("Failed to parse attribute bounds");

    let mut has_not_send = false;
    let mut has_not_sync = false;

    for bound in &attr_bounds.bounds.0 {
        match &bound.value {
            AttrBound::NotSend(_) => has_not_send = true,
            AttrBound::NotSync(_) => has_not_sync = true,
        }
    }

    let bounds = if has_not_send {
        quote! {}
    } else if has_not_sync {
        quote! { : Send }
    } else {
        quote! { : Send + Sync }
    };

    let trait_name = &b.trait_name;
    let attrs = b.attrs.iter().map(|attr| {
        quote! { #attr }
    });

    let functions = b.body.content.iter().map(|f| {
        let async_kw = if f._async.is_some() {
            quote! { async }
        } else {
            quote! {}
        };

        let fn_name = &f.name;

        let generics = if let Some(generics) = &f.generics {
            let params = generics.params.0.iter().map(|param| &param.value);
            quote! { < #(#params),* > }
        } else {
            quote! {}
        };

        let params = &f.params.content;

        let return_type = if let Some(ret) = &f.ret {
            let ret_type = &ret.second;
            quote! { -> #ret_type }
        } else {
            quote! {}
        };

        quote! {
            #async_kw fn #fn_name #generics (#params) #return_type;
        }
    });

    let trait_def = quote! {
        #(#attrs)*
        pub trait #trait_name #bounds {
            #(#functions)*
        }
    };

    let mut output = TokenStream::new();
    trait_def.to_tokens(&mut output);

    let item_ts: TokenStream = item_clone.into();
    output.extend(item_ts);

    output.into()
}

impl quote::ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new('#', Spacing::Joint));
        self.group.to_tokens(tokens);
    }
}
