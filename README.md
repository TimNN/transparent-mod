# `tmod`: Transparent Modules

<!-- Parts of this README are based on https://github.com/dtolnay's setup. -->
<!-- Badge colors were picked from https://uchu.style/. -->

[<img alt="github" src="https://img.shields.io/badge/github-timnn/tmod-afecb6?style=for-the-badge&logo=github" height="20">](https://github.com/TimNN/tmod)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tmod?style=for-the-badge&logo=rust&color=3984f2" height="20">](https://crates.io/crates/tmod)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tmod-c7abe9?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/tmod)
<img alt="license" src="https://img.shields.io/crates/l/tmod?style=for-the-badge&color=e3e5e5" height="20">
[<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/TimNN/tmod/ci.yml?style=for-the-badge" height="20">](https://github.com/TimNN/tmod/actions/workflows/ci.yml)

This crate provides the `#[transparent]` macro, making a module effectively 
transparent:

```rust <!--empty-main-->
use tmod::transparent;

#[transparent(pub)]
mod foo {}  // `{}` is special-cased and changed to `;`.
```

results in

```rust <!--empty-main-->
mod foo;
pub use self::foo::*;
```

Note that a plain `#[transparent] mod foo;` does not work on stable, and 
rust-analyzer will not properly analyze the alternative `#[transparent] mod foo
{}` syntax implemented by this macro.

<!-- readme-license-begin -->

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

#### Disclaimer

<sub>
This is a personal project. The views, code, and opinions expressed here are my
own and do not represent those of my current or past employers.
</sub>
