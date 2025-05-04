/// Map an enum into and from another type (or two types) using `From` in each direction.
///
/// The enum type must be specified, followed by the type to map the enum into (`$into`),
/// optionally followed by the type to map into the enum (`$from`).
/// If `$from` is not specified, it is set to `$into`.
///
/// The two types must be similar enough that the same value expression (e.g., a numeric or string
/// literal) works for either; moreover, the expression must also be a valid pattern for a match
/// arm, so arbitrarily complicated expressions are not permitted.
/// In practice, the types should usually be the same, but specifying two different types could be
/// used for converting into a static reference and trying to convert from a reference of
/// unspecified lifetime, for example. (Such usage is more useful in `injective_enum_map`.)
/// Unintended options like mapping into `Range<u8>` and from `u8` might be possible, but are
/// not tested here. Note that non-unit variants are supported.
///
/// This map is intended to be "bijective", which means that it is both "surjective" and
/// "injective". Being surjective means that every value of the target type to map the enum into
/// should be associated with some enum variant. This is enforced by a `match` mapping values into
/// enum variants; if your value expressions/patterns are fairly normal
/// (no tricks with `|`, `..`, and the like), surjectivity will hold unless you repeat enum
/// variants (causing some arms to be unreachable when mapping variants to values).
///
/// Being injective means that different enum variants should map into different
/// values, so that they can be mapped back unambiguously.
///
/// If the map is not bijective, and either some enum variants are repeated or multiple enum
/// variants map to the same value, then a compiler warning from `#[warn(unreachable_patterns)]`
/// *should* be printed in most circumstances, but it could be a silent logic error. In such a
/// case, only the first duplicate arm (in each direction) will be taken for the duplicated variant
/// or value.
///
/// # Examples
///
/// ## Map into and from two other types:
/// ```
/// use bijective_enum_map::bijective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum AtMostTwo {
///     Zero,
///     One,
///     Two,
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Other {
///     Zeroeth,
///     First,
///     Second,
/// }
///
/// bijective_enum_map! {
///     AtMostTwo, Option<bool>,
///     Zero <=> Some(false),
///     One  <=> Some(true),
///     Two  <=> None,
/// }
///
/// // You can specify the same type twice for from/into, it has no effect.
/// // Note that the path to other enums does have to be specified.
/// bijective_enum_map! {
///     AtMostTwo, Other, Other,
///     Zero <=> Other::Zeroeth,
///     One  <=> Other::First,
///     Two  <=> Other::Second,
/// }
///
/// // The compiler can infer that this is `Option::<bool>::from`
/// assert_eq!(Option::from(AtMostTwo::One), Some(true));
/// assert_eq!(AtMostTwo::from(None), AtMostTwo::Two);
/// assert_eq!(AtMostTwo::from(Other::Zeroeth), AtMostTwo::Zero);
/// ```
///
/// ## Map with a non-unit variant:
/// ```
/// use bijective_enum_map::bijective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum MaybeData {
///     Data(String),
///     Nothing,
/// }
///
/// bijective_enum_map! {
///     MaybeData, Option<String>,
///     Data(data) <=> Some(data),
///     Nothing    <=> None,
/// }
///
/// assert_eq!(
///     MaybeData::from(Some("What do you get when you multiply six by nine?".to_owned())),
///     MaybeData::Data("What do you get when you multiply six by nine?".to_owned()),
/// );
/// assert_eq!(
///     Option::from(MaybeData::Data("42".to_owned())),
///     Some("42".to_owned()),
/// );
/// ```
///
/// ## Intentionally violate injectivity:
/// ```
/// use bijective_enum_map::bijective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum ParsedString {
///     Quoted(String),
///     Unquoted(String),
///     Unknown(String),
/// }
///
/// // Slightly extreme option to silence warnings; you should probably run your code without
/// // a similar allow attribute in order to check that the warning which occurs is as expected.
/// // It seems that `#[allow(unreachable_patterns)]` is overridden by
/// // the `#[warn(unreachable_patterns)]` inside the macro.
/// #[allow(warnings)]
/// {
///     bijective_enum_map! {
///         ParsedString, String,
///         // Because injectivity is violated, the order is significant.
///         // With this order, all incoming strings are mapped to the `Unknown` variant.
///         Unknown(string)  <=> string,
///         Quoted(string)   <=> string,
///         Unquoted(string) <=> string,
///     }
/// }
///
/// assert_eq!(
///     String::from(ParsedString::Quoted("string".to_owned())),
///     "string".to_owned(),
/// );
/// assert_eq!(
///     String::from(ParsedString::Unquoted("string".to_owned())),
///     "string".to_owned(),
/// );
/// assert_eq!(
///     ParsedString::from("string".to_owned()),
///     ParsedString::Unknown("string".to_owned()),
/// );
/// ```
///
/// ## Empty enum, mapped into and from another type:
/// ```
/// use bijective_enum_map::bijective_enum_map;
/// #[derive(Debug, PartialEq, Eq)]
/// enum Empty {}
/// enum AnotherEmpty {}
///
/// // The trailing comma is always optional
/// bijective_enum_map! { Empty, AnotherEmpty }
///
/// // The below is to confirm that the appropriate `From` implementations exist
/// fn _new_empty() -> Empty {
///     panic!()
/// }
/// fn _new_another_empty() -> AnotherEmpty {
///     AnotherEmpty::from(_new_empty())
/// }
/// fn _round_trip() -> Empty {
///     Empty::from(_new_another_empty())
/// }
/// ```
#[macro_export]
macro_rules! bijective_enum_map {
    { $enum_ty:ty, $into:ty, $from:ty, $($body:tt)* } => {
        $crate::__impl_from_enum! { $enum_ty, $into, $($body)* }
        $crate::__impl_enum_from! { $enum_ty, $from, $($body)* }
    };

    { $enum_ty:ty, $into:ty, $from:ty } => {
        $crate::__impl_from_enum! { $enum_ty, $into }
        $crate::__impl_enum_from! { $enum_ty, $from }
    };

    { $enum_ty:ty, $both:ty, $($body:tt)* } => {
        $crate::__impl_from_enum! { $enum_ty, $both, $($body)* }
        $crate::__impl_enum_from! { $enum_ty, $both, $($body)* }
    };

    { $enum_ty:ty, $both:ty } => {
        $crate::__impl_from_enum! { $enum_ty, $both }
        $crate::__impl_enum_from! { $enum_ty, $both }
    };
}


#[cfg(test)]
mod tests {
    use crate::bijective_enum_map;

    #[test]
    fn empty_both_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum Empty {}
        enum AnotherEmpty {}

        // The trailing comma is always optional
        bijective_enum_map! { Empty, AnotherEmpty, AnotherEmpty }

        // The below is to confirm that the appropriate `From` implementations exist
        fn _new_empty() -> Empty {
            panic!()
        }
        fn _new_another_empty() -> AnotherEmpty {
            AnotherEmpty::from(_new_empty())
        }
        fn _round_trip() -> Empty {
            Empty::from(_new_another_empty())
        }
    }

    #[test]
    fn empty_one_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum Empty {}
        enum AnotherEmpty {}

        // The trailing comma is always optional
        bijective_enum_map! { Empty, AnotherEmpty, }

        // The below is to confirm that the appropriate `From` implementations exist
        fn _new_empty() -> Empty {
            panic!()
        }
        fn _new_another_empty() -> AnotherEmpty {
            AnotherEmpty::from(_new_empty())
        }
        fn _round_trip() -> Empty {
            Empty::from(_new_another_empty())
        }
    }

    #[test]
    fn nonempty_both_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum Trivial {
            Num(u8),
        }

        bijective_enum_map! {Trivial, u8, u8, Num(num) <=> num}

        assert_eq!(Trivial::from(2_u8), Trivial::Num(2));
        assert_eq!(u8::from(Trivial::Num(3)), 3);
    }

    #[test]
    fn nonempty_one_specified() {
        #[derive(Debug, PartialEq, Eq)]
        enum Trivial {
            Num(u8),
        }

        bijective_enum_map! {Trivial, u8, Num(num) <=> num}

        assert_eq!(Trivial::from(2_u8), Trivial::Num(2));
        assert_eq!(u8::from(Trivial::Num(3)), 3);
    }

    #[test]
    fn nonempty_enums() {
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

        bijective_enum_map! {
            Enum, Other, Other,
            One   <=> Other::Uno,
            Two   <=> Other::Dos,
            Three <=> Other::Tres,
        }

        assert_eq!(Other::from(Enum::Three), Other::Tres);
        assert_eq!(Enum::from(Other::Uno), Enum::One);
    }

    #[test]
    fn trailing_commas() {
        enum Empty {}
        enum AnotherEmpty {}
        enum YetAnotherEmpty {}
        enum AFourthEmpty {}

        enum Trivial {
            Num(u8),
        }
        enum Trivial2 {
            Num(u16),
        }
        enum Trivial3 {
            Num(i8),
        }
        enum Trivial4 {
            Num(i16),
        }

        bijective_enum_map!(Empty, AnotherEmpty, AnotherEmpty);
        bijective_enum_map! { Empty, YetAnotherEmpty };
        bijective_enum_map! {
            AnotherEmpty, YetAnotherEmpty, YetAnotherEmpty,
        };
        bijective_enum_map! { Empty, AFourthEmpty, };

        bijective_enum_map!(Trivial, u8, u8, Num(num) <=> num);
        bijective_enum_map! { Trivial2, u16, Num(num) <=> num };
        bijective_enum_map! {
            Trivial3, i8, i8, Num(num) <=> num,
        };
        bijective_enum_map! { Trivial4, i16, Num(num) <=> num, };
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
            Unknown(Stuff),
        }

        type Stuff = (u8, Option<&'static str>, Option<u32>, Option<f32>);

        bijective_enum_map! {
            Thing, Stuff,
            Player { name, hp, strength } <=> (0, Some(name), Some(hp), Some(strength)),
            PhysicalObject { name, hp }   <=> (1, Some(name), Some(hp), None),
            Spell { name, strength }      <=> (2, Some(name), None, Some(strength)),
            Marker(name)                  <=> (3, Some(name), None, None),
            HardcodedMarker(name, id)     <=> (4, Some(name), Some(id), None),
            Unknown(stuff)                <=> stuff,
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
            Thing::from((1_u8, Some("object"), Some(100_u32), None)),
            Thing::PhysicalObject { name: "object", hp: 100 },
        );
        assert_eq!(
            Thing::from((1_u8, Some("object"), Some(100_u32), Some(1e30))),
            Thing::Unknown((1_u8, Some("object"), Some(100_u32), Some(1e30))),
        );
    }

    #[test]
    fn intentionally_non_surjective() {
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
            bijective_enum_map! {
                Enum, Other, Other,
                One   <=> Other::Uno,
                Two   <=> Other::Dos,
                Three <=> Other::Tres,
                Three <=> Other::Cuatro,
            }
        }

        assert_eq!(Other::from(Enum::Three), Other::Tres);
        assert_eq!(Enum::from(Other::Uno), Enum::One);
        assert_eq!(Enum::from(Other::Cuatro), Enum::Three);
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
            bijective_enum_map! {
                Other, Enum,
                Uno    <=> Enum::One,
                Dos    <=> Enum::Two,
                Tres   <=> Enum::Three,
                Cuatro <=> Enum::Three,
            }
        }

        assert_eq!(Other::from(Enum::Three), Other::Tres);
        assert_eq!(Enum::from(Other::Uno), Enum::One);
        assert_eq!(Enum::from(Other::Cuatro), Enum::Three);
    }
}

