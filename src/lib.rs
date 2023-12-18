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
* Last updated: 2023-12-18
*/

use log::warn;
use std::fmt;

///
#[derive(Debug, Clone)]
pub struct WidthError;

///
impl fmt::Display for WidthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid target pad width for the provided string length"
        )
    }
}

///
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

pub trait Formattable {}
impl Formattable for char {}
impl Formattable for u8 {}

pub trait PadResult {}
impl PadResult for String {}
impl<T: Formattable> PadResult for Vec<T> {}

#[derive(Debug, Clone, Copy)]
pub struct Symbol<T: Formattable> {
    item: T,
}

impl<T> Symbol<T> 
where T: Formattable
{
    pub fn new(t: T) -> Self {
        Self { item: t }
    }

    pub fn item(self) -> T {
        self.item
    }
}

impl<T> From<Symbol<T>> for char
where T: Formattable, char: From<T>
{
    fn from(s: Symbol<T>) -> Self {
        s.item.into()
    }
}

impl<T> From<Symbol<T>> for u8
where T: Formattable, u8: From<T>
{
    fn from(s: Symbol<T>) -> Self {
        s.item.into()
    }
}

///
pub trait Padder<T: Formattable> {
    type Output;

    fn pad(&self, width: usize, symbol: Symbol<T>, mode: &Alignment) -> Self::Output;
    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        symbol: Symbol<T>,
        mode: &Alignment,
        buffer: &mut Self::Output,
    );
}

///
impl<T: Formattable> Padder<T> for str
where char: From<T>
{
    type Output = String;

    ///
    fn pad(&self, width: usize, symbol: Symbol<T>, mode: &Alignment) -> Self::Output {
        if width < self.len() {
            warn!("{}, will slice the data to fit to the target width...", WidthError);
            return slice_to_fit(self, width, mode).to_string();
        }

        let mut output = String::with_capacity(width);
        let diff: usize = width - self.len();

        if diff == 0 {
            return self.to_string();
        }

        let (lpad, rpad) = match mode {
            Alignment::Left => (0, diff),
            Alignment::Right => (diff, 0),
            Alignment::Center => (diff / 2, diff - diff / 2),
        };

        let pad_char: char = symbol.item().into();

        (0..lpad).for_each(|_| output.push(pad_char));
        output.push_str(self);
        (0..rpad).for_each(|_| output.push(pad_char));

        output
    }

    ///
    fn pad_and_push_to_buffer(
            &self,
            width: usize,
            symbol: Symbol<T>,
            mode: &Alignment,
            buffer: &mut Self::Output,
        ) {
       let padded: String = self.pad(width, symbol, mode);
       buffer.push_str(&padded);
    }
}

pub fn xdwhitespace<T, P>(buffer: T, width: usize, mode: &Alignment) -> P 
where T: Padder<T + <Output = P>> + Formattable, P: PadResult
{
    buffer.pad(width, Symbol::new(' '), mode)
}
pub fn xdzero<T, P>(buffer: T, width: usize, mode: &Alignment)
where T: Padder<T> + Formattable, P: PadResult
{
    buffer.pad(width, Symbol::new('0'), mode)
}

/*
///
impl<T: Formattable> Padder for Vec<T> {
    type Output = Vec<T>;

    /// Allocates memory for a new Vec<T> with capacity width, consumes `self`.
    fn pad(&self, width: usize, pad_symbol: T, mode: &Alignment) -> Self::Output {
        if width < self.len() {
            warn!("{}, will slice the data to fit to the target width...", WidthError);
            return slice_to_fit(self, width, mode)
        }

        let mut output: Vec<T> = Vec::with_capacity(width);
        let diff: usize = width - self.len();

        if diff == 0 {
            return self;
        }

        let (lpad, rpad) = match mode {
            Alignment::Left => (0, diff),
            Alignment::Right => (diff, 0),
            Alignment::Center => (diff / 2, diff - diff / 2),
        };

        (0..lpad).for_each(|_| output.push(pad_symbol));
        output.extend_from_slice(self.as_slice());
        (0..rpad).for_each(|_| output.push(pad_symbol));

        output

    }

    ///
    fn pad_and_push_to_buffer(
            &self,
            width: usize,
            pad_symbol: &Symbol,
            mode: &Alignment,
            buffer: &mut Self::Output,
        ) {
        todo!()
    }
}
*/

///
pub fn whitespace(string: &str, width: usize, mode: Alignment) -> Result<String, WidthError> {
    pad(string, width, &mode, ' ')
}

///
pub fn zeros(string: &str, width: usize, mode: Alignment) -> Result<String, WidthError> {
    pad(string, width, &mode, '0')
}

///
pub fn pad_into_bytes(
    string: &str,
    width: usize,
    mode: Alignment,
    pad_char: char,
) -> Result<Vec<u8>, WidthError> {
    Ok(pad(string, width, &mode, pad_char)?.into_bytes())
}

///
pub fn pad_and_push_to_buffer(
    string: &str,
    width: usize,
    mode: Alignment,
    pad_char: char,
    buffer: &mut Vec<u8>,
) {
    let padded: String = match pad(string, width, &mode, pad_char) {
        Ok(s) => s,
        Err(e) => {
            warn!("could not pad `{}` due to: {}", string, e);
            warn!("will slice the string to fit in the buffer, expect some data to be missing...");
            slice_to_fit(string, width, &mode).to_string()
        }
    };

    buffer.extend_from_slice(padded.as_bytes());
}

