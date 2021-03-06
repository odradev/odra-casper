use odra::types::CLType;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(super) struct WrappedType<'a>(pub &'a CLType);

impl ToTokens for WrappedType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = match &self.0 {
            CLType::Bool => quote!(odra::types::CLType::Bool),
            CLType::I32 => quote!(odra::types::CLType::I32),
            CLType::I64 => quote!(odra::types::CLType::I64),
            CLType::U8 => quote!(odra::types::CLType::U8),
            CLType::U32 => quote!(odra::types::CLType::U32),
            CLType::U64 => quote!(odra::types::CLType::U64),
            CLType::U128 => quote!(odra::types::CLType::U128),
            CLType::U256 => quote!(odra::types::CLType::U256),
            CLType::U512 => quote!(odra::types::CLType::U512),
            CLType::Unit => quote!(odra::types::CLType::Unit),
            CLType::String => quote!(odra::types::CLType::String),
            CLType::Option(ty) => {
                let value_stream = WrappedType(&**ty).to_token_stream();
                quote!(odra::types::CLType::Option(Box::new(#value_stream)))
            }
            CLType::Any => quote!(odra::types::CLType::Any),
            CLType::Key => quote!(odra::types::CLType::Key),
            CLType::URef => quote!(odra::types::CLType::URef),
            CLType::PublicKey => quote!(odra::types::CLType::PublicKey),
            CLType::List(ty) => {
                let value_stream = WrappedType(&**ty).to_token_stream();
                quote!(odra::types::CLType::List(Box::new(#value_stream)))
            }
            CLType::ByteArray(bytes) => quote!(odra::types::CLType::ByteArray(#bytes)),
            CLType::Result { ok, err } => {
                let ok_stream = WrappedType(&**ok).to_token_stream();
                let err_stream = WrappedType(&**err).to_token_stream();
                quote! {
                    odra::types::CLType::Result {
                        ok: Box::new(#ok_stream),
                        err: Box::new(#err_stream),
                    }
                }
            }
            CLType::Map { key, value } => {
                let key_stream = WrappedType(&**key).to_token_stream();
                let value_stream = WrappedType(&**value).to_token_stream();
                quote! {
                    odra::types::CLType::Map {
                        key: Box::new(#key_stream),
                        value: Box::new(#value_stream),
                    }
                }
            }
            CLType::Tuple1(ty) => {
                let ty = &**ty.get(0).unwrap();
                let ty = WrappedType(ty).to_token_stream();
                quote! {
                    odra::types::CLType::Tuple1([#ty])
                }
            }
            CLType::Tuple2(ty) => {
                let t1 = &**ty.get(0).unwrap();
                let t1 = WrappedType(t1).to_token_stream();
                let t2 = &**ty.get(1).unwrap();
                let t2 = WrappedType(t2).to_token_stream();
                quote! {
                    odra::types::CLType::Tuple2([#t1, #t2])
                }
            }
            CLType::Tuple3(ty) => {
                let t1 = &**ty.get(0).unwrap();
                let t1 = WrappedType(t1).to_token_stream();
                let t2 = &**ty.get(1).unwrap();
                let t2 = WrappedType(t2).to_token_stream();
                let t3 = &**ty.get(2).unwrap();
                let t3 = WrappedType(t3).to_token_stream();
                quote! {
                    odra::types::CLType::Tuple2([#t1, #t2, #t3])
                }
            }
        };
        tokens.extend(stream);
    }
}
