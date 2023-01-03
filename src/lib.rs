#![allow(non_camel_case_types)]

mod internal {
    #![allow(non_upper_case_globals)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Signed integer type with width of exactly 8 bits.
pub type int8_t = i8;

/// Signed integer type with width of exactly 16 bits.
pub type int16_t = i16;

/// Signed integer type with width of exactly 32 bits.
pub type int32_t = i32;

/// Signed integer type with width of exactly 64 bits.
pub type int64_t = i64;

/// Unsigned integer type with width of exactly 8 bits.
pub type uint8_t = u8;

/// Unsigned integer type with width of exactly 16 bits.
pub type uint16_t = u16;

/// Unsigned integer type with width of exactly 32 bits.
pub type uint32_t = u32;

/// Unsigned integer type with width of exactly 64 bits.
pub type uint64_t = u64;

/// Fastest signed integer type with width of at least 8 bits.
pub type int_fast8_t = internal::int_fast8_t;

/// Fastest signed integer type with width of at least 16 bits.
pub type int_fast16_t = internal::int_fast16_t;

/// Fastest signed integer type with width of at least 32 bits.
pub type int_fast32_t = internal::int_fast32_t;

/// Fastest signed integer type with width of at least 64 bits.
pub type int_fast64_t = internal::int_fast64_t;

/// Fastest unsigned integer type with width of at least 8 bits.
pub type uint_fast8_t = internal::uint_fast8_t;

/// Fastest unsigned integer type with width of at least 16 bits.
pub type uint_fast16_t = internal::uint_fast16_t;

/// Fastest unsigned integer type with width of at least 32 bits.
pub type uint_fast32_t = internal::uint_fast32_t;

/// Fastest unsigned integer type with width of at least 64 bits.
pub type uint_fast64_t = internal::uint_fast64_t;

/// Smallest signed integer type with width of at least 8 bits.
pub type int_least8_t = internal::int_least8_t;

/// Smallest signed integer type with width of at least 16 bits.
pub type int_least16_t = internal::int_least16_t;

/// Smallest signed integer type with width of at least 32 bits.
pub type int_least32_t = internal::int_least32_t;

/// Smallest signed integer type with width of at least 64 bits.
pub type int_least64_t = internal::int_least64_t;

/// Smallest unsigned integer type with width of at least 8 bits.
pub type uint_least8_t = internal::uint_least8_t;

/// Smallest unsigned integer type with width of at least 16 bits.
pub type uint_least16_t = internal::uint_least16_t;

/// Smallest unsigned integer type with width of at least 32 bits.
pub type uint_least32_t = internal::uint_least32_t;

/// Smallest unsigned integer type with width of at least 64 bits.
pub type uint_least64_t = internal::uint_least64_t;

/// The unsigned integer type of the result of the `sizeof` operator.
pub type size_t = internal::_size_t;

/// Used for a count of bytes or an error indication.
pub type ssize_t = internal::_ssize_t;

/// Maximum-width signed integer type.
pub type intmax_t = internal::intmax_t;

/// Maximum-width unsigned integer type.
pub type uintmax_t = internal::uintmax_t;

/// Signed integer type capable of holding a pointer to [`std::ffi::c_void`].
pub type intptr_t = internal::_intptr_t;

/// Unsigned integer type capable of holding a pointer to [`std::ffi::c_void`].
pub type uintptr_t = internal::_uintptr_t;

/// Minimum value of an object of type [`int8_t`].
pub const INT8_MIN: int8_t = internal::INT8_MIN as _;

/// Maximum value of an object of type [`int8_t`].
pub const INT8_MAX: int8_t = internal::INT8_MAX as _;

/// Maximum value of an object of type [`uint8_t`].
pub const UINT8_MAX: uint8_t = internal::UINT8_MAX as _;

/// Minimum value of an object of type [`int16_t`].
pub const INT16_MIN: int16_t = internal::INT16_MIN as _;

/// Maximum value of an object of type [`int16_t`].
pub const INT16_MAX: int16_t = internal::INT16_MAX as _;

/// Maximum value of an object of type [`uint16_t`].
pub const UINT16_MAX: uint16_t = internal::UINT16_MAX as _;

/// Minimum value of an object of type [`int32_t`].
pub const INT32_MIN: int32_t = internal::INT32_MIN as _;

/// Maximum value of an object of type [`int32_t`].
pub const INT32_MAX: int32_t = internal::INT32_MAX as _;

/// Maximum value of an object of type [`uint32_t`].
pub const UINT32_MAX: uint32_t = internal::UINT32_MAX as _;

/// Minimum value of an object of type [`int64_t`].
pub const INT64_MIN: int64_t = internal::RUST_C99_INT64_MIN as _;

/// Maximum value of an object of type [`int64_t`].
pub const INT64_MAX: int64_t = internal::RUST_C99_INT64_MAX as _;

/// Maximum value of an object of type [`uint64_t`].
pub const UINT64_MAX: uint64_t = uint64_t::MAX as _;

/// Minimum value of an object of type [`int_fast8_t`].
pub const INT_FAST8_MIN: int_fast8_t = internal::INT_FAST8_MIN as _;

/// Maximum value of an object of type [`int_fast8_t`].
pub const INT_FAST8_MAX: int_fast8_t = internal::INT_FAST8_MAX as _;

/// Maximum value of an object of type [`uint_fast8_t`].
pub const UINT_FAST8_MAX: uint_fast8_t = internal::UINT_FAST8_MAX as _;

/// Minimum value of an object of type [`int_fast16_t`].
pub const INT_FAST16_MIN: int_fast16_t = internal::INT_FAST16_MIN as _;

/// Maximum value of an object of type [`int_fast16_t`].
pub const INT_FAST16_MAX: int_fast16_t = internal::INT_FAST16_MAX as _;

/// Maximum value of an object of type [`uint_fast16_t`].
pub const UINT_FAST16_MAX: uint_fast16_t = internal::UINT_FAST16_MAX as _;

/// Minimum value of an object of type [`int_fast32_t`].
pub const INT_FAST32_MIN: int_fast32_t = internal::INT_FAST32_MIN as _;

/// Maximum value of an object of type [`int_fast32_t`].
pub const INT_FAST32_MAX: int_fast32_t = internal::INT_FAST32_MAX as _;

/// Maximum value of an object of type [`uint_fast32_t`].
pub const UINT_FAST32_MAX: uint_fast32_t = internal::UINT_FAST32_MAX as _;

/// Minimum value of an object of type [`int_fast64_t`].
pub const INT_FAST64_MIN: int_fast64_t = internal::RUST_C99_INT_FAST64_MIN as _;

/// Maximum value of an object of type [`int_fast64_t`].
pub const INT_FAST64_MAX: int_fast64_t = internal::RUST_C99_INT_FAST64_MAX as _;

/// Maximum value of an object of type [`uint_fast64_t`].
pub const UINT_FAST64_MAX: uint_fast64_t = uint_fast64_t::MAX as _;

/// Minimum value of an object of type [`int_least8_t`].
pub const INT_LEAST8_MIN: int_least8_t = internal::INT_LEAST8_MIN as _;

/// Maximum value of an object of type [`int_least8_t`].
pub const INT_LEAST8_MAX: int_least8_t = internal::INT_LEAST8_MAX as _;

/// Maximum value of an object of type [`uint_least8_t`].
pub const UINT_LEAST8_MAX: uint_least8_t = internal::UINT_LEAST8_MAX as _;

/// Minimum value of an object of type [`int_least16_t`].
pub const INT_LEAST16_MIN: int_least16_t = internal::INT_LEAST16_MIN as _;

/// Maximum value of an object of type [`int_least16_t`].
pub const INT_LEAST16_MAX: int_least16_t = internal::INT_LEAST16_MAX as _;

/// Maximum value of an object of type [`uint_least16_t`].
pub const UINT_LEAST16_MAX: uint_least16_t = internal::UINT_LEAST16_MAX as _;

/// Minimum value of an object of type [`int_least32_t`].
pub const INT_LEAST32_MIN: int_least32_t = internal::INT_LEAST32_MIN as _;

/// Maximum value of an object of type [`int_least32_t`].
pub const INT_LEAST32_MAX: int_least32_t = internal::INT_LEAST32_MAX as _;

/// Maximum value of an object of type [`uint_least32_t`].
pub const UINT_LEAST32_MAX: uint_least32_t = internal::UINT_LEAST32_MAX as _;

/// Minimum value of an object of type [`int_least64_t`].
pub const INT_LEAST64_MIN: int_least64_t = internal::RUST_C99_INT_LEAST64_MIN as _;

/// Maximum value of an object of type [`int_least64_t`].
pub const INT_LEAST64_MAX: int_least64_t = internal::RUST_C99_INT_LEAST64_MAX as _;

/// Maximum value of an object of type [`uint_least64_t`].
pub const UINT_LEAST64_MAX: uint_least64_t = uint_least64_t::MAX as _;

/// Minimum value of an object of type [`intptr_t`].
pub const INTPTR_MAX: intptr_t = internal::INTPTR_MAX as _;

/// Maximum value of an object of type [`intptr_t`].
pub const INTPTR_MIN: intptr_t = internal::INTPTR_MIN as _;

/// Maximum value of an object of type [`uintptr_t`].
pub const UINTPTR_MAX: uintptr_t = internal::UINTPTR_MAX as _;

/// Minimum value of an object of type [`intmax_t`].
pub const INTMAX_MAX: intmax_t = internal::RUST_C99_INTMAX_MAX as _;

/// Maximum value of an object of type [`intmax_t`].
pub const INTMAX_MIN: intmax_t = internal::RUST_C99_INT_INTMAX_MIN as _;

/// Maximum value of an object of type [`uintmax_t`].
pub const UINTMAX_MAX: uintmax_t = uintmax_t::MAX as _;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn int8() {
        assert_eq!(size_of::<int8_t>(), 1);
        assert_eq!(size_of::<uint8_t>(), 1);
        assert!(size_of::<int_least8_t>() >= 1);
        assert!(size_of::<uint_least8_t>() >= 1);
        assert!(size_of::<int_fast8_t>() >= 1);
        assert!(size_of::<uint_fast8_t>() >= 1);

        assert_eq!(int8_t::MIN, INT8_MIN as _);
        assert_eq!(int8_t::MAX, INT8_MAX as _);
        assert_eq!(int_fast8_t::MIN, INT_FAST8_MIN as _);
        assert_eq!(int_fast8_t::MAX, INT_FAST8_MAX as _);
        assert_eq!(int_least8_t::MIN, INT_LEAST8_MIN as _);
        assert_eq!(int_least8_t::MAX, INT_LEAST8_MAX as _);

        assert_eq!(uint8_t::MAX, UINT8_MAX as _);
        assert_eq!(uint_fast8_t::MAX, UINT_FAST8_MAX as _);
        assert_eq!(uint_least8_t::MAX, UINT_LEAST8_MAX as _);
    }

