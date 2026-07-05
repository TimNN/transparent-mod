use transparent_mod::transparent;

pub fn external() -> &'static str {
    module_path!()
}

#[transparent(pub)]
mod nested {
    pub fn nested_external() -> &'static str {
        module_path!()
    }
}
