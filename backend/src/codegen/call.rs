use odra::{
    contract_def::{Argument, Entrypoint},
    types::CLType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

pub(crate) struct ContractEntrypoints<'a>(pub &'a Vec<Entrypoint>);

impl ToTokens for ContractEntrypoints<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote!(let mut entry_points = odra::types::EntryPoints::new();));
        tokens.append_all(self.0.iter().map(|ep| ContractEntrypoints::build_entry_point(ep)));
    }
}

impl ContractEntrypoints<'_> {
    fn build_entry_point(entrypoint: &Entrypoint) -> TokenStream {
        let entrypoint_ident = format_ident!("{}", entrypoint.ident);
        let params = EntrypointParams(&entrypoint.args);
        let ret = WrappedType(&entrypoint.ret);

        quote! {
            entry_points.add_entry_point(
                odra::types::EntryPoint::new(
                    stringify!(#entrypoint_ident),
                    #params,
                    #ret,
                    odra::types::EntryPointAccess::Public,
                    odra::types::EntryPointType::Contract,
                )
            );
        }
    }
}

struct WrappedType<'a>(pub &'a CLType);

impl ToTokens for WrappedType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = match &self.0 {
            odra::types::CLType::Bool => quote!(odra::types::CLType::Bool),
            odra::types::CLType::I32 => quote!(<i32 as odra::types::CLTyped>::cl_type()),
            odra::types::CLType::I64 => quote!(<i64 as odra::types::CLTyped>::cl_type()),
            odra::types::CLType::U8 => quote!(<u8 as odra::types::CLTyped>::cl_type()),
            odra::types::CLType::U32 => quote!(<u32 as odra::types::CLTyped>::cl_type()),
            odra::types::CLType::U64 => quote!(<u64 as odra::types::CLTyped>::cl_type()),
            odra::types::CLType::U128 => quote!(odra::types::CLType::U128),
            odra::types::CLType::U256 => quote!(odra::types::CLType::U256),
            odra::types::CLType::U512 => quote!(odra::types::CLType::U512),
            odra::types::CLType::Unit => quote!(<() as odra::types::CLTyped>::cl_type()),
            odra::types::CLType::String => quote!(odra::types::CLType::String),
            odra::types::CLType::Option(value) => {
                let value_stream = WrappedType(&**value).to_token_stream();
                quote!(odra::types::CLType::Option(Box::new(#value_stream)))
            }
            odra::types::CLType::Any => quote!(odra::types::CLType::Any),
            _ => panic!("Unsupported arg type"),
        };
        tokens.extend(stream);
    }
}

struct EntrypointParams<'a>(pub &'a Vec<Argument>);

impl ToTokens for EntrypointParams<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.0.is_empty() {
            tokens.extend(quote!(Vec::<odra::types::Parameter>::new()));
        } else {
            let params_content = self.0
                .iter()
                .map(|arg| {
                    let arg_ident = format_ident!("{}", arg.ident);
                    let ty = WrappedType(&arg.ty);
                    quote!(params.push(odra::types::Parameter::new(stringify!(#arg_ident), #ty));)
                })
                .flatten()
                .collect::<TokenStream>();

            let params = quote! {
                {
                    let mut params: Vec<odra::types::Parameter> = Vec::new();
                    #params_content
                    params
                }
            };

            tokens.extend(params);
        };
    }
}
