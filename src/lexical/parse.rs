//! Parse byte iterators to float.

use super::algorithm::*;
use super::digit::*;
use super::exponent::*;
use super::num::*;

// PARSERS
// -------

/// Parse the significant digits of the float.
///
/// * `integer`     - Slice containing the integer digits.
/// * `fraction`    - Slice containing the fraction digits.
fn parse_mantissa(integer: &[u8], fraction: &[u8]) -> (u64, usize) {
    let mut value: u64 = 0;
    // On overflow, validate that all the remaining characters are valid
    // digits, if not, return the first invalid digit. Otherwise,
    // calculate the number of truncated digits.
    let mut integer = integer.iter();
    while let Some(c) = integer.next() {
        value = match add_digit(value, to_digit(*c).unwrap()) {
            Some(v) => v,
            None => return (value, 1 + integer.count() + fraction.len()),
        };
    }
    let mut fraction = fraction.iter();
    while let Some(c) = fraction.next() {
        value = match add_digit(value, to_digit(*c).unwrap()) {
            Some(v) => v,
            None => return (value, 1 + fraction.count()),
        };
    }
    (value, 0)
}

/// Parse float from extracted float components.
///
/// * `integer`     - Slice containing the integer digits.
/// * `fraction`    - Slice containing the fraction digits.
/// * `exponent`    - Parsed, 32-bit exponent.
///
/// # Preconditions
/// 1). The integer should not have leading zeros.
/// 2). The fraction should not have trailing zeros.
pub fn parse_float<F>(integer: &[u8], fraction: &[u8], exponent: i32) -> F
where
    F: Float,
{
    // Parse the mantissa and attempt the fast and moderate-path algorithms.
    let (mantissa, truncated) = parse_mantissa(integer, fraction);
    let is_truncated = truncated != 0;

    // Process the state to a float.
    if mantissa == 0 {
        // Literal 0, return early.
        // Value cannot be truncated, since truncation only occurs on
        // overflow or underflow.
        F::ZERO
    } else if !is_truncated {
        // Try the fast path, no mantissa truncation.
        let mant_exp = mantissa_exponent(exponent, fraction.len(), 0);
        if let Some(float) = fast_path(mantissa, mant_exp) {
            float
        } else {
            fallback_path(
                integer,
                fraction,
                mantissa,
                exponent,
                mant_exp,
                is_truncated,
            )
        }
    } else {
        let mant_exp = mantissa_exponent(exponent, fraction.len(), truncated);
        fallback_path(
            integer,
            fraction,
            mantissa,
            exponent,
            mant_exp,
            is_truncated,
        )
    }
}
