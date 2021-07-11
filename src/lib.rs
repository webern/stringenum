// use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
// use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Lit, MetaNameValue};

/// # Description
///
/// A derive macro that creates `const &'static str` representations of enum variants.
///
/// # Example
///
/// ```
/// use stringenum::StringEnum;
///
/// #[derive(StringEnum)]
/// enum Color {
///     Red,
///     #[stringenum(rename="plum")]
///     Purple,
/// }
///
/// assert_eq!("plum", Color::Purple);
/// assert_eq!("Red", Color::Red);
/// ```
#[proc_macro_derive(StringEnum, attributes(stringenum))]
pub fn stringenum(input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let i = input.clone();
    let derive_input = parse_macro_input!(i as DeriveInput);
    let strenum = Strenum::new(&derive_input);

    // write function implementations to a token stream
    let mut ast2 = TokenStream2::new();
    strenum.write_impls(&mut ast2);
    ast2.into_token_stream().into()
}

/// Represents the information we need in order to build the implementation functions.
#[derive(Debug, Default)]
struct Strenum {
    enum_name: Option<Ident>,
    variants: Vec<Ident>,
    names: Vec<String>,
}

impl Strenum {
    /// Parse the `DeriveInput` for the information we need.
    fn new(input: &DeriveInput) -> Self {
        let mut streenum = Self::default();

        streenum.enum_name = Some(input.ident.clone());
        let enum_data = match &input.data {
            Data::Enum(en) => en,
            _ => panic!("only enums are supported"),
        };

        // check attribute arguments for rename strings and populate name vectors
        for variant in &enum_data.variants {
            let name =
                find_rename_value(&variant.attrs).unwrap_or_else(|| variant.ident.to_string());
            streenum.variants.push(variant.ident.clone());
            streenum.names.push(name);
        }

        streenum
    }

    fn write_impls(&self, ast: &mut TokenStream2) {
        let enum_name = self.enum_name.as_ref().unwrap();

        // we two iters for each vec because they will be moved later
        let variant_name = self.names.iter();
        let variant_ident = self.variants.iter();
        let variant_name2 = self.names.iter();
        let variant_ident2 = self.variants.iter();

        let code = quote!(
            impl #enum_name {
                /// Return a `static`, `const` representation of the variant name.
                pub const fn as_str(&self) -> &'static str {
                    match &self {
                        #( #enum_name::#variant_ident => #variant_name, )*
                    }
                }
            }

            impl std::str::FromStr for #enum_name {
                type Err = &'static str;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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

            impl From<#enum_name> for std::string::String {
                fn from(x: #enum_name) -> Self {
                    x.as_ref().to_owned()
                }
            }

            impl PartialEq<str> for #enum_name {
                fn eq(&self, other: &str) -> bool {
                    self.as_ref() == other
                }
            }

            impl PartialEq<std::string::String> for #enum_name {
                fn eq(&self, other: &std::string::String) -> bool {
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

            impl PartialEq<#enum_name> for std::string::String {
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

/// Check all of the attributes to find a `stringenum(rename = "...")` if it exists.
fn find_rename_value(attrs: &[Attribute]) -> std::option::Option<std::string::String> {
    for a in attrs {
        let maybe_rename = get_rename_value(a);
        if maybe_rename.is_some() {
            return maybe_rename;
        }
    }
    None
}

/// Check an attribute to see if it is a `stringenum(rename = "...")` construct.
fn get_rename_value(a: &Attribute) -> std::option::Option<std::string::String> {
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
