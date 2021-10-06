use crate::{
    ast::traits::AstPart,
    ast_part,
    derive_parse::{optional::Optional, separated::Pair},
    impl_prelude::*,
};
use syn::token::Comma;

ast_newtype!(Signature {
    description: "signature",
    inner: syn::Signature
});

impl Signature {
    pub fn generic_params(&self) -> Option<GenericParams> {
        let generics = &self.inner.generics;
        let angles = Angles::new(generics.lt_token?, generics.gt_token?);

        let angled = angles.wrap(generics.params.clone());

        Some(GenericParams { inner: angled })
    }

    pub fn function_name(&self) -> Identifier {
        self.inner.ident.clone().into()
    }

    pub fn where_clause(&self) -> Option<WhereClause> {
        self.inner.generics.where_clause.clone().map(|c| c.into())
    }

    pub fn qualifiers(&self) -> Qualifiers {
        let syn::Signature {
            asyncness,
            constness,
            unsafety,
            abi,
            ..
        } = self.inner.clone();

        Qualifiers {
            asyncness: asyncness.map(|a| a.into()).into(),
            constness: constness.map(|c| c.into()).into(),
            unsafety: unsafety.map(|u| u.into()).into(),
            abi,
        }
    }

    pub fn asyncness(&self) -> Option<LeafToken<SynToken![async]>> {
        self.inner.asyncness.map(|a| a.into())
    }

    pub fn constness(&self) -> Option<LeafToken<SynToken![const]>> {
        self.inner.constness.map(|a| a.into())
    }

    pub fn unsafety(&self) -> Option<LeafToken<SynToken![unsafe]>> {
        self.inner.unsafety.map(|a| a.into())
    }

    pub fn extern_abi(&self) -> Option<ExternAbi> {
        self.inner.abi.clone().map(|a| a.into())
    }

    pub fn parameters(&self) -> FnParameters {
        let inputs: Separated<syn::FnArg, SynToken![,]> = self.inner.inputs.clone().into();

        FnParameters::from(inputs)
    }

    pub fn return_type(&self) -> ReturnType {
        ReturnType::from(self.inner.output.clone())
    }
}

ast_newtype!(GenericParams {
    description: "generic parameters",
    inner: Angled<Separated<syn::GenericParam, Comma>>
});

ast_newtype!(Const {
    description: "const",
    inner: SynToken![const]
});

token_newtype!(Const);

ast_part!(Const in Signature {
    add(signature, constness) signature.constness = Some(constness.inner);
    remove(signature) signature.constness = None;
});

ast_newtype!(Async {
    description: "async",
    inner: SynToken![async]
});

token_newtype!(Async);

ast_part!(Async in Signature {
    add(signature, asyncness) signature.asyncness = Some(asyncness.inner);
    remove(signature) signature.asyncness = None;
});

ast_newtype!(Unsafe {
    description: "unsafe",
    inner: SynToken![unsafe]
});

token_newtype!(Unsafe);

ast_part!(Unsafe in Signature {
    add(signature, unsafety) signature.unsafety = Some(unsafety.inner);
    remove(signature) signature.unsafety = None;
});

#[derive(SynParse)]
pub struct Qualifiers {
    constness: Optional<Const>,
    asyncness: Optional<Async>,
    unsafety: Optional<Unsafe>,
    abi: Option<syn::Abi>,
}

impl ToTokens for Qualifiers {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let Self {
            asyncness,
            constness,
            unsafety,
            abi,
        } = self;

        tokens.extend(tokens!(#asyncness #constness #unsafety #abi));
    }
}

ast_newtype!(WhereClause {
    description: "where clause",
    inner: syn::WhereClause
});

ast_newtype!(FnParameters {
    description: "function parameters",
    inner: Separated<syn::FnArg, SynToken![,]>
});

impl FnParameters {
    fn self_pair(&self) -> Option<Pair<SelfParameter, SynToken![,]>> {
        self.inner.clone().into_pairs().find_map(|pair| {
            pair.flat_map(|item| match item {
                syn::FnArg::Receiver(receiver) => Some(SelfParameter::from(receiver.clone())),
                syn::FnArg::Typed(_) => None,
            })
        })
    }

    pub fn self_param(&self) -> Option<SelfParameter> {
        self.self_pair().map(|pair| pair.into_item())
    }

    pub fn self_comma(&self) -> Option<SynToken![,]> {
        self.self_pair()?.into_separator()
    }

    pub fn params(&self) -> Separated<FnParameter, SynToken![,]> {
        let iter = self.inner.clone().into_pairs().filter_map(|param| {
            param.flat_map(|p| match p {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(param) => Some(FnParameter::from(param.clone())),
            })
        });

        iter.collect()
    }
}

ast_newtype!(SelfParameter {
    description: "self parameter",
    inner: syn::Receiver
});

ast_newtype!(FnParameter { description: "function parameter", inner: syn::PatType } no Parse);

ast_newtype!(ReturnType {
    description: "return type",
    inner: syn::ReturnType
});

ast_newtype!(ExternAbi {
    description: |abi| {
        match &abi.name {
            Some(lit) => format!("extern {:?}", lit.value()),
            None => format!("extern"),
        }
    },
    inner: syn::Abi
});
