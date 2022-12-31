use proc_macro::TokenStream;
use quote::quote;
use syn;

extern crate proc_macro;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("안녕하세요 매크로! 내 이름은 {}입니다!", stringify!(#name));
            }
        }
    };
    gen.into()
}
