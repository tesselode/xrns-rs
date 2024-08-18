use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, LitStr, Type, Variant};

#[proc_macro_derive(Effect, attributes(effect))]
pub fn effect_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let type_name = input.ident;
	let error_type_name = Ident::new(&format!("Invalid{}", type_name), Span::call_site());
	let Data::Enum(data_enum) = input.data else {
		panic!("Effect can only be derived on enums")
	};
	let match_cases = data_enum
		.variants
		.iter()
		.map(variant_match_case)
		.collect::<Vec<_>>();
	let tokens = quote! {
		impl TryFrom<&str> for #type_name {
			type Error = #error_type_name;

			fn try_from(value: &str) -> Result<Self, Self::Error> {
				let effect_letter = &value[0..1];
				let effect_value = value[1..2]
					.parse::<u8>()
					.map_err(|_| #error_type_name(value.to_string()))?;
				match effect_letter {
					#(#match_cases)*
					_ => Err(#error_type_name(value.to_string()))
				}
			}
		}

		#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
		#[display("The effect {} is invalid.", self.0)]
		pub struct #error_type_name(#[error(not(source))] pub String);
	};
	tokens.into()
}

fn variant_match_case(variant: &Variant) -> proc_macro2::TokenStream {
	let variant_name = &variant.ident;
	let effect_attr = variant_effect_attr(variant);
	let effect_letter = &effect_attr[0..1];
	let Fields::Named(fields) = &variant.fields else {
		panic!("only named fields are supported");
	};
	let field = &fields.named[0];
	let field_name = &field.ident;
	match &field.ty {
		Type::Path(path) if path.path.is_ident("bool") => quote! {
			#effect_letter => Ok(Self::#variant_name { #field_name: effect_value == 0 }),
		},
		_ => quote! {
			#effect_letter => Ok(Self::#variant_name { #field_name: effect_value }),
		},
	}
}

fn variant_effect_attr(variant: &Variant) -> String {
	variant
		.attrs
		.iter()
		.find(|attr| attr.path().is_ident("effect"))
		.expect("missing 'effect' attribute for variant")
		.parse_args::<LitStr>()
		.expect("invalid args to 'effect' attribute")
		.value()
}