///
/// # Panic
/// Iff the [`pad`] function returns a [`WidthError`], at which point the string can't be padded.
pub fn try_pad_and_push_to_buffer(
    string: &str,
    width: usize,
    mode: Alignment,
    pad_char: char,
    buffer: &mut Vec<u8>,
) {
    buffer.extend_from_slice(pad(string, width, &mode, pad_char).unwrap().as_bytes());
}

///
/// # Error
/// Iff the target padding width is less than the length of the string to pad.
fn pad(string: &str, width: usize, mode: &Alignment, pad_char: char) -> Result<String, WidthError> {
    if width < string.len() {
        return Err(WidthError);
    }

    let mut output = String::with_capacity(width);
    let diff: usize = width - string.len();

    if diff == 0 {
        return Ok(string.to_string());
    }

    let (lpad, rpad) = match mode {
        Alignment::Left => (0, diff),
        Alignment::Right => (diff, 0),
        Alignment::Center => (diff / 2, diff - diff / 2),
    };

    (0..lpad).for_each(|_| output.push(pad_char));
    output.push_str(string);
    (0..rpad).for_each(|_| output.push(pad_char));

    Ok(output)
}

///
fn slice_to_fit<'a>(string: &'a str, width: usize, mode: &'a Alignment) -> &'a str {
    match mode {
        Alignment::Left => &string[0..width],
        Alignment::Right => &string[(string.len() - width)..],
        Alignment::Center => {
            &string[(string.len() / 2 - width / 2)..(string.len() / 2 + width / 2)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn pad_unwrap_on_error() {
        let mut buffer: Vec<u8> = Vec::with_capacity(193838);
        try_pad_and_push_to_buffer(
            "the length of this string is much larger than the target padding width",
            10,
            Alignment::Right,
            ' ',
            &mut buffer,
        );
    }

    #[test]
    fn pad_and_slice_to_fit_left_align() {
        let mut buffer: Vec<u8> = Vec::with_capacity(193838);
        pad_and_push_to_buffer(
            "the length of this string is much larger than the target padding width",
            10,
            Alignment::Left,
            ' ',
            &mut buffer,
        );

        assert_eq!("the length".as_bytes(), buffer);
    }

    #[test]
    fn pad_and_slice_to_fit_right_align() {
        let mut buffer: Vec<u8> = Vec::with_capacity(193838);
        pad_and_push_to_buffer(
            "the length of this string is much larger than the target padding width",
            10,
            Alignment::Right,
            ' ',
            &mut buffer,
        );

        assert_eq!("ding width".as_bytes(), buffer);
    }

    #[test]
    fn pad_and_slice_to_fit_center_align_even() {
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        pad_and_push_to_buffer("hejjag", 2, Alignment::Center, ' ', &mut buffer);

        assert_eq!("jj".as_bytes(), buffer);
    }

    #[test]
    fn pad_and_slice_to_fit_center_align_uneven() {
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        pad_and_push_to_buffer(
            "a little bit trickier center align!",
            10,
            Alignment::Center,
            ' ',
            &mut buffer,
        );

        assert_eq!(" trickier ".as_bytes(), buffer);
    }

    #[test]
    #[should_panic]
    fn pad_should_panic() {
        let _ = whitespace("lol hahahah xd test", 15, Alignment::Center).unwrap();
    }

    #[test]
    fn pad_whitespace_right_align_into_bytes() {
        let width: usize = 30;
        let bytes = pad_into_bytes("this is cool", width, Alignment::Right, ' ');

        assert_eq!(
            format!("{:>width$}", "this is cool").into_bytes(),
            bytes.unwrap()
        );
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

        assert_eq!(format!("{:<width$}", "this is cool"), pad.unwrap());
    }

    #[test]
    fn pad_whitespace_right_align() {
        let width: usize = 30;
        let pad = whitespace("this is cool", width, Alignment::Right);

        assert_eq!(format!("{:>width$}", "this is cool"), pad.unwrap());
    }

    #[test]
    fn pad_whitespace_center_align() {
        let width: usize = 30;
        let pad = whitespace("this is cool", width, Alignment::Center);

        assert_eq!(format!("{:^width$}", "this is cool"), pad.unwrap());
    }

    #[test]
    fn pad_zeros_left_align() {
        let width: usize = 30;
        let pad = zeros("this is cool", width, Alignment::Left);

        assert_eq!(format!("{:0<width$}", "this is cool"), pad.unwrap());
    }

    #[test]
    fn pad_zeros_right_align() {
        let width: usize = 30;
        let pad = zeros("this is cool", width, Alignment::Right);

        assert_eq!(format!("{:0>width$}", "this is cool"), pad.unwrap());
    }

    #[test]
    fn pad_zeros_center_align() {
        let width: usize = 30;
        let pad = zeros("this is cool", width, Alignment::Center);

        assert_eq!(format!("{:0^width$}", "this is cool"), pad.unwrap());
    }

    #[test]
    fn pad_1000000_zeros_center() {
        let width: usize = 100_000_000;
        let pad = zeros("this is cool", width, Alignment::Center);

        assert_eq!(format!("{:0^width$}", "this is cool"), pad.unwrap());
    }
}
