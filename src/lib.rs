/*
* MIT License
*
* Copyright (c) 2023 Firelink Data
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*
* File created: 2023-12-14
* Last updated: 2023-12-14
*/

use std::fmt;

///
#[derive(Debug, Clone)]
pub struct WidthError;

///
impl fmt::Display for WidthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid target pad width for the provided string length."
        )
    }
}

///
pub enum Alignment {
    Left,
    Right,
    Center,
}

///
pub fn whitespace(string: &str, width: usize, mode: Alignment) -> String {
    pad(string, width, mode, ' ')
}

///
pub fn zeros(string: &str, width: usize, mode: Alignment) -> String {
    pad(string, width, mode, '0')
}

///
pub fn pad_into_bytes(string: &str, width: usize, mode: Alignment, pad_char: char) -> Vec<u8> {
    pad(string, width, mode, pad_char).into_bytes()
}

///
pub fn pad_and_push_to_buffer(
    string: &str,
    width: usize,
    mode: Alignment,
    pad_char: char,
    buffer: &mut Vec<u8>,
) {
    buffer.extend_from_slice(pad(string, width, mode, pad_char).as_bytes());
}

///
/// Panics
/// Iff the target pad width is less than the provided string length.
fn pad(string: &str, width: usize, mode: Alignment, pad_char: char) -> String {
    if width < string.len() {
        panic!("Invalid target pad width for the provide string length.")
    }

    let mut output = String::with_capacity(width);
    let diff: usize = width - string.len();

    if diff == 0 {
        return string.to_string();
    }

    let (lpad, rpad) = match mode {
        Alignment::Left => (0, diff),
        Alignment::Right => (diff, 0),
        Alignment::Center => (diff / 2, diff - diff / 2),
    };

    (0..lpad).for_each(|_| output.push(pad_char));
    output.push_str(string);
    (0..rpad).for_each(|_| output.push(pad_char));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn pad_should_panic() {
        let _ = whitespace("lol hahahah xd test", 15, Alignment::Center);
    }

    #[test]
    fn pad_whitespace_right_align_into_bytes() {
        let width: usize = 30;
        let bytes = pad_into_bytes("this is cool", width, Alignment::Right, ' ');

        assert_eq!(format!("{:>width$}", "this is cool").into_bytes(), bytes,);
    }

    #[test]
    fn pad_zeros_left_align_push_to_buffer() {
        let width: usize = 128;
        let expected: Vec<u8> =
            format!("{:0<width$}", "testing buffer reuse smart memory").into_bytes();

        let mut buffer: Vec<u8> = Vec::with_capacity(1024 * 1024);
        pad_and_push_to_buffer(
            "testing buffer reuse smart memory",
            width,
            Alignment::Left,
            '0',
            &mut buffer,
        );

        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_whitespace_left_align() {
        let width: usize = 30;
        let pad = whitespace("this is cool", width, Alignment::Left);

        assert_eq!(format!("{:<width$}", "this is cool"), pad);
    }

    #[test]
    fn pad_whitespace_right_align() {
        let width: usize = 30;
        let pad = whitespace("this is cool", width, Alignment::Right);

        assert_eq!(format!("{:>width$}", "this is cool"), pad);
    }

    #[test]
    fn pad_whitespace_center_align() {
        let width: usize = 30;
        let pad = whitespace("this is cool", width, Alignment::Center);

        assert_eq!(format!("{:^width$}", "this is cool"), pad);
    }

    #[test]
    fn pad_zeros_left_align() {
        let width: usize = 30;
        let pad = zeros("this is cool", width, Alignment::Left);

        assert_eq!(format!("{:0<width$}", "this is cool"), pad);
    }

    #[test]
    fn pad_zeros_right_align() {
        let width: usize = 30;
        let pad = zeros("this is cool", width, Alignment::Right);

        assert_eq!(format!("{:0>width$}", "this is cool"), pad);
    }

    #[test]
    fn pad_zeros_center_align() {
        let width: usize = 30;
        let pad = zeros("this is cool", width, Alignment::Center);

        assert_eq!(format!("{:0^width$}", "this is cool"), pad);
    }

    #[test]
    fn pad_1000000_zeros_center() {
        let width: usize = 100_000_000;
        let pad = zeros("this is cool", width, Alignment::Center);

        assert_eq!(format!("{:0^width$}", "this is cool"), pad);
    }
}
