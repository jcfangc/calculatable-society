mod components;

use components::property_map;
use proc_macro::TokenStream;

#[proc_macro_derive(PropertyMap)]
pub fn property_map_derive(input: TokenStream) -> TokenStream {
    property_map::property_map_derive(input)
}
