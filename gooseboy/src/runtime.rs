use core::fmt;

pub struct StackWriter<'a> {
    pub buf: &'a mut [u8],
    pub pos: usize,
}

impl<'a> fmt::Write for StackWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len().min(self.buf.len() - self.pos);
        self.buf[self.pos..self.pos + len].copy_from_slice(&bytes[..len]);
        self.pos += len;
        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut buf = [0u8; 256];
        let mut writer = $crate::runtime::StackWriter { buf: &mut buf, pos: 0 };
        let _ = core::write!(&mut writer, $($arg)*);

        unsafe {
            $crate::bindings::log(writer.buf.as_ptr() as i32, writer.pos as i32);
        }
    }};
}
