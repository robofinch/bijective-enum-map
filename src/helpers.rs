#[doc(hidden)]
#[macro_export]
macro_rules! __impl_from_enum {
    { $enum_name:ty, $into:ty, $($enum_variant:ident <=> $value:expr),+ $(,)? } => {
        impl ::core::convert::From<$enum_name> for $into {
            #[inline]
            fn from(value: $enum_name) -> Self {
                match value {
                    $( <$enum_name>::$enum_variant => $value ),+
                }
            }
        }
    };

    { $enum_name:ty, $into:ty $(,)? } => {
        impl ::core::convert::From<$enum_name> for $into {
            #[inline]
            fn from(value: $enum_name) -> Self {
                match value {}
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_enum_try_from {
    { $enum_name:ty, $try_from:ty, $($enum_variant:ident <=> $value:pat),+ $(,)? } => {
        impl ::core::convert::TryFrom<$try_from> for $enum_name {
            type Error = ();

            #[inline]
            fn try_from(value: $try_from) -> Result<Self, Self::Error> {
                #![allow(clippy::allow_attributes)]
                #[warn(unreachable_patterns)]
                Ok(match value {
                    $( $value => Self::$enum_variant ),+,
                    #[allow(clippy::wildcard_enum_match_arm)]
                    #[allow(unreachable_patterns)]
                    _ => return Err(()),
                })
            }
        }
    };

    { $enum_name:ty, $try_from:ty $(,)? } => {
        impl ::core::convert::TryFrom<$try_from> for $enum_name {
            type Error = ();

            #[inline]
            fn try_from(_value: $try_from) -> Result<Self, Self::Error> {
                Err(())
            }
        }
    };
}
