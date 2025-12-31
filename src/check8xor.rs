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

//
// An 8-bit XOR sum checksum type with tests, implements Check8 trait
//

use crate::Check8;          // for the Check8 trait

pub struct Check8Xor
{
    accum: u8,
}

impl Check8 for Check8Xor
{
    fn new() -> impl Check8 {
        Check8Xor { accum: 0 }
    }

    fn get_accum(&self) -> u8 {
        self.accum
    }

    fn init(&mut self, val: u8) -> u8 {
        self.accum = val;
        self.accum
    }

    fn add(&mut self, val: u8) -> u8 {
        self.accum ^= val;
        self.accum
    }

    fn calculate_from_byte_array(&mut self, array: &[u8]) -> u8 {
        for val in array {
            self.add(*val);
        }
        self.accum
    }

    fn calculate_from_string(&mut self, string: &str) -> u8 {
        self.calculate_from_byte_array(string.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::Check8;
    use crate::check8xor::Check8Xor;

    #[test]
    fn init_with_zero_returns_zero() {
        let mut sum = Check8Xor::new();
        sum.init(255);
        let result = sum.init(0);
        assert_eq!(result, 0)
    }

    #[test]
    fn calculate_from_byte_array_returns_correct_sum() {
        let test_array : [u8; 3] = [0x01, 0x02, 0x03];

        // calculate the 8-bit xor of the test array
        let mut expected :u8 = 0;
        for val in test_array.iter() {
            expected ^= *val;
        }

        let mut sum = Check8Xor::new();
        let result = sum.calculate_from_byte_array(&test_array);
        assert_eq!(result, expected)
    }

    #[test]
    fn calculate_from_string_returns_correct_sum() {
        let test_string = "hello";

        // calculate the 8-bit xor of the test string
        let mut expected :u8 = 0;
        for val in test_string.as_bytes() {
            expected ^= *val;
        }

        let mut sum = Check8Xor::new();
        let result = sum.calculate_from_string("hello");
        assert_eq!(result, expected)
    }
}