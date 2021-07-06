use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, visit_mut, AttributeArgs, ItemEnum, Variant};

#[proc_macro_attribute]
pub fn stringenum(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse args
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let macro_args = Args::from_list(&attr_args).expect("Unknown args in `streenum` macro");
    let mut streenum = Streenum::new(macro_args);

    // Parse and modify source
    let mut ast: ItemEnum = syn::parse(input).expect("streenum` only works on enums");
    streenum.enum_name = Some(ast.ident.clone());
    streenum.visit_item_enum_mut(&mut ast);
    let mut ast2: TokenStream2 = ast.to_token_stream().into();
    streenum.append_impls(&mut ast2);
    ast2.into_token_stream().into()
}

/// Stores the user's requested options
#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct Args {
    rename: Option<String>,
    default: Option<String>,
}

/// Does the visiting and logic
#[derive(Debug, Default)]
struct Streenum {
    args: Args,
    enum_name: Option<Ident>,
    variants: Vec<Ident>,
    renames: Vec<Option<String>>,
}

impl Streenum {
    fn new(args: Args) -> Self {
        Self {
            args,
            ..Self::default()
        }
    }

    fn append_impls(&self, ast: &mut TokenStream2) {
        // let mut tokens = ast.into_token_stream();
        let enum_name = self.enum_name.as_ref().unwrap();
        let variant_name = self.variants.iter().map(|v| v.to_string());
        let variant_ident = self.variants.iter();

        let variant_name2 = self.variants.iter().map(|v| v.to_string());
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
        );

        ast.append_all(code.into_iter())
    }
}

impl VisitMut for Streenum {
    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        // Let the default implementation do its thing, recursively.
        visit_mut::visit_item_enum_mut(self, node);
    }

    fn visit_variant_mut(&mut self, node: &mut Variant) {
        let ident = node.ident.clone();
        self.renames.push(None);
        self.variants.push(ident);

        // Let the default implementation do its thing, recursively.
        visit_mut::visit_variant_mut(self, node)
    }
}