    #[test]
    fn int16() {
        assert_eq!(size_of::<int16_t>(), 2);
        assert_eq!(size_of::<uint16_t>(), 2);
        assert!(size_of::<int_least16_t>() >= 2);
        assert!(size_of::<uint_least16_t>() >= 2);
        assert!(size_of::<int_fast16_t>() >= 2);
        assert!(size_of::<uint_fast16_t>() >= 2);

        assert_eq!(int16_t::MIN, INT16_MIN as _);
        assert_eq!(int16_t::MAX, INT16_MAX as _);
        assert_eq!(int_fast16_t::MIN, INT_FAST16_MIN as _);
        assert_eq!(int_fast16_t::MAX, INT_FAST16_MAX as _);
        assert_eq!(int_least16_t::MIN, INT_LEAST16_MIN as _);
        assert_eq!(int_least16_t::MAX, INT_LEAST16_MAX as _);

        assert_eq!(uint16_t::MAX, UINT16_MAX as _);
        assert_eq!(uint_fast16_t::MAX, UINT_FAST16_MAX as _);
        assert_eq!(uint_least16_t::MAX, UINT_LEAST16_MAX as _);
    }

    #[test]
    fn int32() {
        assert_eq!(size_of::<int32_t>(), 4);
        assert_eq!(size_of::<uint32_t>(), 4);
        assert!(size_of::<int_least32_t>() >= 4);
        assert!(size_of::<uint_least32_t>() >= 4);
        assert!(size_of::<int_fast32_t>() >= 4);
        assert!(size_of::<uint_fast32_t>() >= 4);

        assert_eq!(int32_t::MIN, INT32_MIN as _);
        assert_eq!(int32_t::MAX, INT32_MAX as _);
        assert_eq!(int_fast32_t::MIN, INT_FAST32_MIN as _);
        assert_eq!(int_fast32_t::MAX, INT_FAST32_MAX as _);
        assert_eq!(int_least32_t::MIN, INT_LEAST32_MIN as _);
        assert_eq!(int_least32_t::MAX, INT_LEAST32_MAX as _);

        assert_eq!(uint32_t::MAX, UINT32_MAX as _);
        assert_eq!(uint_fast32_t::MAX, UINT_FAST32_MAX as _);
        assert_eq!(uint_least32_t::MAX, UINT_LEAST32_MAX as _);
    }

