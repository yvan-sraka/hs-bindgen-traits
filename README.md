<!-- cargo-sync-readme start -->

# `hs-bingen-traits`

Utility traits behind [`hs-bindgen`](https://github.com/yvan-sraka/hs-bindgen)
ergonomics. It helps user to easily define wrapper function to derive a Rust
type from and into a C-FFI safe target type (that match the memory layout of
an Haskell type).

What's this library for?
[Does `repr(C)` define a trait I can use to check structs were declared with `#repr(C)`?](https://users.rust-lang.org/t/16323)
The answer is sadly no ... that's what this library trying to provide, like
what [`safer_ffi`](https://docs.rs/safer-ffi/latest/safer_ffi/layout/trait.ReprC.html)
does, but in a simpler and more minimal way, since the goal here is only to
target Haskell FFI.

<!-- cargo-sync-readme end -->
