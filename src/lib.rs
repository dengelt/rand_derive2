use syn::DeriveInput;

mod generate;
mod generate_enum;
mod generate_struct;
pub(crate) mod parser;

#[proc_macro_derive(RandGen, attributes(rand_derive,))]
pub fn rand_gen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let transform = generate::transform(input);

    transform.into()
}
