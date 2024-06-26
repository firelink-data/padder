//
// MIT License
//
// Copyright (c) 2024 Firelink Data
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// File created: 2023-12-14
// Last updated: 2024-05-10
//

//!
//! Highly efficient data and string formatting library for Rust.
//!
//! Pad and format virtually any generic slice or vector efficiently with minimal memory
//! overhead. This crate has guaranteed performance improvements over the standard
//! library [`format!`] macro. Clone this repository and run cargo bench to see benchmark
//! comparisons between this implementation and the standard library.
//!
//! The library defines a core trait called [`Source`] which enables efficient padding on
//! the type. It is currently implemented on three main types of datastructures:
//!  - the string slice `&str`,
//!  - the generic slice `&[T]`,
//!  - and the generic vector `Vec<T>,
//!
//! Note that the type T has to adhere to the trait bound `T: From<Symbol>`, where [`Symbol`]
//! is the Enum representing the available characters to pad and format with. If you want to
//! extend the padding capabilities of the [`Source`] trait with your own type T, then you
//! need to also implement the [`From<Symbol>`] trait for your type T.
//!
//! # Examples
//!
//! Given the below string slice and target pad width 6, with [`Alignment::Left`] and
//! [`Symbol::Whitespace`], the resulting output String can be seen on the right:
//!
//! +---+---+---+       +---+---+---+---+---+---+
//! | a | b | c |  -->  | a | b | c |   |   |   |
//! +---+---+---+       +---+---+---+---+---+---+
//!
//! ```
//! use padder::*;
//!
//! let output: String = "abc".pad(6, Alignment::Left, Symbol::Whitespace);
//! let expected = String::from("abc   ");
//! assert_eq!(expected, output);
//! ```
//!
//! You can also pad to an existing buffer, providing you precise control over any memory
//! allocations performed in your program. Given the below char slice and target pad
//! width 10, with [`Alignment::Right`] and [`Symbol::Hashtag`], the resulting contents
//! of the allocated buffer can be seen on the right:
//!
//! +---+---+---+       +---+---+---+---+---+---+---+---+---+---+
//! | a | b | c |  -->  | # | # | # | # | # | # | # | a | b | c |
//! +---+---+---+       +---+---+---+---+---+---+---+---+---+---+
//!
//! ```
//! use padder::*;
//!
//! let width: usize = 10;
//! let source: &[char] = &['a', 'b', 'c'];
//!
//! let mut buffer: Vec<char> = Vec::with_capacity(width);
//! pad_and_push_to_buffer(
//!     source,
//!     width,
//!     Alignment::Right,
//!     Symbol::Hashtag,
//!     &mut buffer,
//! );
//!
//! let mut expected = vec!['#'; 7];
//! expected.extend_from_slice(source);
//! assert_eq!(expected, buffer);
//! ```
//!
//! For more inspiration and guidance on how to use this crate and its trait, please refer
//! to the examples of the README in the project repository.
//!

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::clone;
use std::fmt;

/// Exhaustive enum for the alternative ways to pad and format data.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Alignment {
    Left,
    #[default]
    Right,
    Center,
}

///
impl Alignment {
    /// Get amount of chars to pad on either side of the [`Source`].
    pub fn left_right_padding(&self, diff: usize) -> (usize, usize) {
        match self {
            Self::Left => (0, diff),
            Self::Right => (diff, 0),
            Self::Center => (diff / 2, diff - diff / 2),
        }
    }
}

/// Exhaustive enum for the supported padding symbols.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Symbol {
    Hyphen,
    Underscore,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Hashtag,
    Backslash,
    Forwardslash,
    Asterisk,
    #[default]
    Whitespace,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

