use std::str::FromStr;

use thiserror::Error;

pub struct DigitGrid {
    width: usize,
    height: usize,
    content: Vec<u8>,
}

impl DigitGrid {
    pub fn width(&self) -> isize {
        self.width.try_into().unwrap()
    }

    pub fn height(&self) -> isize {
        self.height.try_into().unwrap()
    }

    pub fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return None;
        }

        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        Some(self.content[x + y * self.width])
    }
}

#[derive(Debug, Error)]
pub enum DigitGridParseError {
    #[error("width of line {0} did not match previous widths")]
    LineMismatch(usize),
    #[error("found unexpected character {c} at position ({x}, {y})")]
    UnexpectedCharater { x: usize, y: usize, c: char },
    #[error("expected trailing newline character")]
    NoTrailingNewline,
}

impl FromStr for DigitGrid {
    type Err = DigitGridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut height = 0;
        let mut content = Vec::new();

        let mut current_width = 0;

        for c in s.chars() {
            if c == '\n' {
                if *width.get_or_insert(current_width) != current_width {
                    return Err(DigitGridParseError::LineMismatch(height));
                }
                current_width = 0;
                height += 1;
            } else {
                content.push(
                    c.to_digit(10)
                        .ok_or(DigitGridParseError::UnexpectedCharater {
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
            return Err(DigitGridParseError::NoTrailingNewline);
        }

        Ok(Self {
            width: width.unwrap_or(content.len()),
            height,
            content,
        })
    }
}
