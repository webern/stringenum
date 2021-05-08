use std::str::FromStr;
use streenum::streenum;

#[streenum]
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
enum Foo {
    Bar,
    Baz,
}

const BAZ_STR_CONST: &str = Foo::Baz.as_str();

fn main() {
    let bar = Foo::from_str("Bar").unwrap();
    assert_eq!(bar, Foo::Bar);
    let baz = Foo::from_str("Baz").unwrap();
    assert_eq!(baz, Foo::Baz);
    let bar_str: &'static str = Foo::Bar.as_str();
    assert_eq!("Bar", bar_str);
    let baz_str: &'static str = Foo::Baz.as_str();
    assert_eq!("Baz", baz_str);
    assert_eq!("Baz", BAZ_STR_CONST);
}
