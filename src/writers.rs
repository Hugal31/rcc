use std::io::{Result, Write};

pub struct IndentWriter<'a, W: 'a>
    where W: Write {
    inner: &'a mut W
}

impl<'a, W> IndentWriter<'a, W>
    where W: Write {
    pub fn new(inner: &'a mut W) -> IndentWriter<'a, W> {
        IndentWriter{
            inner
        }
    }
}

/// TODO Use other thing than writer
impl<'a, W> Write for IndentWriter<'a, W>
    where W: Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut tmp_buf = buf.to_vec();
        tmp_buf.insert(0, b'\t');

        let mut offset = 0;
        while let Some(idx) = tmp_buf[offset..].iter().position(|&c| c == b'\n') {
            offset += idx + 1;
            if offset == tmp_buf.len() {
                break;
            }
            tmp_buf.insert(offset, b'\t');
        }

        self.inner.write_all(&tmp_buf).map(|_| buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_single_line() {
        let mut buf = vec!();
        {
            let mut writer = IndentWriter::new(&mut buf);

            writer.write_all(b"Test indent").unwrap();
        }
        assert_eq!(buf, b"\tTest indent");
    }

    #[test]
    fn test_indent_multiple_lines() {
        let mut buf = vec!();
        {
            let mut writer = IndentWriter::new(&mut buf);
            writer.write_all(b"Line 1\n").unwrap();
            writer.write_all(b"Line 2\nLine 3\n").unwrap();
        }
        assert_eq!(buf, b"\tLine 1\n\tLine 2\n\tLine 3\n");
    }

    #[test]
    fn test_size_without_indent() {
        let mut buf = vec!();
        let mut writer = IndentWriter::new(&mut buf);
        let text = b"Line 1\nLine 2\n";
        let text_size = text.len();
        let size = writer.write(text).unwrap();
        assert_eq!(size, text_size);
    }
}
