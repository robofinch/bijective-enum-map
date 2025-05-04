#[doc(hidden)]
#[macro_export]
macro_rules! __impl_from_enum {
    {
        $enum_ty:ty,
        $into:ty,
        $($enum_variant:ident$(($($tuple:tt)*))?$({$($struct:tt)*})? <=> $value:expr),+
        $(,)?
    } => {
        impl ::core::convert::From<$enum_ty> for $into {
            #[inline]
            fn from(value: $enum_ty) -> Self {
                // This is because we can't do <$enum_ty>::$enum_variant
                // (that syntax is unstable/experimental in that position)
                use $enum_ty as __enum_ty;
                #[warn(unreachable_patterns)]
                match value {
                    $( __enum_ty::$enum_variant$(($($tuple)*))?$({$($struct)*})? => $value ),+
                }
            }
        }
    };

    { $enum_ty:ty, $into:ty $(,)? } => {
        impl ::core::convert::From<$enum_ty> for $into {
            #[inline]
            fn from(value: $enum_ty) -> Self {
                match value {}
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_enum_from {
    {
        $enum_ty:ty,
        $from:ty,
        $($enum_variant:ident$(($($tuple:tt)*))?$({$($struct:tt)*})? <=> $value:pat),+
        $(,)?
    } => {
        impl ::core::convert::From<$from> for $enum_ty {
            #[inline]
            fn from(value: $from) -> Self {
                #[warn(unreachable_patterns)]
                match value {
                    $( $value => Self::$enum_variant$(($($tuple)*))?$({$($struct)*})? ),+
                }
            }
        }
    };

    { $enum_ty:ty, $from:ty $(,)? } => {
        impl ::core::convert::From<$from> for $enum_ty {
            #[inline]
            fn from(value: $from) -> Self {
                match value {}
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_enum_try_from {
    {
        $enum_ty:ty,
        $try_from:ty,
        $($enum_variant:ident$(($($tuple:tt)*))?$({$($struct:tt)*})? <=> $value:pat),+
        $(,)?
    } => {
        impl ::core::convert::TryFrom<$try_from> for $enum_ty {
            type Error = ();

            #[inline]
            fn try_from(value: $try_from) -> Result<Self, Self::Error> {
                #![allow(clippy::allow_attributes)]
                #[warn(unreachable_patterns)]
                Ok(match value {
                    $( $value => Self::$enum_variant$(($($tuple)*))?$({$($struct)*})? ),+,
                    #[allow(clippy::wildcard_enum_match_arm)]
                    #[allow(unreachable_patterns)]
                    _ => return Err(()),
                })
            }
        }
    };

    { $enum_ty:ty, $try_from:ty $(,)? } => {
        impl ::core::convert::TryFrom<$try_from> for $enum_ty {
            type Error = ();

            #[inline]
            fn try_from(_value: $try_from) -> Result<Self, Self::Error> {
                Err(())
            }
        }
    };
}
