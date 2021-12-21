use std::str::FromStr;

use thiserror::Error;

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    content: Vec<T>,
}

impl<T> Grid<T> {
    pub fn width(&self) -> isize {
        self.width.try_into().unwrap()
    }

    pub fn height(&self) -> isize {
        self.height.try_into().unwrap()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn idx(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() as isize {
            return None;
        }

        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        Some(x + y * self.width)
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.idx(x, y).map(|i| &self.content[i])
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        self.idx(x, y).map(|i| &mut self.content[i])
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("width of line {0} did not match previous widths")]
    LineMismatch(usize),
    #[error("found unexpected character {c} at position ({x}, {y})")]
    UnexpectedCharater { x: usize, y: usize, c: char },
    #[error("expected trailing newline character")]
    NoTrailingNewline,
}

impl FromStr for Grid<u8> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut height = 0;
        let mut content = Vec::new();

        let mut current_width = 0;

        for c in s.chars() {
            if c == '\n' {
                if *width.get_or_insert(current_width) != current_width {
                    return Err(ParseError::LineMismatch(height));
                }
                current_width = 0;
                height += 1;
            } else {
                content.push(
                    c.to_digit(10)
                        .ok_or(ParseError::UnexpectedCharater {
                            x: current_width,
                            y: height,
                            c,
                        })?
                        .try_into()
                        .unwrap(), // Single digit should fit into u8
                );
                current_width += 1;
            }
        }

        if current_width > 0 {
            return Err(ParseError::NoTrailingNewline);
        }

        Ok(Self {
            width: width.unwrap_or(content.len()),
            height,
            content,
        })
    }
}
