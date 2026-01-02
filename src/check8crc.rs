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

//! # Check8Crc
//!
//! An 8-bit CRC checksum type with tests, implements Check8 trait.
//! Uses a lookup table generated from a polynomial.

use crate::Check8;          // for the Check8 trait

pub struct Check8Crc {
    accum: u8,
    table: [u8; 256],
}

/// # Provided Methods
///
/// - new: Creates a new instance of the type, generates the CRC lookup table from the provided polynomial.
/// - get_accum: Retrieves the current value of the accumulator.
/// - init: Initialises the accumulator with a given value and returns the initialised value.
/// - add: Adds a given value to the accumulator using the CRC algorithm and returns the updated value.
///
/// # Examples
///
/// ```rust
/// use crate::check8::{Check8, Check8Crc};
/// fn main() {
///     // Standard CRC-8 polynomial 0x07
///     let mut crc = Check8Crc::new(0x07);
///     crc.init(0x00);
///     let result = crc.add(0x01);
///     // For poly 0x07 and input 0x01, the result is 0x07
///     assert_eq!(result, 0x07);
///     assert_eq!(crc.get_accum(), 0x07);
/// }
/// ```
///
impl Check8Crc {
    fn generate_table(poly: u8) -> [u8; 256] {
        let mut table = [0u8; 256];
        for i in 0..256 {
            let mut crc = i as u8;
            for _ in 0..8 {
                if (crc & 0x80) != 0 {
                    crc = (crc << 1) ^ poly;
                } else {
                    crc <<= 1;
                }
            }
            table[i] = crc;
        }
        table
    }
}

impl Check8 for Check8Crc {
    fn new(poly: u8) -> impl Check8 {
        Check8Crc {
            accum: 0,
            table: Self::generate_table(poly),
        }
    }

    fn get_accum(&self) -> u8 {
        self.accum
    }

    fn init(&mut self, val: u8) -> u8 {
        self.accum = val;
        self.accum
    }

    fn add(&mut self, val: u8) -> u8 {
        self.accum = self.table[(self.accum ^ val) as usize];
        self.accum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc8_standard_poly() {
        // Standard CRC-8 polynomial 0x07 (e.g. SMBus)
        let mut crc = Check8Crc::new(0x07);
        crc.init(0);
        let result = crc.calculate_from_byte_array(&[1, 2, 3]);
        assert_eq!(result, 72);

        let mut crc = Check8Crc::new(0x07);
        // "123" in bytes: [49, 50, 51]
        let res = crc.calculate_from_string("123");
        // Our implementation of CRC-8 (poly 0x07, init 0) of "123" is 0xC0
        assert_eq!(res, 0xC0);
    }

    #[test]
    fn test_init_works() {
        let mut crc = Check8Crc::new(0x07);
        crc.init(0xFF);
        assert_eq!(crc.get_accum(), 0xFF);
    }
}