/// Convert the [`Symbol`] enum into its character representation.
/// Moves the ownership of the enum to the caller.
impl From<Symbol> for char {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Hyphen => '-',
            Symbol::Underscore => '_',
            Symbol::Dot => '.',
            Symbol::Comma => ',',
            Symbol::Colon => ':',
            Symbol::Semicolon => ';',
            Symbol::Hashtag => '#',
            Symbol::Backslash => '\\',
            Symbol::Forwardslash => '/',
            Symbol::Asterisk => '*',
            Symbol::Whitespace => ' ',
            Symbol::Zero => '0',
            Symbol::One => '1',
            Symbol::Two => '2',
            Symbol::Three => '3',
            Symbol::Four => '4',
            Symbol::Five => '5',
            Symbol::Six => '6',
            Symbol::Seven => '7',
            Symbol::Eight => '8',
            Symbol::Nine => '9',
        }
    }
}

/// Convert the [`Symbol`] enum into its slice-char representation.
/// Moves the ownership of the enum to the caller.
impl From<Symbol> for &[char] {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Hyphen => &['-'],
            Symbol::Underscore => &['_'],
            Symbol::Dot => &['.'],
            Symbol::Comma => &[','],
            Symbol::Colon => &[':'],
            Symbol::Semicolon => &[';'],
            Symbol::Hashtag => &['#'],
            Symbol::Backslash => &['\\'],
            Symbol::Forwardslash => &['/'],
            Symbol::Asterisk => &['*'],
            Symbol::Whitespace => &[' '],
            Symbol::Zero => &['0'],
            Symbol::One => &['1'],
            Symbol::Two => &['2'],
            Symbol::Three => &['3'],
            Symbol::Four => &['4'],
            Symbol::Five => &['5'],
            Symbol::Six => &['6'],
            Symbol::Seven => &['7'],
            Symbol::Eight => &['8'],
            Symbol::Nine => &['9'],
        }
    }
}

/// Convert the [`Symbol`] enum into its byte presentation.
/// Moves the ownership of the enum to the caller.
impl From<Symbol> for u8 {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Hyphen => b'-',
            Symbol::Underscore => b'_',
            Symbol::Dot => b'.',
            Symbol::Comma => b',',
            Symbol::Colon => b':',
            Symbol::Semicolon => b';',
            Symbol::Hashtag => b'#',
            Symbol::Backslash => b'\\',
            Symbol::Forwardslash => b'/',
            Symbol::Asterisk => b'*',
            Symbol::Whitespace => b' ',
            Symbol::Zero => b'0',
            Symbol::One => b'1',
            Symbol::Two => b'2',
            Symbol::Three => b'3',
            Symbol::Four => b'4',
            Symbol::Five => b'5',
            Symbol::Six => b'6',
            Symbol::Seven => b'7',
            Symbol::Eight => b'8',
            Symbol::Nine => b'9',
        }
    }
}

/// Convert the [`Symbol`] enum into its slice representation.
/// Moves the ownership of the enum to the caller.
impl From<Symbol> for &[u8] {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Hyphen => "-".as_bytes(),
            Symbol::Underscore => "_".as_bytes(),
            Symbol::Dot => ".".as_bytes(),
            Symbol::Comma => ",".as_bytes(),
            Symbol::Colon => ":".as_bytes(),
            Symbol::Semicolon => ";".as_bytes(),
            Symbol::Hashtag => "#".as_bytes(),
            Symbol::Backslash => "\\".as_bytes(),
            Symbol::Forwardslash => "/".as_bytes(),
            Symbol::Asterisk => "*".as_bytes(),
            Symbol::Whitespace => " ".as_bytes(),
            Symbol::Zero => "0".as_bytes(),
            Symbol::One => "1".as_bytes(),
            Symbol::Two => "2".as_bytes(),
            Symbol::Three => "3".as_bytes(),
            Symbol::Four => "4".as_bytes(),
            Symbol::Five => "5".as_bytes(),
            Symbol::Six => "6".as_bytes(),
            Symbol::Seven => "7".as_bytes(),
            Symbol::Eight => "8".as_bytes(),
            Symbol::Nine => "9".as_bytes(),
        }
    }
}

