use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn is_pub(f: &&syn::Field) -> bool {
    matches!(f.vis, syn::Visibility::Public(_))
}

/// Proc macro that generates a debug implementation with the non pub fields ommited
///
/// This macro works by generating a substruct with only the pub fields and implementing debug on that struct.
#[proc_macro_derive(PubDebug)]
pub fn pub_debug(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let substruct_name = syn::Ident::new(&format!("{name}Debug"), name.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("This macro is only supported for structs")
    };

    let pub_fields = fields.iter().filter(is_pub);

    let copy_fields = fields.iter().filter(is_pub).map(|f| {
        let name = &f.ident;
        quote! {
            #name: self.#name
        }
    });

    let generated = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

                #[derive(Debug)]
                #[allow(unused)]
                struct #substruct_name {
                    #(#pub_fields),*
                }

                let tmp = #substruct_name {
                    #(#copy_fields),*
                };

                write!(f, "{tmp:#?}")
            }
        }
    };

    generated.into()
}

/// Proc macro that generates a `build_editor(ui)` method on the struct that builds an
/// interface for editing the type in real time through the imgui ui
#[proc_macro_derive(UiEditable)]
pub fn pub_imgui_editable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    quote!(
        use hudhook::imgui;
        impl #name {
            pub fn build_editor(&mut self, ui: &imgui::Ui) {
                unimplemented!()
            }
        }
    )
    .into()
}
