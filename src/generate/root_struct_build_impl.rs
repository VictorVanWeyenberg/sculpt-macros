use quote::{format_ident, quote};
use crate::SculptableStruct;

pub fn generate_root_struct_build_impl(sculptable: &SculptableStruct) ->proc_macro2::TokenStream {
    let sculptable_ident = format_ident!("{}", sculptable.name);
    let builder_name = format_ident!("{}Builder", sculptable.name);
    let callbacks_name = format_ident!("{}Callbacks", builder_name);
    quote! {
        impl #sculptable_ident {
            pub fn build<T: #callbacks_name>(t: &mut T) -> #sculptable_ident {
                #builder_name::<T>::new(t).build()
            }
        }
    }
}