/// A trait providing functions to perform padding and formatting on the implemented type.
///
/// The main [`Source::pad`] API for this trait requires the caller to provide three knowns:
///     1. The target width as a usize.
///     2. The padding alignment mode, see [`Alignment`] for allowed modes.
///     3. The symbol to pad with, see [`Symbol`] for allowed symbols.
///
/// The trait is bound only for types T that implement the [`From<Symbol>`] trait. This is
/// to guarantee that the datatype that the caller wants to pad with can be converted from
/// the [`Symbol`] enum type to the corresponding type.
///
/// Utilizing this trait has guaranteed performance improvements over the [`format!`] macro
/// in the standard library, mainly due to only allocating memory on the heap once.
pub trait Source: fmt::Debug {
    type Buffer;
    type Output;

    /// Slice the source to fit the target width and return it as the defined output type.
    ///
    /// This function is called whenever a call to [`pad`] is attempted but the
    /// length of the source is less than the width. This truncates the source
    /// and may lead to data loss. This is logged to stdout whenever it occurs.
    fn slice_to_fit(&self, width: usize, mode: Alignment) -> Self::Output;

    /// Pad the source, the caller type, to fit the target width.
    fn pad(&self, width: usize, mode: Alignment, symbol: Symbol) -> Self::Output;

    /// Pad a buffer with the contents of source, to fit the target width.
    /// Truncates the contents of source if width < source.len().
    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Symbol,
        buffer: &mut Self::Buffer,
    );
}

/// Trait implementation for a string slice.
impl Source for &str
where
    char: From<Symbol>,
{
    type Buffer = String;
    type Output = String;

    fn slice_to_fit(&self, width: usize, mode: Alignment) -> Self::Output {
        match mode {
            Alignment::Left => self[0..width].to_string(),
            Alignment::Right => self[(self.len() - width)..].to_string(),
            Alignment::Center => self
                [(self.len() / 2 - width / 2)..(self.len() / 2 + width / 2 + width % 2)]
                .to_string(),
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Symbol) -> Self::Output {
        if width < self.len() {
            return self.slice_to_fit(width, mode);
        }

        let diff: usize = width - self.len();
        if diff == 0 {
            return self.to_string();
        }

        let (lpad, rpad) = mode.left_right_padding(diff);
        let pad_char: char = symbol.into();

        let mut output = String::with_capacity(width);
        (0..lpad).for_each(|_| output.push(pad_char));
        output.push_str(self);
        (0..rpad).for_each(|_| output.push(pad_char));

        output
    }

    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Symbol,
        buffer: &mut Self::Buffer,
    ) {
        let padded: Self::Output = self.pad(width, mode, symbol);
        buffer.push_str(padded.as_str());
    }
}

/// Trait implementation for a slice of types &[T].
impl<T> Source for &[T]
where
    T: From<Symbol> + clone::Clone + fmt::Debug,
    for<'a> &'a [T]: From<Symbol>,
{
    type Buffer = Vec<T>;
    type Output = Vec<T>;

    fn slice_to_fit(&self, width: usize, mode: Alignment) -> Self::Output {
        match mode {
            Alignment::Left => self[0..width].to_vec(),
            Alignment::Right => self[(self.len() - width)..].to_vec(),
            Alignment::Center => self
                [(self.len() / 2 - width / 2)..(self.len() / 2 + width / 2 + width % 2)]
                .to_vec(),
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Symbol) -> Self::Output {
        if width < self.len() {
            return self.slice_to_fit(width, mode);
        }

        let diff: usize = width - self.len();
        if diff == 0 {
            return self.to_vec();
        }

        let (lpad, rpad) = mode.left_right_padding(diff);
        let pad_type: &[T] = symbol.into();

        let mut output: Vec<T> = Vec::with_capacity(width);
        (0..lpad).for_each(|_| output.extend_from_slice(pad_type));
        output.extend_from_slice(self);
        (0..rpad).for_each(|_| output.extend_from_slice(pad_type));

        output
    }

    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Symbol,
        buffer: &mut Self::Buffer,
    ) {
        let padded: Self::Output = self.pad(width, mode, symbol);
        buffer.extend_from_slice(&padded);
    }
}

