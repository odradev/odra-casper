use odra::contract_def::{Argument, Entrypoint, EntrypointType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use super::ty::WrappedType;

pub(crate) struct ContractEntrypoints<'a>(pub &'a Vec<Entrypoint>);

impl ToTokens for ContractEntrypoints<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote!(let mut entry_points = odra::types::EntryPoints::new();));
        tokens.append_all(self.0.iter().map(ContractEntrypoints::build_entry_point));
    }
}

impl ContractEntrypoints<'_> {
    fn build_entry_point(entrypoint: &Entrypoint) -> TokenStream {
        let entrypoint_ident = format_ident!("{}", entrypoint.ident);
        let params = EntrypointParams(&entrypoint.args);
        let ret = WrappedType(&entrypoint.ret);
        let access = match &entrypoint.ty {
            EntrypointType::Constructor => quote! {
                odra::types::EntryPointAccess::Groups(vec![odra::types::Group::new("constructor")])
            },
            EntrypointType::Public => quote! { odra::types::EntryPointAccess::Public },
        };
        quote! {
            entry_points.add_entry_point(
                odra::types::EntryPoint::new(
                    stringify!(#entrypoint_ident),
                    #params,
                    #ret,
                    #access,
                    odra::types::EntryPointType::Contract,
                )
            );
        }
    }
}

struct EntrypointParams<'a>(pub &'a Vec<Argument>);

impl ToTokens for EntrypointParams<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.0.is_empty() {
            tokens.extend(quote!(Vec::<odra::types::Parameter>::new()));
        } else {
            let params_content = self
                .0
                .iter()
                .flat_map(|arg| {
                    let arg_ident = format_ident!("{}", arg.ident);
                    let ty = WrappedType(&arg.ty);
                    quote!(params.push(odra::types::Parameter::new(stringify!(#arg_ident), #ty));)
                })
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

#[cfg(test)]
mod test {
    // use std::vec;

    // use odra::contract_def::{Entrypoint, EntrypointType};
    // use pretty_assertions::assert_str_eq;
    // use quote::ToTokens;

    // use super::ContractEntrypoints;

    // #[test]
    // fn parse_cl_type() {
    //     let a = vec![Entrypoint {
    //         ident: "A".to_string(),
    //         args: vec![],
    //         ret: odra::types::CLType::Map {
    //             key: Box::new(odra::types::CLType::Bool),
    //             value: Box::new(odra::types::CLType::U128),
    //         },
    //         ty: EntrypointType::Public,
    //     }];
    //     let ep = ContractEntrypoints(&a);
    //     let result = ep.to_token_stream();

    //     assert_str_eq!(
    //         result.to_string(),
    //         quote::quote! {
    //             let mut entry_points = odra::types::EntryPoints::new();
    //             entry_points.add_entry_point(
    //                 odra::types::EntryPoint::new(
    //                     stringify!(A),
    //                     Vec::<odra::types::Parameter>::new(),
    //                     odra::types::CLType::Bool,
    //                     odra::types::EntryPointAccess::Public,
    //                     odra::types::EntryPointType::Contract,
    //                 )
    //             );
    //         }
    //         .to_string()
    //     );
    // }
}
