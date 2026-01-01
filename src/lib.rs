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

// re-export to make the provided implementation types available to the user
pub use crate::check8sum::Check8Sum;
pub use crate::check8xor::Check8Xor;

///
/// # Required Methods
///
/// - new: Creates a new instance of the type.
/// - get_accum: Retrieves the current value of the accumulator.
/// - init: Initializes the accumulator with a given value and returns the initialized value.
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

pub trait Check8 {
    fn new() -> impl Check8;
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