/// Trait implementation for a Vec<T> with support for both T and &[T] trait bounds.
impl<T> Source for Vec<T>
where
    T: From<Symbol> + clone::Clone + fmt::Debug,
    for<'a> &'a [T]: From<Symbol>,
{
    type Buffer = Vec<T>;
    type Output = Vec<T>;

    fn slice_to_fit(&self, width: usize, mode: Alignment) -> Self::Output {
        match mode {
            Alignment::Left => self[..width].to_vec(),
            Alignment::Right => self[(self.len() - width)..].to_vec(),
            Alignment::Center => self
                [(self.len() / 2 - width / 2)..(self.len() / 2 + width / 2 + width % 2)]
                .to_vec(),
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Symbol) -> Self::Output {
        if width < self.len() {
            return self.slice_to_fit(width, mode);
        }

        let diff: usize = width - self.len();
        if diff == 0 {
            return self.to_vec();
        }

        let (lpad, rpad) = mode.left_right_padding(diff);
        let pad_type: &[T] = symbol.into();

        let mut output: Vec<T> = Vec::with_capacity(width);
        (0..lpad).for_each(|_| output.extend_from_slice(pad_type));
        output.extend_from_slice(self);
        (0..rpad).for_each(|_| output.extend_from_slice(pad_type));

        output
    }

    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Symbol,
        buffer: &mut Self::Buffer,
    ) {
        let padded: Self::Output = self.pad(width, mode, symbol);
        buffer.extend_from_slice(&padded);
    }
}

/// Wrapper for the [`Source`] trait implementation of its [`pad`] function.
pub fn pad<S: Source>(source: S, width: usize, mode: Alignment, symbol: Symbol) -> S::Output {
    source.pad(width, mode, symbol)
}

