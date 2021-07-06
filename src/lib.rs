// use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
// use syn::visit_mut::VisitMut;
use syn::{
    parse_macro_input, AttrStyle, Attribute, Data, DataEnum, DeriveInput, Lit, MetaNameValue,
};

#[proc_macro_derive(StringEnum, attributes(stringenum))]
pub fn stringenum(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let i = input.clone();
    let derive_input = parse_macro_input!(i as DeriveInput);
    let mut streenum = Streenum::new();

    // Parse and modify source
    // let mut ast: ItemEnum = syn::parse(input).expect("only works on enums");
    streenum.enum_name = Some(derive_input.ident.clone());
    let enum_data = match &derive_input.data {
        Data::Enum(en) => en,
        _ => panic!("only enums are supported"),
    };
    streenum.find_renames(enum_data);
    // panic!("{:?}", streenum);
    // let mut ast2: TokenStream2 = ast.to_token_stream().into();
    let mut ast2 = TokenStream2::new();
    streenum.append_impls(&mut ast2);
    ast2.into_token_stream().into()
    // input
    // TokenStream::new()
}

// #[proc_macro_attribute]
// pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }

/// Does the visiting and logic
#[derive(Debug, Default)]
struct Streenum {
    enum_name: Option<Ident>,
    variants: Vec<Ident>,
    names: Vec<String>,
}

impl Streenum {
    fn new() -> Self {
        Self::default()
    }

    fn append_impls(&self, ast: &mut TokenStream2) {
        // let mut tokens = ast.into_token_stream();
        let enum_name = self.enum_name.as_ref().unwrap();
        let variant_name = self.names.iter();
        let variant_ident = self.variants.iter();

        let variant_name2 = self.names.iter();
        let variant_ident2 = self.variants.iter();
        // #(#variants),*
        let code = quote!(
            impl #enum_name {
                pub const fn as_str(&self) -> &'static str {
                    match &self {
                        #( #enum_name::#variant_ident => #variant_name, )*
                    }
                }
            }

            impl std::str::FromStr for #enum_name {
                type Err = &'static str;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #( #variant_name2 => Ok(#enum_name::#variant_ident2),)*
                        _ => Err("unrecognized variant name"),
                    }
                }
            }

            impl AsRef<str> for #enum_name {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

            impl std::ops::Deref for #enum_name {
                type Target = str;

                fn deref(&self) -> &Self::Target {
                    self.as_str()
                }
            }

            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(self.as_str(), f)
                }
            }

            impl std::borrow::Borrow<str> for #enum_name {
                fn borrow(&self) -> &str {
                    self.as_str()
                }
            }

            impl From<#enum_name> for String {
                fn from(x: #enum_name) -> Self {
                    x.as_ref().to_owned()
                }
            }

            impl PartialEq<str> for #enum_name {
                fn eq(&self, other: &str) -> bool {
                    self.as_ref() == other
                }
            }

            impl PartialEq<String> for #enum_name {
                fn eq(&self, other: &String) -> bool {
                    self.as_ref() == other.as_str()
                }
            }

            impl PartialEq<&str> for #enum_name {
                fn eq(&self, other: &&str) -> bool {
                    &(self.as_ref()) == other
                }
            }

            impl PartialEq<#enum_name> for str {
                fn eq(&self, other: &#enum_name) -> bool {
                    self == other.as_ref()
                }
            }

            impl PartialEq<#enum_name> for String {
                fn eq(&self, other: &#enum_name) -> bool {
                    self.as_str() == other.as_str()
                }
            }

            impl PartialEq<#enum_name> for &str {
                fn eq(&self, other: &#enum_name) -> bool {
                    *self == other.as_str()
                }
            }
        );

        ast.append_all(code.into_iter())
    }
}

impl Streenum {
    fn find_renames(&mut self, enum_data: &DataEnum) {
        // Let the default implementation do its thing, recursively.
        // visit_mut::visit_item_enum_mut(self, node);
        for variant in &enum_data.variants {
            let name =
                find_rename_value(&variant.attrs).unwrap_or_else(|| variant.ident.to_string());
            self.variants.push(variant.ident.clone());
            self.names.push(name);
        }
    }
}

fn find_rename_value(attrs: &[Attribute]) -> Option<String> {
    for a in attrs {
        let maybe_rename = get_rename_value(a);
        if maybe_rename.is_some() {
            return maybe_rename;
        }
    }
    None
}

fn get_rename_value(a: &Attribute) -> Option<String> {
    if !matches!(a.style, AttrStyle::Outer)
        || a.path.segments.len() != 1
        || a.path.segments.first().unwrap().ident.to_string() != "stringenum"
    {
        return None;
    }

    let meta: MetaNameValue = a.parse_args().unwrap();

    if meta.path.leading_colon.is_some()
        || meta.path.segments.len() != 1
        || meta.path.segments.first().unwrap().ident.to_string() != "rename"
    {
        panic!("bad argument, only 'rename' is supported")
    }

    let s = match meta.lit {
        Lit::Str(s) => s.value(),
        _ => panic!("rename value must be a string"),
    };

    Some(s)
}

// impl VisitMut for Streenum {
//
//
//     fn visit_variant_mut(&mut self, node: &mut Variant) {
//         let ident = node.ident.clone();
//         self.renames.push(None);
//         self.variants.push(ident);
//
//         // Let the default implementation do its thing, recursively.
//         visit_mut::visit_variant_mut(self, node)
//     }
// }
