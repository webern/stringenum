use std::str::FromStr;
use stringenum::StringEnum;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, StringEnum)]
enum Foo {
    Bar,
    Baz,
    /// We can rename the variant's string equivalent like this. Note that we do
    /// not support automatic capitalization changes, you need to do renames for
    /// snake case or any other preference.
    #[stringenum(rename = "Bones the Cat")]
    Cat,
}

// the as_str function is const
const BAZ_STR_CONST: &str = Foo::Baz.as_str();
const BONES_THE_CAT: &str = Foo::Cat.as_str();

fn main() {
    // we can parse enum variants from their string equivalents
    let bar = Foo::from_str("Bar").unwrap();
    assert_eq!(bar, Foo::Bar);
    let baz = Foo::from_str("Baz").unwrap();
    assert_eq!(baz, Foo::Baz);

    // the lifetime of as_str is static
    let bar_str: &'static str = Foo::Bar.as_str();
    assert_eq!("Bar", bar_str);
    let baz_str: &'static str = Foo::Baz.as_str();
    assert_eq!("Baz", baz_str);
    assert_eq!("Baz", BAZ_STR_CONST);

    // partial eq works
    assert_eq!(Foo::Baz, "Baz");
    assert_eq!("Bones the Cat", Foo::Cat);
    assert_eq!(BONES_THE_CAT.to_string(), Foo::Cat);
}
