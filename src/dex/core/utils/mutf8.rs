use std::fmt;

/// A lazy iterator that decodes Android's Modified UTF-8 directly from a byte slice.
pub struct Mutf8Iterator<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Mutf8Iterator<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
}

impl<'a> Iterator for Mutf8Iterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.bytes.len() {
            return None;
        }

        let b1 = self.bytes[self.pos] as u32;
        self.pos += 1;

        if b1 == 0 { return None; }

        let code_point = if b1 < 0x80 {
            b1
        } else if (b1 & 0xe0) == 0xc0 {
            let b2 = self.bytes.get(self.pos).copied().unwrap_or(0) as u32;
            self.pos += 1;
            ((b1 & 0x1f) << 6) | (b2 & 0x3f)
        } else if (b1 & 0xf0) == 0xe0 {
            let b2 = self.bytes.get(self.pos).copied().unwrap_or(0) as u32;
            self.pos += 1;
            let b3 = self.bytes.get(self.pos).copied().unwrap_or(0) as u32;
            self.pos += 1;
            ((b1 & 0x0f) << 12) | ((b2 & 0x3f) << 6) | (b3 & 0x3f)
        } else {
            b1
        };

        std::char::from_u32(code_point).or(Some('?'))
    }
}

/// Wrapper for easy display without allocation
pub struct Mutf8Display<'a>(pub &'a [u8]);

impl<'a> fmt::Display for Mutf8Display<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in Mutf8Iterator::new(self.0) {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Debug for Mutf8Display<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for c in Mutf8Iterator::new(self.0) {
            write!(f, "{}", c.escape_debug())?;
        }
        write!(f, "\"")?;
        Ok(())
    }
}
