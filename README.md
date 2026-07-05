# Transparent Modules for Rust

<!-- Parts of this README are based on https://github.com/dtolnay's setup. -->
<!-- Badge colors were picked from https://uchu.style/. -->

[<img alt="github" src="https://img.shields.io/badge/github-timnn/transparent-mod-afecb6?style=for-the-badge&logo=github" height="20">](https://github.com/TimNN/transparent-mod)
[<img alt="crates.io" src="https://img.shields.io/crates/v/transparent-mod?style=for-the-badge&logo=rust&color=3984f2" height="20">](https://crates.io/crates/transparent-mod)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-transparent-mod-c7abe9?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/transparent-mod)
<img alt="license" src="https://img.shields.io/crates/l/transparent-mod?style=for-the-badge&color=e3e5e5" height="20">
[<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/TimNN/transparent-mod/ci.yml?style=for-the-badge" height="20">](https://github.com/TimNN/transparent-mod/actions/workflows/ci.yml)

This crate provides the `#[transparent]` macro, making a module effectively
transparent:

```rust <!--empty-main-->
use transparent_mod::transparent;

#[transparent(pub)]
mod foo { /* ... */ }

#[transparent]
pub mod bar;
```

results in

```rust <!--empty-main-->
mod foo { /* ... */ }
pub use self::foo::*;

pub mod bar;
pub(self) use self::bar::*;
```

> [!IMPORTANT]
> Using the `#[transparent]` macro with `mod bar;` (an external
> module) requires at least Rust 1.100 or `#![feature(proc_macro_hygiene)]`.

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
