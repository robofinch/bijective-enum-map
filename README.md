<div align="center" class="rustdoc-hidden">
<h1> Bijective Enum Map </h1>
</div>

[<img alt="github" src="https://img.shields.io/badge/github-robofinch/bijective--enum--map-08f?logo=github" height="20">](https://github.com/robofinch/bijective-enum-map)
[![Latest version](https://img.shields.io/crates/v/bijective-enum-map.svg)](https://crates.io/crates/bijective-enum-map)
[![Documentation](https://img.shields.io/docsrs/bijective-enum-map)](https://docs.rs/bijective-enum-map)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)

Provides macros to convert between an enum and other types, either bijectively (with [`From`]
conversions in both directions) or injectively (with [`From`] conversion from the enum to another
type, and [`TryFrom`] in the other direction).

## Motivation

Enums can be useful to clearly indicate permissible values. Rather than having magic values or
hardcoded strings scattered throughout a codebase, it's nice to put all the conversion with
specific version numbers or strings into one place, and use an enum in most of the code.

This, then, requires converting an enum variant into and from a value which it represents.
Ideally, with little boilerplate.

There are several existing alternatives to this crate. For instance, choosing discriminants can
yield conversions in one direction. In the cases of non-numeric values, this might not be possible,
and in cases where an enum needs to be mapped to and from another value, the boilerplate for
converting back to enum variants could be annoying.
Writing every enum variant and value twice requires extra work and could be vulnerable to typos;
this complaint also applies to manually implementing `From` and `TryFrom`.

One existing crate for this problem is [bidirectional_enum], which requires that a macro be
applied to the enum's definition site.
Unfortunately, it is constrained to converting an enum to only one other type, and one of my actual
use cases requires an enum be converted to/from both `&'static str` and `u8`.

If you are looking to convert between enums in particular, [enum-to-enum] may provide what you want;
it has better utility for its target conversions, but my use cases involved other types.

Some other crates focus on discriminants, and some *could* provide conversions between an enum
and some other type, but would not solve the target problem of reducing repetition and boilerplate.

## Terminology

The terms used here for describing functions/maps come from math.

A map is "injective" if different inputs (in this case, enum variants) yield different outputs.

A map is "surjective" if any possible value of the output type is actually an output of some
input.

A map is "bijective" if it is both injective and surjective. Bijective maps are precisely the maps
which have an inverse, thus making `From` conversions in both directions possible for
the [`bijective_enum_map`] macro.

Bijectivity and injectivity can still be violated with these macros, but *should* trigger
`#[warn(unreachable_patterns)]` in most circumstances.

## Examples

Usually, [`injective_enum_map`] is more useful.

```rust
use bijective_enum_map::injective_enum_map;

#[derive(Debug, PartialEq, Eq)]
enum AtMostTwo {
    Zero,
    One,
    Two,
}

injective_enum_map! {
    AtMostTwo, u8,
    Zero <=> 0,
    One  <=> 1,
    Two  <=> 2,
}

injective_enum_map! {
    AtMostTwo, &'static str, &str,
    Zero <=> "zero",
    One  <=> "one",
    Two  <=> "two",
}

assert_eq!(u8::from(AtMostTwo::One), 1_u8);
assert_eq!(AtMostTwo::try_from(2_u8), Ok(AtMostTwo::Two));
assert_eq!(AtMostTwo::try_from(4_u8), Err(()));
// `<&str>::from` would also work
assert_eq!(<&'static str>::from(AtMostTwo::One), "one");
assert_eq!(AtMostTwo::try_from("two"), Ok(AtMostTwo::Two));
assert_eq!(AtMostTwo::try_from("four"), Err(()));
```

An excerpt from one my actual use cases, showing the importance of reducing boilerplate:
```rust
use bijective_enum_map::injective_enum_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChunkVersion {
    V0,  V1,  V2,  V3,  V4,  V5,  V6,  V7,  V8,  V9,
    V10, V11, V12, V13, V14, V15, V16, V17, V18, V19,
    V20, V21, V22, V23, V24, V25, V26, V27, V28, V29,
    V30, V31, V32, V33, V34, V35, V36, V37, V38, V39,
    V40, V41,
}

injective_enum_map! {
    ChunkVersion, u8,
    V0  <=> 0,    V1  <=> 1,    V2  <=> 2,    V3  <=> 3,    V4  <=> 4,
    V5  <=> 5,    V6  <=> 6,    V7  <=> 7,    V8  <=> 8,    V9  <=> 9,
    V10 <=> 10,   V11 <=> 11,   V12 <=> 12,   V13 <=> 13,   V14 <=> 14,
    V15 <=> 15,   V16 <=> 16,   V17 <=> 17,   V18 <=> 18,   V19 <=> 19,
    V20 <=> 20,   V21 <=> 21,   V22 <=> 22,   V23 <=> 23,   V24 <=> 24,
    V25 <=> 25,   V26 <=> 26,   V27 <=> 27,   V28 <=> 28,   V29 <=> 29,
    V30 <=> 30,   V31 <=> 31,   V32 <=> 32,   V33 <=> 33,   V34 <=> 34,
    V35 <=> 35,   V36 <=> 36,   V37 <=> 37,   V38 <=> 38,   V39 <=> 39,
    V40 <=> 40,   V41 <=> 41,
}
```


## Features

There is one feature, `use_type_as`. Internally, one macro needs to declare an alias for an enum
type, because the syntax `<$enum_ty>::$enum_variant` is unstable/experimental in the left side of
a match arm. In order to support the minimum possible MSRV, we default to using
`type __EnumTy = $enum_ty` to work around this problem; however, if there are generic parameters,
this solution is not as good as `use $enum_ty as __EnumTy`, which was stabilized in Rust 1.85
(the 2024 edition). If `use_type_as` is enabled, then `use $enum_ty as __EnumTy` is used.

The `use_type_as` feature therefore has an MSRV of 1.85.

## Minimum supported Rust Version (MSRV)
The macros work on Rust 1.56 (the 2021 edition), which might be a loose bound.
See the above note on `use_type_as`.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE][])
 * MIT license ([LICENSE-MIT][])

at your option.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT

[`bijective_enum_map`]: https://docs.rs/bijective-enum-map/latest/bijective_enum_map/macro.bijective_enum_map.html
[`injective_enum_map`]: https://docs.rs/bijective-enum-map/latest/bijective_enum_map/macro.injective_enum_map.html

[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html

[bidirectional_enum]: https://crates.io/crates/bidirectional_enum
[enum-to-enum]: https://crates.io/crates/enum_to_enum
