use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::visit_mut::VisitMut;
use syn::{
    parse_macro_input, visit_mut, Attribute, AttributeArgs, ItemEnum, ItemImpl, Path, Type,
    TypePath, Variant,
};

/// Define a `#[model]` attribute that can be placed on structs to be used in an API model.
/// Model requirements are automatically applied to the struct and its fields.
/// (The attribute must be placed on sub-structs; it can't be recursively applied to structs
/// referenced in the given struct.)
#[proc_macro_attribute]
pub fn streenum(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse args
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let macro_args = Args::from_list(&attr_args).expect("Unknown args in `streenum` macro");
    let mut streenum = Streenum::new(macro_args);

    // Parse and modify source
    let mut ast: ItemEnum = syn::parse(input).expect("streenum` only works on enums");
    streenum.enum_name = Some(ast.ident.clone());
    streenum.visit_item_enum_mut(&mut ast);
    streenum.append_impls(&mut ast);
    ast.into_token_stream().into()
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

    fn append_impls(&self, ast: &mut ItemEnum) {
        let mut tokens = ast.into_token_stream();
        let enum_name = self.enum_name.as_ref().unwrap();
        let code = quote!(
            impl #enum_name {

            }
        );

        // let code = format!("impl {}{{}}", self.enum_name.unwrap());
        // let proc_macro_token_stream: proc_macro::TokenStream = code.parse().unwrap();
        // let proc_macro2_token_stream: proc_macro2::TokenStream = proc_macro_token_stream.into();
        // let proc_macro_token_tree: proc_macro2::TokenTree = code.parse().unwrap();
        // // let mut my_impls = ItemImpl {
        // //     attrs: vec![],
        // //     defaultness: None,
        // //     unsafety: None,
        // //     impl_token: syn::token::Impl::default(),
        // //     generics: Default::default(),
        // //     trait_: None,
        // //     self_ty: Box::new(Type::Path(TypePath {
        // //         qself: None,
        // //         path: Path::from(self.enum_name.as_ref().unwrap()),
        // //     })),
        // //     brace_token: Default::default(),
        // //     items: vec![],
        // // };
        // // my_impls.to_tokens(&mut tokens);
        tokens.append_all(code.into_iter())
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
