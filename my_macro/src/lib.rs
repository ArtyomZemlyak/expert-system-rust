extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn big_int_match(_item: TokenStream) -> TokenStream {
    let str_upper_bound: String = _item.to_string();
    let upper_bound: usize = str_upper_bound.parse().unwrap();
    let mut ts = String::from("match i {\n");
    for i in 0..upper_bound {
        ts.extend([String::from("    "), i.to_string(), String::from("=>"), i.to_string(), String::from(",\n")]);
    }
    ts.extend([String::from("    _ => 1010\n")]);
    ts.extend([String::from("}")]);
    ts.parse().unwrap()
}