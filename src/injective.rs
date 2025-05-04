/// Map an enum into and from another type (or two types) using `From` and `TryFrom`
/// (with unit error).
///
/// The enum type must be specified, followed by the type to map the enum into (`$into`),
/// optionally followed by the type to try to map into the enum (`$try_from`).
/// If `$try_from` is not specified, it is set to `$into`.
///
/// The two types must be similar enough that the same value expression (e.g., a numeric or string
/// literal) works for either; moreover, the expression must also be a valid pattern for a match
/// arm, so arbitrarily complicated expressions are not permitted.
/// In practice, the types should usually be the same, but specifying
/// two different types is useful for converting into `&'static str` and trying to convert
/// from `&str`, for example. Unintended options like mapping into `Range<u8>` and from `u8`
/// might be possible, but are not tested here. Note that non-unit variants are supported.
///
/// This map is intended to be "injective"; different enum variants should map into different
/// values, so that they can be mapped back unambiguously. The map may (or may not) also be
/// "surjective", in which any possible value of the target type is associated with some enum
/// variant, in which case the `TryFrom` implementation would not be able to fail (but this macro
/// does not check for surjectivity). You should use `bijective_enum_map` if the mapping is
/// also surjective.
///
/// If the map is not injective, and multiple enum variants map to the same value, then a compiler
/// warning from `#[warn(unreachable_patterns)]` *should* be printed in most circumstances,
/// but it could be a silent logic error. In such a case, only the first enum variant
/// listed will be mapped from the duplicate value. Such a warning should also occur if an enum
/// variant is repeated.
///
/// # Examples
///
/// ## Map into and from two other types:
/// ```
/// use bijective_enum_map::injective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum AtMostTwo {
///     Zero,
///     One,
///     Two,
/// }
///
/// injective_enum_map! {
///     AtMostTwo, u8,
///     Zero <=> 0,
///     One  <=> 1,
///     Two  <=> 2,
/// }
///
/// injective_enum_map! {
///     AtMostTwo, &'static str, &str,
///     Zero <=> "zero",
///     One  <=> "one",
///     Two  <=> "two",
/// }
///
/// assert_eq!(u8::from(AtMostTwo::One), 1_u8);
/// assert_eq!(AtMostTwo::try_from(2_u8), Ok(AtMostTwo::Two));
/// assert_eq!(AtMostTwo::try_from(4_u8), Err(()));
/// // `str::from` would also work
/// assert_eq!(<&'static str>::from(AtMostTwo::One), "one");
/// assert_eq!(AtMostTwo::try_from("two"), Ok(AtMostTwo::Two));
/// assert_eq!(AtMostTwo::try_from("four"), Err(()));
/// ```
///
/// ## Empty enum, mapped into and from different types:
/// ```
/// use bijective_enum_map::injective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum Empty {}
///
/// // The trailing comma is always optional
/// injective_enum_map! { Empty, &'static str, &str }
///
/// assert_eq!(Empty::try_from("42"), Err(()))
/// ```
///
/// ## Intentionally violate injectivity:
/// ```
/// use bijective_enum_map::injective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum Version {
///     V1,
///     V2Any,
///     V2,
///     V2Alternate,
///     V3,
/// }
///
/// // Slightly extreme option to silence warnings; you should probably run your code without
/// // a similar allow attribute in order to check that the warning which occurs is as expected.
/// // It seems that `#[allow(unreachable_patterns)]` is overridden by
/// // the `#[warn(unreachable_patterns)]` inside the macro.
/// #[allow(warnings)]
/// {
///     injective_enum_map! {
///         Version, u8,
///         // Because injectivity is violated, the order is significant.
///         // With this order, `2` is mapped to the `V2Any` variant.
///         V1          <=> 1,
///         V2Any       <=> 2,
///         V2          <=> 2,
///         V2Alternate <=> 2,
///         V3          <=> 3,
///     }
/// }
///
/// assert_eq!(u8::from(Version::V2), 2);
/// assert_eq!(u8::from(Version::V3), 3);
/// assert_eq!(Version::try_from(1_u8), Ok(Version::V1));
/// assert_eq!(Version::try_from(2_u8), Ok(Version::V2Any));
/// assert_eq!(Version::try_from(5_u8), Err(()));
/// ```
///
/// ## Map into and from another enum:
/// ```
/// use bijective_enum_map::injective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum Enum {
///     One,
///     Two,
///     Three,
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Other {
///     Uno,
///     Dos,
///     Tres,
/// }
///
/// // Writing `Other` twice has the same effect as writing it once
/// injective_enum_map! {
///     Enum, Other, Other,
///     One   <=> Other::Uno,
///     Two   <=> Other::Dos,
///     Three <=> Other::Tres,
/// }
///
/// assert_eq!(Other::from(Enum::Three), Other::Tres);
/// // Note that this conversion cannot fail, but `injective_enum_map` does not know that.
/// // You could use `bijective_enum_map` instead.
/// assert_eq!(Enum::try_from(Other::Uno), Ok(Enum::One));
/// ```
#[macro_export]
macro_rules! injective_enum_map {
    { $enum_ty:ty, $into:ty, $try_from:ty, $($body:tt)* } => {
        $crate::__impl_from_enum! { $enum_ty, $into, $($body)* }
        $crate::__impl_enum_try_from! { $enum_ty, $try_from, $($body)* }
    };

    { $enum_ty:ty, $into:ty, $try_from:ty } => {
        $crate::__impl_from_enum! { $enum_ty, $into }
        $crate::__impl_enum_try_from! { $enum_ty, $try_from }
    };

    { $enum_ty:ty, $both:ty, $($body:tt)* } => {
        $crate::__impl_from_enum! { $enum_ty, $both, $($body)* }
        $crate::__impl_enum_try_from! { $enum_ty, $both, $($body)* }
    };

    { $enum_ty:ty, $both:ty } => {
        $crate::__impl_from_enum! { $enum_ty, $both }
        $crate::__impl_enum_try_from! { $enum_ty, $both }
    };
}


#[cfg(test)]
mod tests {
    use crate::injective_enum_map;

    #[test]
    fn empty_both_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum Empty {}

        injective_enum_map! {Empty, u8, u32}

        assert_eq!(Empty::try_from(2_u32), Err(()));
    }

    #[test]
    fn empty_one_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum Empty {}

        injective_enum_map! {Empty, u8}

        assert_eq!(Empty::try_from(2_u8), Err(()));
    }

    #[test]
    fn nonempty_both_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum AtMostTwo {
            Zero,
            One,
            Two,
        }

        injective_enum_map! {
            AtMostTwo, u8, u32,
            Zero <=> 0,
            One  <=> 1,
            Two  <=> 2,
        }

        assert_eq!(u8::from(AtMostTwo::One), 1_u8);
        assert_eq!(AtMostTwo::try_from(2_u32), Ok(AtMostTwo::Two));
        assert_eq!(AtMostTwo::try_from(4_u32), Err(()));
    }

    #[test]
    fn nonempty_one_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum AtMostTwo {
            Zero,
            One,
            Two,
        }

        injective_enum_map! {
            AtMostTwo, u32,
            Zero <=> 0,
            One  <=> 1,
            Two  <=> 2,
        }

        assert_eq!(u32::from(AtMostTwo::One), 1_u32);
        assert_eq!(AtMostTwo::try_from(2_u32), Ok(AtMostTwo::Two));
        assert_eq!(AtMostTwo::try_from(4_u32), Err(()));
    }

    #[test]
    fn nonempty_to_enum_bijective() {
        #[derive(Debug, PartialEq, Eq)]
        enum Enum {
            One,
            Two,
            Three,
        }

        #[derive(Debug, PartialEq, Eq)]
        enum Other {
            Uno,
            Dos,
            Tres,
        }

        injective_enum_map! {
            Enum, Other, Other,
            One   <=> Other::Uno,
            Two   <=> Other::Dos,
            Three <=> Other::Tres,
        }

        assert_eq!(Other::from(Enum::Three), Other::Tres);
        // Note that this conversion cannot fail, but `injective_enum_map` does not know that.
        assert_eq!(Enum::try_from(Other::Uno), Ok(Enum::One));
    }

    #[test]
    fn nonempty_to_enum_injective() {
        #[derive(Debug, PartialEq, Eq)]
        enum Enum {
            One,
            Two,
            Three,
        }

        #[derive(Debug, PartialEq, Eq)]
        enum Other {
            Uno,
            Dos,
            Tres,
            Cuatro,
        }

        injective_enum_map! {
            Enum, Other, Other,
            One   <=> Other::Uno,
            Two   <=> Other::Dos,
            Three <=> Other::Tres,
        }

        assert_eq!(Other::from(Enum::Three), Other::Tres);
        assert_eq!(Enum::try_from(Other::Uno), Ok(Enum::One));
        assert_eq!(Enum::try_from(Other::Cuatro), Err(()));
    }

    #[test]
    fn enum_to_string() {
        #[derive(Debug, PartialEq, Eq)]
        enum Empty {}

        #[derive(Debug, PartialEq, Eq)]
        enum Nonempty {
            Something,
        }

        injective_enum_map! {Empty, &'static str, &str}
        injective_enum_map! {
            Nonempty, &'static str, &str,
            Something <=> "Something",
        }

        assert_eq!(Empty::try_from("Anything"), Err(()));
        assert_eq!(Nonempty::try_from("Something"), Ok(Nonempty::Something));
        assert_eq!(Nonempty::try_from("Nothing"), Err(()));
    }

    #[test]
    fn trailing_commas() {
        enum Empty {}
        enum Nonempty {
            Something,
        }

        injective_enum_map!(Empty, u8, u8);
        injective_enum_map! { Empty, u16 };
        injective_enum_map! {
            Empty, i8, i8,
        };
        injective_enum_map! { Empty, i16, };

        injective_enum_map!(Nonempty, u8, u8, Something <=> 0);
        injective_enum_map! { Nonempty, u16, Something <=> 0};
        injective_enum_map! {
            Nonempty, i8, i8, Something <=> 0,
        };
        injective_enum_map! { Nonempty, i16, Something <=> 0,};
    }

    #[test]
    fn non_unit_variant() {
        // These would be strings, but don't want to pull in `alloc`.
        #[derive(Debug, PartialEq)]
        enum Thing {
            Player {
                name:     &'static str,
                hp:       u32,
                strength: f32,
            },
            PhysicalObject {
                name:     &'static str,
                hp:       u32,
            },
            Spell {
                name:     &'static str,
                strength: f32,
            },
            Marker(&'static str),
            HardcodedMarker(&'static str, u32),
        }

        type Stuff = (u8, Option<&'static str>, Option<u32>, Option<f32>);

        injective_enum_map! {
            Thing, Stuff,
            Player { name, hp, strength } <=> (0, Some(name), Some(hp), Some(strength)),
            PhysicalObject { name, hp }   <=> (1, Some(name), Some(hp), None),
            Spell { name, strength }      <=> (2, Some(name), None, Some(strength)),
            Marker(name)                  <=> (3, Some(name), None, None),
            HardcodedMarker(name, id)     <=> (4, Some(name), Some(id), None),
        }

        assert_eq!(
            Stuff::from(Thing::Player { name: "person", hp: 2, strength: 1.5 }),
            (0, Some("person"), Some(2), Some(1.5)),
        );
        assert_eq!(
            Stuff::from(Thing::Marker("place")),
            (3, Some("place"), None, None),
        );
        assert_eq!(
            Thing::try_from((1_u8, Some("object"), Some(100_u32), None)),
            Ok(Thing::PhysicalObject { name: "object", hp: 100 }),
        );
        assert_eq!(
            Thing::try_from((1_u8, Some("object"), Some(100_u32), Some(1e30))),
            Err(()),
        );
    }

    #[test]
    fn intentionally_non_injective() {
        #[derive(Debug, PartialEq, Eq)]
        enum Enum {
            One,
            Two,
            Three,
        }

        #[derive(Debug, PartialEq, Eq)]
        enum Other {
            Uno,
            Dos,
            Tres,
            Cuatro,
        }

        #[allow(warnings)]
        {
            injective_enum_map! {
                Other, Enum,
                Uno    <=> Enum::One,
                Dos    <=> Enum::Two,
                Tres   <=> Enum::Three,
                Cuatro <=> Enum::Three,
            }
        }

        assert_eq!(Other::try_from(Enum::Three), Ok(Other::Tres));
        assert_eq!(Enum::from(Other::Uno), Enum::One);
        assert_eq!(Enum::from(Other::Cuatro), Enum::Three);
    }
}

#[cfg(doctest)]
pub mod compile_fail_tests {
    /// ```compile_fail,E0004
    /// use bijective_enum_map::injective_enum_map;
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum Nonempty {
    ///     Something,
    /// }
    ///
    /// injective_enum_map! {Nonempty, u8}
    ///
    /// assert_eq!(Nonempty::try_from(2_u8), Err(()));
    /// ```
    pub fn _nonempty_but_nothing_provided() {}

    /// ```compile_fail,E0004
    /// use bijective_enum_map::injective_enum_map;
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum Nonempty {
    ///     Something,
    ///     SomethingElse,
    /// }
    ///
    /// injective_enum_map! { Nonempty, u8, Something <=> 0 }
    ///
    /// assert_eq!(Nonempty::try_from(2_u8), Err(()));
    /// ```
    pub fn _nonempty_but_not_enough_provided() {}

    /// ```compile_fail
    /// #![deny(warnings)]
    ///
    /// use bijective_enum_map::injective_enum_map;
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum AtMostTwo {
    ///     Zero,
    ///     One,
    ///     Two,
    /// }
    ///
    /// injective_enum_map! {
    ///     AtMostTwo, u8,
    ///     Zero <=> 0,
    ///     One  <=> 1,
    ///     Two  <=> 0,
    /// }
    /// ```
    pub fn _nonempty_not_injective_warning() {}

    /// ```compile_fail
    /// #![deny(warnings)]
    ///
    /// use bijective_enum_map::injective_enum_map;
    /// enum AtMostTwo {
    ///     Zero,
    ///     One,
    ///     Two,
    /// }
    ///
    /// enum BoolEnum {
    ///     True,
    ///     False,
    /// }
    ///
    /// injective_enum_map! {
    ///     bool, AtMostTwo,
    ///     False <=> AtMostTwo::Zero,
    ///     True  <=> AtMostTwo::One,
    ///     False <=> AtMostTwo::Two,
    /// }
    /// ```
    pub fn _nonempty_not_surjective_warning() {}

    // A warning is printed, but unfortunately, #[deny] doesn't work very well in doctests.
    // /// ```compile_fail
    // /// #![deny(unreachable_patterns)]
    // ///
    // /// use bijective_enum_map::injective_enum_map;
    // /// #[derive(Debug, PartialEq, Eq)]
    // /// enum AtMostTwo {
    // ///     Zero,
    // ///     One,
    // ///     Two,
    // /// }
    // ///
    // /// #[deny(unreachable_patterns)]
    // /// injective_enum_map! {
    // ///     AtMostTwo, u8,
    // ///     Zero <=> 0,
    // ///     One  <=> 1,
    // ///     Two  <=> 0,
    // /// }
    // /// ```
    // pub fn _nonempty_not_injective() {}

    /// ```compile_fail
    /// #![deny(warnings)]
    ///
    /// use bijective_enum_map::injective_enum_map;
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum AtMostTwo {
    ///     Zero,
    ///     One,
    ///     Two,
    /// }
    ///
    /// enum Other {
    ///     Uno,
    ///     Dos,
    /// }
    ///
    /// injective_enum_map! {
    ///     AtMostTwo, Other,
    ///     Zero <=> Other::Uno,
    ///     One  <=> Other::Uno,
    ///     Two  <=> Other::Dos,
    /// }
    ///
    /// let _ = AtMostTwo::try_from(Other::Uno);
    /// ```
    pub fn _nonempty_to_enum_not_injective_warning() {}

    // Surprisingly, this compiles. It defaults to `&'static str`, as far as I can tell.
    // /// ```compile_fail
    // /// use bijective_enum_map::injective_enum_map;
    // /// enum Nonempty {
    // ///     Something,
    // /// }
    // ///
    // /// injective_enum_map! {
    // ///     Nonempty, &str,
    // ///     Something <=> "Something",
    // /// }
    // ///
    // /// let _ = <&str>::from(Nonempty::Something);
    // /// ```
    // pub fn _enum_to_string_bad_lifetimes() {}

    // Doesn't seem to have a compiler error number
    /// ```compile_fail
    /// use bijective_enum_map::injective_enum_map;
    /// enum Nonempty {
    ///     Something,
    /// }
    ///
    /// injective_enum_map! {
    ///     Nonempty, u8
    ///     Something <=> 0
    /// }
    /// ```
    pub fn _missing_comma() {}
}
