use std::error::Error;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=README.md");

    let mut readme = fs::read_to_string("README.md")?;

    // We don't want the `mod` from the test to be placed inside the
    // auto-generated `fn main` because that breaks the `use self::...`.
    readme = readme.replace("```rust <!--empty-main-->\n", "```rust\n# fn main() {}\n");

    readme = readme.replace("\nmod foo", "\n# #[path=\"foo.rs\"]\nmod foo");

    readme.truncate(
        readme
            .find("<!-- readme-license-begin -->")
            .ok_or("Missing license marker")?,
    );

    let mut out_file = env::var("OUT_DIR")?;
    out_file.push_str("/README.processed.md");

    fs::write(&out_file, readme.as_bytes())?;

    let mut foo_file = env::var("OUT_DIR")?;
    foo_file.push_str("/foo.rs");

    fs::write(&foo_file, [])?;

    println!("cargo::rustc-env=README={out_file}");

    Ok(())
}
