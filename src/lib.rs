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

///
/// Given the below string slice and target pad width 6, with [`Alignment::Left`] and
/// [`PadSymbol::Whitespace`], the resulting output String can be seen on the right:
///
/// +---+---+---+        +---+---+---+---+---+---+
/// | a | b | c |   -->  | a | b | c |   |   |   |
/// +---+---+---+        +---+---+---+---+---+---+
///
///

use std::clone;

/// Exhaustive enum for the alternative ways to pad and format data.
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

///
#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    Whitespace,
    Zero,
    Hyphen,
}

///
impl From<Symbol> for char {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Hyphen => '-',
            Symbol::Whitespace => ' ',
            Symbol::Zero => '0',
        }
    }
}

///
impl From<Symbol> for &[u8] {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Hyphen => "-".as_bytes(),
            Symbol::Whitespace => " ".as_bytes(),
            Symbol::Zero => "0".as_bytes(),
        }
    }
}

pub trait Source {
    type Buffer;
    type Output;

    fn slice_to_fit(&self, width: usize, mode: Alignment) -> Self::Output;
    fn pad(&self, width: usize, mode: Alignment, symbol: Symbol) -> Self::Output;
    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Symbol,
        buffer: &mut Self::Buffer,
    );
}

impl Source for str
where
    char: From<Symbol>,
{
    type Buffer = String;
    type Output = String;

    fn slice_to_fit(&self, width: usize, mode: Alignment) -> Self::Output {
        match mode {
            Alignment::Left => self[0..width].to_string(),
            Alignment::Right => self[(self.len() - width)..].to_string(),
            Alignment::Center => {
                self[(self.len() / 2 - width / 2)..(self.len() / 2 + width / 2)].to_string()
            }
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Symbol) -> Self::Output {
        if width < self.len() {
            return self.slice_to_fit(width, mode);
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

        let pad_char: char = symbol.into();

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

///
impl<T> Source for Vec<T>
where
    T: From<Symbol> + clone::Clone,
    for <'a> &'a [T]: From<Symbol>,
{
    type Buffer = Vec<T>;
    type Output = Vec<T>;

    fn slice_to_fit(&self, width: usize, mode: Alignment) -> Self::Output {
        match mode {
            Alignment::Left => self[..width].to_vec(),
            Alignment::Right => self[(self.len() - width)..].to_vec(),
            Alignment::Center => self[(self.len() / 2 - width / 2)..(self.len() / 2 + width / 2)].to_vec(),
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Symbol) -> Self::Output {
        if width < self.len() { return self.slice_to_fit(width, mode); }

        let mut output: Vec<T> = Vec::with_capacity(width);
        let diff: usize = width - self.len();

        if diff == 0 { return self.to_vec(); }

        let (lpad, rpad) = match mode {
            Alignment::Left => (0, diff),
            Alignment::Right => (diff, 0),
            Alignment::Center => (diff / 2, diff - diff / 2),
        };

        let pad_byte: &[T] = symbol.into();

        (0..lpad).for_each(|_| output.extend_from_slice(pad_byte));
        output.extend_from_slice(self);
        (0..rpad).for_each(|_| output.extend_from_slice(pad_byte));

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

pub trait Target {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_vec_left_align_hyphen() {
        let output = vec![0u8, 1, 2, 3].pad(6, Alignment::Left, Symbol::Hyphen);
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
    fn pad_str_left_align_whitespace() {
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
}
