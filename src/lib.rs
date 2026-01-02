/*

MIT License

Copyright (c) 2025 Tony Hedge, Benthic Sciences LLP

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

//! # Check8
//!
//! A library of 8-bit checksum types implementing the Check8 trait

mod check8sum;  // implements Check8Sum - a simple arithmetic sum checksum type
mod check8xor;  // implements Check8Xor - a simple XOR checksum type
mod check8crc;  // implements Check8Crc - an 8-bit CRC checksum type

// re-export to make the provided implementation types available to the user
pub use crate::check8sum::Check8Sum;
pub use crate::check8xor::Check8Xor;
pub use crate::check8crc::Check8Crc;

///
/// # Required Methods
///
/// - new: Creates a new instance of the type.
/// - get_accum: Retrieves the current value of the accumulator.
/// - init: Initialises the accumulator with a given value and returns the initialised value.
/// - add: Adds a given value to the accumulator using the appropriate algorithm and returns the updated value.
///
/// # Provided Methods
///
/// - calculate_from_byte_array:
///     Processes a slice of bytes by adding each byte's value to the accumulator using the add method.
///     Finally, it retrieves the accumulated value using get_accum.
///     - **Parameter**: array - A byte slice to process.
///     - **Returns**: The final accumulated value as an u8.
///
/// - calculate_from_string:
///     Converts a string to its byte representation and processes it using calculate_from_byte_array.
///     - **Parameter**: string - A string whose byte representation is processed.
///     - **Returns**: The final accumulated value as an u8.
///
/// # Examples
///
/// Demonstrates use of the Check8 trait as a parameter to a function.
/// ```
///use crate::check8::{Check8, Check8Sum, Check8Xor};
///
/// fn calculate_from_string_with_type_as_parameter(string: &str, checksum_type: &mut impl Check8) -> u8 {
///     checksum_type.calculate_from_string(string)
/// }
///
/// fn main()  {
///     let test_string = "hello";
///
///     let mut sum_add = Check8Sum::new(0);
///     let result_add = calculate_from_string_with_type_as_parameter(test_string, &mut sum_add);
///     println!("{}, 8-bit Arithmetic Checksum: {:#04x}", test_string, result_add);
///
///     let mut sum_xor = Check8Xor::new(0);
///     let result_xor = calculate_from_string_with_type_as_parameter(test_string, &mut sum_xor);
///     println!("{}, 8-bit XOR Checksum: {:#04x}", test_string, result_xor);
///     assert!(result_add != result_xor);
/// }
///```
///

pub trait Check8 {
    fn new(initial: u8) -> impl Check8;
    fn get_accum(&self) -> u8;
    fn init(&mut self, val: u8) -> u8;
    fn add(&mut self, val: u8) -> u8;

    fn calculate_from_byte_array(&mut self, array: &[u8]) -> u8 {
        for val in array {
            self.add(*val);
        }
        self.get_accum()
    }

    fn calculate_from_string(&mut self, string: &str) -> u8 {
        self.calculate_from_byte_array(string.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ```rust
    /// Calculates a checksum value from a given string using the provided checksum type.
    ///
    /// # Parameters
    /// - `string`: A reference to the input string for which the checksum will be calculated.
    /// - `checksum_type`: A mutable reference to an instance of a type that implements the `Check8` trait.
    ///   This type is used to perform the actual checksum computation.
    ///
    /// # Returns
    /// - A `u8` value representing the computed checksum.
    ///
    /// # Notes
    /// - The function delegates the actual checksum computation to the `calculate_from_string` method
    ///   of the provided `checksum_type` instance.
    /// - The `checksum_type` must implement the `Check8` trait, which defines the required behavior
    ///   for checksum calculation.
    ///
    /// ```
    fn calculate_from_string_with_type_as_parameter(string: &str, checksum_type: &mut impl Check8) -> u8 {
        checksum_type.calculate_from_string(string)
    }
    #[test]
    fn test_calculate_from_string_with_type_as_parameter() {
        let test_string = "hello";

        // calculate the 8-bit arithmetic sum and 8-bit xor of the test string
        let mut expected_sum:u32 = 0;
        let mut expected_xor:u8 = 0;
        for val in test_string.as_bytes() {
            expected_sum += *val as u32;
            expected_xor ^= *val;
        }
        expected_sum %= 256;

        let mut sum_add = Check8Sum::new(0);
        let result_add = calculate_from_string_with_type_as_parameter(test_string, &mut sum_add);
        assert_eq!(result_add, expected_sum as u8);

        let mut sum_xor = Check8Xor::new(0);
        let result_xor = calculate_from_string_with_type_as_parameter(test_string, &mut sum_xor);
        assert_eq!(result_xor, expected_xor);
    }
    
}