/// Wrapper for the [`Source`] trait implementation of its [`pad_and_push_to_buffer`] function.
pub fn pad_and_push_to_buffer<S: Source>(
    source: S,
    width: usize,
    mode: Alignment,
    symbol: Symbol,
    buffer: &mut S::Buffer,
) {
    source.pad_and_push_to_buffer(width, mode, symbol, buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_underscore_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Underscore,
            &mut buffer,
        );
        let expected = Vec::from(&['a', 'b', 'c', '_']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_underscore_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Underscore,
            &mut buffer,
        );
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Underscore.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_underscore_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Underscore,
            &mut buffer,
        );
        let expected = String::from("abc_");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_underscore_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Underscore,
            &mut buffer,
        );
        let expected: Vec<u8> =
            Vec::from(&[Symbol::Underscore.into(), 1, 2, Symbol::Underscore.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_dot_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Dot, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '.']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_dot_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Dot, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Dot.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_dot_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Dot, &mut buffer);
        let expected = String::from("abc.");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_dot_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Dot, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Dot.into(), 1, 2, Symbol::Dot.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_comma_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Comma, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', ',']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_comma_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Comma, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Comma.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_comma_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Comma, &mut buffer);
        let expected = String::from("abc,");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_comma_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Comma, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Comma.into(), 1, 2, Symbol::Comma.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_colon_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Colon, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', ':']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_colon_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Colon, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Colon.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_colon_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Colon, &mut buffer);
        let expected = String::from(":abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_colon_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Colon, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Colon.into(), 1, 2, Symbol::Colon.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_semicolon_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Semicolon,
            &mut buffer,
        );
        let expected = Vec::from(&['a', 'b', 'c', ';']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_semicolon_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Semicolon,
            &mut buffer,
        );
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Semicolon.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_semicolon_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Right,
            Symbol::Semicolon,
            &mut buffer,
        );
        let expected = String::from(";abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_semicolon_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Semicolon,
            &mut buffer,
        );
        let expected: Vec<u8> =
            Vec::from(&[Symbol::Semicolon.into(), 1, 2, Symbol::Semicolon.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_hashtag_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Hashtag,
            &mut buffer,
        );
        let expected = Vec::from(&['a', 'b', 'c', '#']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_hashtag_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Hashtag,
            &mut buffer,
        );
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Hashtag.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_hashtag_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Right,
            Symbol::Hashtag,
            &mut buffer,
        );
        let expected = String::from("#abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_hashtag_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Hashtag,
            &mut buffer,
        );
        let expected: Vec<u8> = Vec::from(&[Symbol::Hashtag.into(), 1, 2, Symbol::Hashtag.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_backslash_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Backslash,
            &mut buffer,
        );
        let expected = Vec::from(&['a', 'b', 'c', '\\']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_backslash_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Backslash,
            &mut buffer,
        );
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Backslash.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_backslash_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Right,
            Symbol::Backslash,
            &mut buffer,
        );
        let expected = String::from("\\abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_backslash_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Backslash,
            &mut buffer,
        );
        let expected: Vec<u8> =
            Vec::from(&[Symbol::Backslash.into(), 1, 2, Symbol::Backslash.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_forwardslash_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Forwardslash,
            &mut buffer,
        );
        let expected = Vec::from(&['a', 'b', 'c', '/']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_forwardslash_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Forwardslash,
            &mut buffer,
        );
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Forwardslash.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_forwardslash_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Right,
            Symbol::Forwardslash,
            &mut buffer,
        );
        let expected = String::from("/abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_forwardslash_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Forwardslash,
            &mut buffer,
        );
        let expected: Vec<u8> = Vec::from(&[
            Symbol::Forwardslash.into(),
            1,
            2,
            Symbol::Forwardslash.into(),
        ]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_asterisk_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Asterisk,
            &mut buffer,
        );
        let expected = Vec::from(&['a', 'b', 'c', '*']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_asterisk_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Asterisk,
            &mut buffer,
        );
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Asterisk.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_asterisk_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Right,
            Symbol::Asterisk,
            &mut buffer,
        );
        let expected = String::from("*abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_asterisk_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Asterisk,
            &mut buffer,
        );
        let expected: Vec<u8> =
            Vec::from(&[Symbol::Asterisk.into(), 1, 2, Symbol::Asterisk.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_one_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::One, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '1']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_one_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::One, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::One.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_one_from_char() {
        let width: usize = 4;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::One, &mut buffer);
        let expected = String::from("1abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_one_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::One, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::One.into(), 1, 2, Symbol::One.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_two_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Two, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '2']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_two_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Two, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Two.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_two_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Two, &mut buffer);
        let expected = String::from("222abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_two_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Two, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Two.into(), 1, 2, Symbol::Two.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_three_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Three, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '3']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_three_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Three, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Three.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_three_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Three, &mut buffer);
        let expected = String::from("333abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_three_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Three, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Three.into(), 1, 2, Symbol::Three.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_four_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Four, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '4']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_four_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Four, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Four.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_four_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Four, &mut buffer);
        let expected = String::from("444abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_four_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Four, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Four.into(), 1, 2, Symbol::Four.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_five_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Five, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '5']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_five_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Five, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Five.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_five_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Five, &mut buffer);
        let expected = String::from("555abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_five_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Five, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Five.into(), 1, 2, Symbol::Five.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_six_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Six, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '6']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_six_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Six, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Six.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_six_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Six, &mut buffer);
        let expected = String::from("666abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_six_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Six, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Six.into(), 1, 2, Symbol::Six.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_seven_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Seven, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '7']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_seven_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Seven, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Seven.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_seven_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Seven, &mut buffer);
        let expected = String::from("777abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_seven_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Seven, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Seven.into(), 1, 2, Symbol::Seven.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_eight_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Eight, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '8']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_eight_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Eight, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Eight.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_eight_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Eight, &mut buffer);
        let expected = String::from("888abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_eight_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Eight, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Eight.into(), 1, 2, Symbol::Eight.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_nine_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Nine, &mut buffer);
        let expected = Vec::from(&['a', 'b', 'c', '9']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_nine_from_byte_slice() {
        let width: usize = 4;
        let source: &[u8] = &[1u8, 2, 3];
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Nine, &mut buffer);
        let expected = Vec::from(&[1u8, 2, 3, Symbol::Nine.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_nine_from_char() {
        let width: usize = 6;
        let source: &str = "abc";
        let mut buffer = String::new();
        pad_and_push_to_buffer(source, width, Alignment::Right, Symbol::Nine, &mut buffer);
        let expected = String::from("999abc");
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_nine_from_byte() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, Symbol::Nine, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[Symbol::Nine.into(), 1, 2, Symbol::Nine.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_hyphen_from_byte() {
        let width: usize = 3;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Left, Symbol::Hyphen, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[1, 2, Symbol::Hyphen.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_whitespace_from_byte() {
        let width: usize = 3;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Left,
            Symbol::Whitespace,
            &mut buffer,
        );
        let expected: Vec<u8> = Vec::from(&[1, 2, Symbol::Whitespace.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_zero_from_byte() {
        let width: usize = 3;
        let source = Vec::from(&[1u8, 2]);
        let mut buffer: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Left, Symbol::Zero, &mut buffer);
        let expected: Vec<u8> = Vec::from(&[1, 2, Symbol::Zero.into()]);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_whitespace_from_char_slice() {
        let width: usize = 4;
        let source: &[char] = &['a', 'b', 'c'];
        let mut buffer: Vec<char> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            source,
            width,
            Alignment::Center,
            Symbol::Whitespace,
            &mut buffer,
        );
        let expected = Vec::from(&['a', 'b', 'c', ' ']);
        assert_eq!(expected, buffer);
    }

    #[test]
    fn wrapper_pad_slice_byte_left_align_zero() {
        let width: usize = 200;
        let mut output: Vec<u8> = Vec::with_capacity(width);
        pad_and_push_to_buffer(
            "aeirjgjkadmval,mr1873481 y uhaegr".as_bytes(),
            width,
            Alignment::Left,
            Symbol::Zero,
            &mut output,
        );

        let mut expected = String::from("aeirjgjkadmval,mr1873481 y uhaegr");
        (0..(width - expected.len())).for_each(|_| expected.push_str("0"));

        assert_eq!(expected.as_bytes(), output);
        assert_ne!(expected.capacity(), output.capacity());
        assert_eq!(width, output.capacity());
    }

    #[test]
    fn wrapper_pad_vec_char_left_align_zero() {
        let output: Vec<char> = pad(vec!['a', 'b', 'c', '0'], 13, Alignment::Left, Symbol::Zero);
        let mut expected: Vec<char> = vec!['a', 'b', 'c', '0'];
        expected.extend_from_slice(&vec!['0'; 9]);

        assert_eq!(expected, output);
        assert_eq!(expected.capacity(), output.capacity());
    }

    #[test]
    fn wrapper_pad_vec_u8_right_align_whitespace() {
        let output: Vec<u8> = pad(
            vec![0u8, 2, 65, 8, 41],
            13,
            Alignment::Right,
            Symbol::Whitespace,
        );
        let mut expected: Vec<u8> = vec![b' '; 8];
        expected.extend_from_slice(&vec![0u8, 2, 65, 8, 41]);

        assert_eq!(expected, output);
        assert_ne!(expected.capacity(), output.capacity());
    }

    #[test]
    fn wrapper_pad_str_center_align_hyphen() {
        let output: String = pad("hejj jag", 20, Alignment::Center, Symbol::Hyphen);
        let mut expected = String::from("------hejj jag");
        expected.push_str("------");

        assert_eq!(expected, output);
        assert_ne!(expected.capacity(), output.capacity());
    }

    #[test]
    fn wrapper_pad_and_push_to_buffer_str_left_align_zero() {
        let width: usize = 20;
        let mut output = String::with_capacity(width);
        pad_and_push_to_buffer(
            "testcool  123",
            width,
            Alignment::Left,
            Symbol::Zero,
            &mut output,
        );

        let mut expected = String::from("testcool  123");
        expected.push_str("0000000");

        assert_eq!(expected, output);
        assert_ne!(expected.capacity(), output.capacity());
        assert_eq!(width, output.capacity());
    }

    #[test]
    fn pad_vec_char_left_align_hypgen() {
        let output: Vec<char> = vec!['a', 'b', 'c'].pad(6, Alignment::Left, Symbol::Hyphen);
        let expected = vec!['a', 'b', 'c', '-', '-', '-'];
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_vec_u8_left_align_hyphen() {
        let output = vec![0u8, 1, 2, 3].pad(6, Alignment::Left, Symbol::Hyphen);
        let mut expected = vec![0u8, 1, 2, 3];
        expected.extend_from_slice("--".as_bytes());

        assert_eq!(expected, output);
        // The Vec that we extend from slice with is `most likely` going to have
        // a capacity of 8, since the Vec implementation doubles the allocated space
        // that is needed as it extends, however, our implementation allocates exactly
        // the amount that is needed.
        assert_ne!(expected.capacity(), output.capacity());
    }

    #[test]
    fn pad_vec_u8_right_align_hyphen() {
        let output = vec![14u8, 12u8, 9u8].pad(5, Alignment::Right, Symbol::Hyphen);
        let mut expected = "--".as_bytes().to_vec();
        expected.extend_from_slice(&vec![14u8, 12u8, 9u8]);

        assert_eq!(expected, output);
        assert_ne!(expected.capacity(), output.capacity());
    }

    #[test]
    fn pad_vec_u8_center_align_hyphen() {
        let output = vec![14u8, 12u8, 9u8].pad(5, Alignment::Center, Symbol::Hyphen);
        let mut expected = "-".as_bytes().to_vec();
        expected.extend_from_slice(&vec![14u8, 12u8, 9u8]);
        expected.extend_from_slice("-".as_bytes());

        assert_eq!(expected, output);
        assert_ne!(expected.capacity(), output.capacity());
    }

    #[test]
    fn pad_str_left_align_hyphen() {
        let output = "hej178".pad(12, Alignment::Left, Symbol::Hyphen);
        let expected = "hej178------".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_right_align_hyphen() {
        let output = "9184".pad(8, Alignment::Right, Symbol::Hyphen);
        let expected = "----9184".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_center_align_hyphen() {
        let output = "hejjj".pad(9, Alignment::Center, Symbol::Hyphen);
        let expected = "--hejjj--".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_center_align_hyphen_even() {
        let output = "hejjj".pad(10, Alignment::Center, Symbol::Hyphen);
        let expected = "--hejjj---".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_neft_align_whitespace() {
        let output = "hej".pad(6, Alignment::Left, Symbol::Whitespace);
        let expected = "hej   ".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_right_align_whitespace() {
        let output = "hejjj".pad(9, Alignment::Right, Symbol::Whitespace);
        let expected = "    hejjj".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_center_align_whitespace() {
        let output = "hejjj".pad(9, Alignment::Center, Symbol::Whitespace);
        let expected = "  hejjj  ".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_center_align_whitespace_even() {
        let output = "hejjj".pad(10, Alignment::Center, Symbol::Whitespace);
        let expected = "  hejjj   ".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_left_align_zero() {
        let output = "hej178".pad(12, Alignment::Left, Symbol::Zero);
        let expected = "hej178000000".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_right_align_zero() {
        let output = "9184".pad(8, Alignment::Right, Symbol::Zero);
        let expected = "00009184".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_center_align_zero() {
        let output = "hejjj".pad(9, Alignment::Center, Symbol::Whitespace);
        let expected = "  hejjj  ".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_center_align_zero_even() {
        let output = "hejjj".pad(10, Alignment::Center, Symbol::Whitespace);
        let expected = "  hejjj   ".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn str_truncate_left_uneven() {
        let output = "kappa".pad(3, Alignment::Left, Symbol::Hyphen);
        let expected = "kap".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn str_truncate_left_even() {
        let output = "kappa".pad(2, Alignment::Left, Symbol::Hyphen);
        let expected = "ka".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn str_truncate_right() {
        let output = "kappa".pad(3, Alignment::Right, Symbol::Hyphen);
        let expected = "ppa".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn str_truncate_center_uneven() {
        let output = "kappa".pad(3, Alignment::Center, Symbol::Hyphen);
        let expected = "app".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn str_truncate_center_even() {
        let output = "kappa".pad(2, Alignment::Center, Symbol::Hyphen);
        let expected = "ap".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn str_no_pad_required() {
        let output = "kappa".pad(5, Alignment::Right, Symbol::Hyphen);
        let expected = "kappa".to_string();
        assert_eq!(expected, output);
    }

    #[test]
    fn slice_truncate_left() {
        let output = vec![0u8, 1, 2, 3, 4]
            .as_slice()
            .pad(3, Alignment::Left, Symbol::Whitespace);
        let expected = vec![0u8, 1, 2];
        assert_eq!(expected, output);
    }

    #[test]
    fn slice_truncate_right() {
        let output =
            vec![0u8, 1, 2, 3, 4, 5, 6]
                .as_slice()
                .pad(5, Alignment::Right, Symbol::Hyphen);
        let expected = vec![2u8, 3, 4, 5, 6];
        assert_eq!(expected, output);
    }

    #[test]
    fn slice_truncate_center_uneven() {
        let output = vec![0u8, 1, 2, 3, 4]
            .as_slice()
            .pad(3, Alignment::Center, Symbol::Whitespace);
        let expected = vec![1u8, 2, 3];
        assert_eq!(expected, output);
    }

    #[test]
    fn slice_truncate_center_even() {
        let output = vec![0u8, 1, 2, 3, 4]
            .as_slice()
            .pad(4, Alignment::Center, Symbol::Whitespace);
        let expected = vec![0u8, 1, 2, 3];
        assert_eq!(expected, output);
    }

    #[test]
    fn vec_truncate_left() {
        let output = vec![0u8, 1, 2, 3, 4].pad(3, Alignment::Left, Symbol::Whitespace);
        let expected = vec![0u8, 1, 2];
        assert_eq!(expected, output);
    }

    #[test]
    fn vec_truncate_right() {
        let output = vec![0u8, 1, 2, 3, 4, 5, 6].pad(5, Alignment::Right, Symbol::Hyphen);
        let expected = vec![2u8, 3, 4, 5, 6];
        assert_eq!(expected, output);
    }

    #[test]
    fn vec_truncate_center_uneven() {
        let output = vec![0u8, 1, 2, 3, 4].pad(3, Alignment::Center, Symbol::Whitespace);
        let expected = vec![1u8, 2, 3];
        assert_eq!(expected, output);
    }

    #[test]
    fn vec_truncate_center_even() {
        let output = vec![0u8, 1, 2, 3, 4].pad(4, Alignment::Center, Symbol::Whitespace);
        let expected = vec![0u8, 1, 2, 3];
        assert_eq!(expected, output);
    }

    #[test]
    fn vec_pad_and_push_to_buffer_right_zero() {
        let width: usize = 8;
        let source = vec!['a', 'b', 'c', 'd', 'e'];
        let mut buffer = Vec::with_capacity(width);
        source.pad_and_push_to_buffer(width, Alignment::Right, Symbol::Zero, &mut buffer);
        let expected = vec!['0', '0', '0', 'a', 'b', 'c', 'd', 'e'];
        assert_eq!(expected, buffer);
    }

    #[test]
    fn vec_no_pad_required() {
        let output = vec![100u8, 14, 15, 98].pad(4, Alignment::Center, Symbol::Hyphen);
        let expected = vec![100u8, 14, 15, 98];
        assert_eq!(expected, output);
    }

    #[test]
    fn default_alignment() {
        let expected = Alignment::Right;
        let default = Alignment::default();
        assert_eq!(expected, default);
    }

    #[test]
    fn default_symbol() {
        let expected = Symbol::Whitespace;
        let default = Symbol::default();
        assert_eq!(expected, default);
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Deserialize, PartialEq)]
    struct DummyAlignment {
        alignment: Alignment,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_alignment() {
        let json = r#"{"alignment": "Right"}"#;
        let output: DummyAlignment = serde_json::from_str(json).unwrap();
        let expected = DummyAlignment {
            alignment: Alignment::Right,
        };
        assert_eq!(expected, output);
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Deserialize, PartialEq)]
    struct DummySymbol {
        symbol: Symbol,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_symbol() {
        let json = r#"{"symbol": "Whitespace"}"#;
        let output: DummySymbol = serde_json::from_str(json).unwrap();
        let expected = DummySymbol {
            symbol: Symbol::Whitespace,
        };
        assert_eq!(expected, output);
    }
}