    #[test]
    fn int64() {
        assert_eq!(size_of::<int64_t>(), 8);
        assert_eq!(size_of::<uint64_t>(), 8);
        assert!(size_of::<int_least64_t>() >= 8);
        assert!(size_of::<uint_least64_t>() >= 8);
        assert!(size_of::<int_fast64_t>() >= 8);
        assert!(size_of::<uint_fast64_t>() >= 8);

        assert_eq!(int64_t::MIN, INT64_MIN as _);
        assert_eq!(int64_t::MAX, INT64_MAX as _);
        assert_eq!(int_fast64_t::MIN, INT_FAST64_MIN as _);
        assert_eq!(int_fast64_t::MAX, INT_FAST64_MAX as _);
        assert_eq!(int_least64_t::MIN, INT_LEAST64_MIN as _);
        assert_eq!(int_least64_t::MAX, INT_LEAST64_MAX as _);

        assert_eq!(uint64_t::MAX, UINT64_MAX as _);
        assert_eq!(uint_fast64_t::MAX, UINT_FAST64_MAX as _);
        assert_eq!(uint_least64_t::MAX, UINT_LEAST64_MAX as _);
    }

    #[test]
    fn intmax() {
        assert_eq!(size_of::<intmax_t>(), 8);
        assert_eq!(size_of::<uintmax_t>(), 8);

        assert_eq!(intmax_t::MIN, INTMAX_MIN as _);
        assert_eq!(intmax_t::MAX, INTMAX_MAX as _);
    }
}
