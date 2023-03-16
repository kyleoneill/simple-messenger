extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn authenticate_user(attr: TokenStream, item: TokenStream) -> TokenStream {
    // attr is going to be what is passed to the proc macro
    // item is going to be the entire function, signature and body
    panic!("item: \"{}\"", item.to_string());
    item
}
