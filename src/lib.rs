#![no_std]

// The helper macros in this module should not be considered part of the public API
// (for either usage or semver purposes).
mod helpers;


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
/// from `&str`, for example.
///
/// This map is intended to be "injective"; different enum variants should map into different
/// values, so that they can be mapped back unambiguously. The map may (or may not) also be
/// "surjective", in which any possible value of the target type is associated with some enum
/// variant, in which case the `TryFrom` implementation would not be able to fail (but this macro
/// does not check for surjectivity). If the map is not injective, and multiple enum variants map
/// to the same value, then a compiler warning from `#[warn(unreachable_patterns)]` *should* be
/// printed, but depending on circumstances it could be a silent logic error. In such a case,
/// only the first enum variant listed will be mapped from the duplicate value.
///
/// # Examples
///
/// ## Nonempty map, into and from the same type:
/// ```
/// # use prismarine_anchor_util::injective_enum_map;
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
/// assert_eq!(u8::from(AtMostTwo::One), 1_u8);
/// assert_eq!(AtMostTwo::try_from(2_u8), Ok(AtMostTwo::Two));
/// assert_eq!(AtMostTwo::try_from(4_u8), Err(()));
/// ```
///
/// ## Empty map, into and from different types:
/// ```
/// # use prismarine_anchor_util::injective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum Empty {}
///
/// // The trailing comma is always optional
/// injective_enum_map! { Empty, &'static str, &str }
///
/// assert_eq!(Empty::try_from("42"), Err(()))
/// ```
///
/// ## Nonempty map, into and from the same type explicitly written twice:
/// ```
/// # use prismarine_anchor_util::injective_enum_map;
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
/// injective_enum_map! {
///     Enum, Other, Other,
///     One   <=> Other::Uno,
///     Two   <=> Other::Dos,
///     Three <=> Other::Tres,
/// }
///
/// assert_eq!(Other::from(Enum::Three), Other::Tres);
/// // Note that this conversion cannot fail, but `injective_enum_map` does not know that.
/// // You could manually implement `From` by unwrapping the result of `try_from`.
/// assert_eq!(Enum::try_from(Other::Uno), Ok(Enum::One));
/// ```
#[macro_export]
macro_rules! injective_enum_map {
    { $enum_name:ty, $into:ty, $try_from:ty, $($body:tt)* } => {
        $crate::__impl_from_enum! { $enum_name, $into, $($body)* }
        $crate::__impl_enum_try_from! { $enum_name, $try_from, $($body)* }
    };

    { $enum_name:ty, $into:ty, $try_from:ty } => {
        $crate::__impl_from_enum! { $enum_name, $into }
        $crate::__impl_enum_try_from! { $enum_name, $try_from }
    };

    { $enum_name:ty, $both:ty, $($body:tt)* } => {
        $crate::__impl_from_enum! { $enum_name, $both, $($body)* }
        $crate::__impl_enum_try_from! { $enum_name, $both, $($body)* }
    };

    { $enum_name:ty, $both:ty } => {
        $crate::__impl_from_enum! { $enum_name, $both }
        $crate::__impl_enum_try_from! { $enum_name, $both }
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
}

#[cfg(doctest)]
pub mod compile_fail_tests {
    /// ```compile_fail,E0004
    /// use prismarine_anchor_util::injective_enum_map;
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
    /// use prismarine_anchor_util::injective_enum_map;
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
    /// use prismarine_anchor_util::injective_enum_map;
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

    // A warning is printed, but unfortunately, #[deny] doesn't work very well in doctests.
    // /// ```compile_fail
    // /// #![deny(unreachable_patterns)]
    // ///
    // /// use prismarine_anchor_util::injective_enum_map;
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
    /// use prismarine_anchor_util::injective_enum_map;
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
    // /// use prismarine_anchor_util::injective_enum_map;
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
    /// use prismarine_anchor_util::injective_enum_map;
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
