extern crate proc_macro2;

use proc_macro2::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn vertex_layout(_: TokenStream, TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
}


#[vertex_layout]
pub struct MyVertex{
    [vertex_position]
    pub position: [f32; 3],
    [vertex_color]
    pub color: [f32; 4],
}
