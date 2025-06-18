// SPDX-License-Identifier: GPL-2.0

//! Traits for type conversion.

/// A trait for fallible conversions from primitive types.
///
/// 1. General info
///     - When does this return None?
///     - When does this return Some?
/// 2. It has derive macro
/// 3. Default impl for the rest
///
/// # Examples
///
/// ```rust
/// use kernel::convert::FromPrimitive;
///
/// #[derive(PartialEq)]
/// enum Foo {
///     A,
///     B = 0x17,
///     C = -2,
/// }
///
/// impl FromPrimitive for Foo {
///     fn from_i64(n: i64) -> Option<Self> {
///         match n {
///             0 => Some(Self::A),
///             0x17 => Some(Self::B),
///             -2 => Some(Self::C),
///             _ => None,
///         }
///     }
///
///     fn from_u64(n: u64) -> Option<Self> {
///         i64::try_from(n).ok().and_then(Self::from_i64)
///     }
/// }
///
/// assert_eq!(Foo::from_u64(0), Some(Foo::A));
/// assert_eq!(Foo::from_u64(0x17), Some(Foo::B));
/// assert_eq!(Foo::from_i64(-2), Some(Foo::C));
/// assert_eq!(Foo::from_i64(-3), None);
/// ```
pub trait FromPrimitive: Sized {
    #[inline]
    fn from_bool(b: bool) -> Option<Self> {
        Self::from_u64(u64::from(b))
    }

    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        i64::try_from(n).ok().and_then(Self::from_i64)
    }

    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Self::from_i64(i64::from(n))
    }

    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Self::from_i64(i64::from(n))
    }

    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Self::from_i64(i64::from(n))
    }

    fn from_i64(n: i64) -> Option<Self>;

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        i64::try_from(n).ok().and_then(Self::from_i64)
    }

    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        u64::try_from(n).ok().and_then(Self::from_u64)
    }

    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Self::from_u64(u64::from(n))
    }

    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Self::from_u64(u64::from(n))
    }

    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Self::from_u64(u64::from(n))
    }

    fn from_u64(n: u64) -> Option<Self>;

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        u64::try_from(n).ok().and_then(Self::from_u64)
    }
}
