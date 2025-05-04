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
/// (no tricks with `|`, `..`, and the like), surjectivity must hold.
///
/// Being injective means that different enum variants should map into different
/// values, so that they can be mapped back unambiguously. If the map is not injective, and
/// multiple enum variants map to the same value, then a compiler warning from
/// `#[warn(unreachable_patterns)]` *should* be printed in most circumstances, but it could
/// be a silent logic error. In such a case, only the first enum variant listed will be mapped
/// from the duplicate value.
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
/// fn new_empty() -> Empty {
///     panic!()
/// }
/// fn new_another_empty() -> AnotherEmpty {
///     AnotherEmpty::from(new_empty())
/// }
/// fn round_trip() -> Empty {
///     Empty::from(new_another_empty())
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
