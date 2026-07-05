use std::error::Error;
use std::{env, fs};

static STYLE: &str = r"
<style>
blockquote {
  border-left: 0.25em solid DarkOrchid;
  padding-left: 1em;
  margin: 0 0 0.75em 0;
}
</style>
";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=README.md");

    let mut readme = fs::read_to_string("README.md")?;

    readme = readme.replace("[!IMPORTANT]", "**Important:**");

    // We don't want the `mod` from the test to be placed inside the
    // auto-generated `fn main` because that breaks the `use self::...`.
    readme = readme.replace(
        "```rust <!--empty-main-->\n",
        "```rust\n# #![cfg_attr(outlined_unstable, feature(proc_macro_hygiene))]\n# fn main() {}\n",
    );

    readme = readme.replace("\nmod bar", "\n# #[path=\"bar.rs\"]\nmod bar");

    readme.truncate(
        readme
            .find("<!-- readme-license-begin -->")
            .ok_or("Missing license marker")?,
    );

    readme.push_str(STYLE);

    let mut out_file = env::var("OUT_DIR")?;
    out_file.push_str("/README.processed.md");

    fs::write(&out_file, readme.as_bytes())?;

    let mut bar_file = env::var("OUT_DIR")?;
    bar_file.push_str("/bar.rs");

    fs::write(&bar_file, [])?;

    println!("cargo::rustc-env=README={out_file}");

    Ok(())
}
