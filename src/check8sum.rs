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

//! # Check8Sum
//!
//! An 8-bit "wrapping" arithmetic sum checksum type with tests, implements Check8 trait

use crate::Check8;          // for the Check8 trait

// NOTE: we deliberately do not document the private fields

pub struct Check8Sum
{
    accum: u8,
}

/// # Provided Methods
///
/// - new: Creates a new instance of the type.
/// - get_accum: Retrieves the current value of the accumulator.
/// - init: Initializes the accumulator with a given value and returns the initialized value.
/// - add: Adds a given value to the accumulator using the appropriate algorithm and returns the updated value.
///
/// # Examples
///
/// ```rust
/// use crate::check8::{Check8, Check8Sum};
/// fn main() {
///     let mut sum = Check8Sum::new();
///     sum.init(255);
///     let result = sum.add(1);
///     assert_eq!(result, 0);
///     assert_eq!(sum.get_accum(), 0);
/// }
/// ```
///

impl Check8 for Check8Sum {

    fn new() -> impl Check8 {
        Check8Sum { accum: 0 }
    }

    fn get_accum(&self) -> u8 {
        self.accum
    }

    fn init(&mut self, val: u8) -> u8 {
        self.accum = val;
        self.accum
    }

    fn add(&mut self, val: u8) -> u8 {
        self.accum = self.accum.wrapping_add(val);
        self.accum
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_returns_zero() {
        let sum = Check8Sum::new();
        assert_eq!(sum.get_accum(), 0)
    }

    #[test]
    fn init_with_zero_returns_zero() {
        let mut sum = Check8Sum::new();
        let result = sum.init(0);
        assert_eq!(result, 0)
    }

    #[test]
    fn init_with_value_returns_value() {
        let mut sum = Check8Sum::new();
        let result = sum.init(255);
        assert_eq!(result, 255)
    }

    #[test]
    fn add_rolls_over() {
        let mut sum = Check8Sum::new();
        sum.init(255);
        let result = sum.add(1);
        assert_eq!(result, 0)
    }

    #[test]
    fn calculate_from_byte_array_returns_correct_sum() {
        let test_array : [u8; 3] = [0x01, 0x02, 0x03];

        // calculate the 8-bit arithmetic sum of the test array
        let mut expected :u32 = 0;
        for val in test_array.iter() {
            expected += *val as u32;
        }
        let expected = expected % 256;

        let mut sum = Check8Sum::new();
        let result = sum.calculate_from_byte_array(&test_array);
        assert_eq!(result, expected as u8)
    }

    #[test]
    fn calculate_from_string_returns_correct_sum() {
        let test_string = "hello";

        // calculate the 8-bit arithmetic sum of the test string
        let mut expected :u32 = 0;
        for val in test_string.as_bytes() {
            expected += *val as u32;
        }
        let expected = expected % 256;

        let mut sum = Check8Sum::new();
        let result = sum.calculate_from_string("hello");
        assert_eq!(result, expected as u8)
    }
}