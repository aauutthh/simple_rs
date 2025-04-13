use proc_macro2::{TokenStream, TokenTree};
use std::fs;
use syn::__private::ToTokens;

pub fn read_syn_file_from_file(file_path: &str) -> TokenStream {
    // 读取文件内容
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    // 使用 proc_macro2 解析文件内容为 TokenStream
    let syn_file = syn::parse_file(&content).expect("Unable to parse file");
    let mut tokens = TokenStream::new();
    for item in &syn_file.items {
        tokens.extend(item.to_token_stream());
    }
    tokens
}

pub fn read_token_from_file(file_path: &str) -> TokenStream {
    // 读取文件内容
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    // 使用 proc_macro2 解析文件内容为 TokenStream
    syn::parse_str(&content).expect("Unable to parse file")
}

pub fn print_token_tree_recursive(tokens: &TokenStream, indent: usize, prefix: &str) {
    let indent_str = format!("{}{}", prefix, "  ".repeat(indent)); // 缩进
    for tree in tokens.clone().into_iter() {
        match tree {
            TokenTree::Group(group) => {
                println!("{}- {:?}-Group:", indent_str, group.delimiter());
                print_token_tree_recursive(&group.stream(), indent + 1, prefix);
            }
            TokenTree::Ident(ident) => {
                println!("{}- Ident: {}", indent_str, ident);
            }
            TokenTree::Punct(punct) => {
                println!("{}- Punct: {}", indent_str, punct);
            }
            TokenTree::Literal(lit) => {
                println!("{}- Literal: {}", indent_str, lit);
            }
        }
    }
}

// 递归打印 TokenStream 的 token 树
pub fn print_token_tree(tokens: &TokenStream) {
    println!("Top:");
    print_token_tree_recursive(tokens, 1, "");
}
