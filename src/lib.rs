//! Rust bindings to `libsais`.
//!
//! We provide 6 versions of the `suffix_array_*` function.
//! The suffix indicates the alphabet of the input text:
//! - `u8`
//! - `u16`
//! - `i32`
//! When the text is more than `2^31` characters long, we provide the `long_`
//! variants, that return an array of `i64` instead of `i32`.
//!
//! Currently not supported, but PRs welcome:
//! - Versions taking a `ctx` context parameter to reuse allocations.
//! - Burrows-Wheeler Transform (BWT) and longest common prefix (LCP) related functions.
//!
//! TODO: Bindings to OpenMP versions.

/// Direct bindings to the C API.
pub mod sys {
    pub use libsais_sys::*;
}

pub fn suffix_array_u8(text: &[u8], sa: &mut [i32]) -> Result<(), ()> {
    if sa.len() < text.len() {
        return Err(());
    }
    let res = unsafe {
        sys::libsais(
            text.as_ptr(),
            sa.as_mut_ptr(),
            text.len() as i32,
            sa.len() as i32 - text.len() as i32,
            std::ptr::null_mut(),
        )
    };
    if res == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn suffix_array_u16(text: &[u16], sa: &mut [i32]) -> Result<(), ()> {
    if sa.len() < text.len() {
        return Err(());
    }
    let res = unsafe {
        sys::libsais16(
            text.as_ptr(),
            sa.as_mut_ptr(),
            text.len() as i32,
            sa.len() as i32 - text.len() as i32,
            std::ptr::null_mut(),
        )
    };
    if res == 0 {
        Ok(())
    } else {
        Err(())
    }
}

/// Text is modified during construction, but restored afterwards.
/// `alphabet_size` is the alphabet size.
/// TODO: Is this #distinct characters or upper bound on character value?
pub fn suffix_array_i32(text: &mut [i32], sa: &mut [i32], alphabet_size: i32) -> Result<(), ()> {
    if sa.len() < text.len() {
        return Err(());
    }
    let res = unsafe {
        sys::libsais_int(
            text.as_mut_ptr(),
            sa.as_mut_ptr(),
            text.len() as i32,
            alphabet_size,
            sa.len() as i32 - text.len() as i32,
        )
    };
    if res == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn long_suffix_array_u8(text: &[u8], sa: &mut [i64]) -> Result<(), ()> {
    if sa.len() < text.len() {
        return Err(());
    }
    let res = unsafe {
        sys::libsais64(
            text.as_ptr(),
            sa.as_mut_ptr(),
            text.len() as i64,
            sa.len() as i64 - text.len() as i64,
            std::ptr::null_mut(),
        )
    };
    if res == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn long_suffix_array_u16(text: &[u16], sa: &mut [i64]) -> Result<(), ()> {
    if sa.len() < text.len() {
        return Err(());
    }
    let res = unsafe {
        sys::libsais16x64(
            text.as_ptr(),
            sa.as_mut_ptr(),
            text.len() as i64,
            sa.len() as i64 - text.len() as i64,
            std::ptr::null_mut(),
        )
    };
    if res == 0 {
        Ok(())
    } else {
        Err(())
    }
}

/// Text is modified during construction, but restored afterwards.
/// `alphabet_size` is the alphabet size.
/// TODO: Is this #distinct characters or upper bound on character value?
pub fn long_suffix_array_i64(
    text: &mut [i64],
    sa: &mut [i64],
    alphabet_size: i64,
) -> Result<(), ()> {
    if sa.len() < text.len() {
        return Err(());
    }
    let res = unsafe {
        sys::libsais64_long(
            text.as_mut_ptr(),
            sa.as_mut_ptr(),
            text.len() as i64,
            alphabet_size,
            sa.len() as i64 - text.len() as i64,
        )
    };
    if res == 0 {
        Ok(())
    } else {
        Err(())
    }
}
