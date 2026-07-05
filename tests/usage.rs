#![cfg_attr(outlined_unstable, feature(proc_macro_hygiene))]

use transparent_mod::transparent;

#[transparent]
mod external;

#[transparent]
#[path = "path/arbitrary.rs"]
mod via_path;

#[transparent]
mod inline {
    use transparent_mod::transparent;

    #[transparent(pub)]
    mod nested {
        pub fn nested_internal() -> &'static str {
            module_path!()
        }
    }
}

#[test]
fn tests() {
    assert_eq!(external(), "usage::external");
    assert_eq!(nested_external(), "usage::external::nested");
    assert_eq!(via_path(), "usage::via_path");
    assert_eq!(nested_internal(), "usage::inline::nested");
}