#[cfg(doctest)]
pub mod compile_fail_tests {
    /// ```compile_fail,E0004
    /// use bijective_enum_map::bijective_enum_map;
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum Nonempty {
    ///     Something,
    /// }
    ///
    /// bijective_enum_map! {Nonempty, u8}
    /// ```
    pub fn _nonempty_but_nothing_provided() {}

    /// ```compile_fail,E0004
    /// use bijective_enum_map::bijective_enum_map;
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum Nonempty {
    ///     Something,
    ///     SomethingElse,
    /// }
    ///
    /// bijective_enum_map! { Nonempty, u8, Something <=> 0 }
    /// ```
    pub fn _nonempty_but_not_enough_provided() {}

    /// ```compile_fail
    /// #![deny(warnings)]
    ///
    /// use bijective_enum_map::bijective_enum_map;
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum AtMostTwo {
    ///     Zero,
    ///     One,
    ///     Two,
    /// }
    ///
    /// bijective_enum_map! {
    ///     AtMostTwo, bool,
    ///     Zero <=> false,
    ///     One  <=> true,
    ///     Two  <=> false,
    /// }
    /// ```
    pub fn _nonempty_not_injective_warning() {}

    /// ```compile_fail
    /// #![deny(warnings)]
    ///
    /// use bijective_enum_map::bijective_enum_map;
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
    /// bijective_enum_map! {
    ///     bool, AtMostTwo,
    ///     False <=> AtMostTwo::Zero,
    ///     True  <=> AtMostTwo::One,
    ///     False <=> AtMostTwo::Two,
    /// }
    /// ```
    pub fn _nonempty_not_surjective_warning() {}

    /// ```compile_fail
    /// #![deny(warnings)]
    ///
    /// use bijective_enum_map::bijective_enum_map;
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
    /// bijective_enum_map! {
    ///     AtMostTwo, Other,
    ///     Zero <=> Other::Uno,
    ///     One  <=> Other::Uno,
    ///     Two  <=> Other::Dos,
    /// }
    ///
    /// let _ = AtMostTwo::from(Other::Uno);
    /// ```
    pub fn _nonempty_to_enum_not_injective_warning() {}

    // Doesn't seem to have a compiler error number
    /// ```compile_fail
    /// use bijective_enum_map::bijective_enum_map;
    /// enum Nonempty {
    ///     Something,
    /// }
    ///
    /// bijective_enum_map! {
    ///     Nonempty, ()
    ///     Something <=> ()
    /// }
    /// ```
    pub fn _missing_comma() {}

    /// ```
    /// use bijective_enum_map::bijective_enum_map;
    /// enum Nonempty {
    ///     Something,
    /// }
    ///
    /// bijective_enum_map! {
    ///     Nonempty, (),
    ///     Something <=> ()
    /// }
    /// ```
    pub fn not_missing_comma() {}
}
