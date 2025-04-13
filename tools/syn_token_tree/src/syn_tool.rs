use proc_macro2::{TokenStream, TokenTree};
use std::fmt::Write;
use std::fs;
use syn::__private::ToTokens;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
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

pub fn print_token_tree_recursive<W: Write>(
    w: &mut W,
    tokens: &TokenStream,
    indent: usize,
    prefix: &str,
) -> Result<()> {
    let indent_str = format!("{}{}", prefix, "  ".repeat(indent)); // 缩进
    for tree in tokens.clone().into_iter() {
        match tree {
            TokenTree::Group(group) => {
                write!(w, "{}- {:?}-Group:\n", indent_str, group.delimiter())?;
                print_token_tree_recursive(w, &group.stream(), indent + 1, prefix)?;
            }
            TokenTree::Ident(ident) => {
                write!(w, "{}- Ident: {}\n", indent_str, ident)?;
            }
            TokenTree::Punct(punct) => {
                write!(w, "{}- Punct: {}\n", indent_str, punct)?;
            }
            TokenTree::Literal(lit) => {
                write!(w, "{}- Literal: {}\n", indent_str, lit)?;
            }
        }
    }
    Ok(())
}

// 递归打印 TokenStream 的 token 树
pub fn print_token_tree(tokens: &TokenStream) {
    use std::io::{self, Write as IoWrite};
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    print_token_tree_recursive(&mut buffer, &tokens, 1, "") //
        .expect("Failed to print tree");

    // Print the buffer to stdout
    stdout
        .write_all(buffer.as_bytes())
        .expect("failed to write to stdout");
    stdout //
        .flush()
        .expect("failed to flush stdout");
}
