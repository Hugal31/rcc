use std::io::{Result, Write};

use memchr::memchr;

pub struct IndentWriter<'a, W: 'a>
    where W: Write {
    is_new_line: bool,
    inner: &'a mut W,
}

impl<'a, W> IndentWriter<'a, W>
    where W: Write {
    pub fn new(inner: &'a mut W) -> IndentWriter<'a, W> {
        IndentWriter{
            is_new_line: true,
            inner,
        }
    }
}

const TAB: &[u8; 1] = b"\t";
    
impl<'a, W> Write for IndentWriter<'a, W>
    where W: Write {

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.is_new_line {
            if self.inner.write(TAB)? == 0 {
                return Ok(0);
            }
            self.is_new_line = false;
        }

        if let Some(idx) = memchr(b'\n', buf) {
            let result = self.inner.write(&buf[..idx + 1]);
            match result {
                Ok(size) if size == idx + 1 => self.is_new_line = true,
                _ => (),
            }
            result
        } else {
            self.inner.write(buf)
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        use std::io::{Error, ErrorKind};
        while !buf.is_empty() {
            let was_new_line = self.is_new_line;
            match self.write(buf) {
                Ok(0) if was_new_line == self.is_new_line => return Err(Error::new(ErrorKind::WriteZero,
                                                                                      "failed to write whole buffer")),
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SlowWriter {
        pub speed: usize,
        pub inner: Vec<u8>,
    }

    impl Write for SlowWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if buf.len() <= self.speed {
                self.inner.write(buf)
            } else {
                self.inner.write(&buf[..self.speed])
            }
        }

        fn flush(&mut self) -> Result<()> {
            self.inner.flush()
        }
    }

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
        let text = b"Line 1\n";
        let text_size = text.len();
        let size = writer.write(text).unwrap();
        assert_eq!(size, text_size);
    }

    #[test]
    fn test_write_two_time() {
        let mut buf = vec!();
        {
            let mut writer = IndentWriter::new(&mut buf);
            writer.write_all(b"Line").unwrap();
            writer.write_all(b" 1\n").unwrap();
        }
        assert_eq!(buf, b"\tLine 1\n")
    }

    #[test]
    fn test_with_slow_writer() {
        let mut slow_writer = SlowWriter{
            speed: 2,
            inner: vec!(),
        };
        {
            let mut writer = IndentWriter::new(&mut slow_writer);
            writer.write_all(b"Line\n").unwrap();
        }
        assert_eq!(slow_writer.inner, b"\tLine\n");
    }

    #[test]
    fn test_with_1_byte_slow_writer() {
        let mut slow_writer = SlowWriter{
            speed: 1,
            inner: vec!(),
        };
        {
            let mut writer = IndentWriter::new(&mut slow_writer);
            writer.write_all(b"Line\n").unwrap();
        }
        assert_eq!(slow_writer.inner, b"\tLine\n");
    }

    #[test]
    fn test_with_0_byte_slow_buffer() {
        use std::io::{ErrorKind};

        let mut slow_writer = SlowWriter{
            speed: 0,
            inner: vec!(),
        };
        let mut writer = IndentWriter::new(&mut slow_writer);
        let result = writer.write_all(b"Line\n");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::WriteZero);
    }
}
