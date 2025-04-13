mod syn_tool;

use syn::parse_quote;
use syn_tool::{print_token_tree, read_token_from_file};

fn main() {
    // rs_file_name from args[1]
    if std::env::args().len() >= 2 {
        let rs_file_name = std::env::args().nth(1).expect("No file name given");
        let tokens = read_token_from_file(&rs_file_name);
        print_token_tree(&tokens);
    } else {
        let tokens = parse_quote! {
            fn main() {
                println!("Hello, world!");
            }
        };
        print_token_tree(&tokens);
    }
}
