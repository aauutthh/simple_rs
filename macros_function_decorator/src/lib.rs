use proc_macro::TokenStream;
use quote::quote;
use syn::__private::ToTokens;
use syn::meta::ParseNestedMeta;
use syn::parse::Parser;
use syn::{ItemFn, LitStr, Visibility, parse_macro_input};

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let fn_name = &func.sig.ident;
    let fn_body = &func.block;
    let expanded = quote! {
        fn #fn_name() {
            println!("----- Calling function: {} -----", stringify!(#fn_name));
            #fn_body
        }
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn with_logging(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut level: Option<String> = None;
    let mut message: Option<LitStr> = None;

    // // 用 syn 的解析器来处理 #[with_logging(level = "...", message = "...")]
    // let parser = syn::meta::parser(|meta: ParseNestedMeta| {
    //     meta.parse_nested_meta(|nested| {
    //         let ident = nested.path.get_ident().map(|i| i.to_string());

    //         if let Some(name) = ident {
    //             if name == "level" {
    //                 let lit: LitStr = nested.value()?.parse()?;
    //                 level = Some(lit.value());
    //             } else if name == "message" {
    //                 let lit: LitStr = nested.value()?.parse()?;
    //                 message = Some(lit);
    //             }
    //         }

    //         Ok(())
    //     })
    // });

    // // 调用解析器（使用 syn 的 API）
    // if let Err(err) = parser.parse2(proc_macro2::TokenStream::from(attr)) {
    //     return err.to_compile_error().into();
    // }

    // 解析函数体
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    //let fn_attrs = &input_fn.attrs;
    let fn_vis: &Visibility = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    println!("fn_sig: {}", fn_sig.to_token_stream().to_string());

    //let level = level.unwrap_or_else(|| "info".to_string());
    // let msg_token = message
    //     .map(|m| quote! { #m })
    //     .unwrap_or_else(|| quote! { stringify!(#fn_name) });

    let xgen = quote! {
        #fn_vis #fn_sig {
            println!(">>> {}", "function");
            #fn_block
        }
    };

    xgen.into()
}
