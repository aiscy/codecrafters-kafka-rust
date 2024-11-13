use std::io::{Seek, SeekFrom, Write};

pub(crate) struct Counter<T> {
    inner: T,
    size: core::num::Wrapping<usize>,
}

impl<T> Counter<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self {
            inner,
            size: core::num::Wrapping(0),
        }
    }

    pub(crate) fn size(&self) -> usize {
        self.size.0
    }
}

impl<T: Write> Write for Counter<T> {
    fn write(&mut self, buf: &[u8]) -> binrw::io::Result<usize> {
        self.size += buf.len();

        self.inner.write(buf)
    }

    fn flush(&mut self) -> binrw::io::Result<()> {
        self.inner.flush()
    }
}

impl<T: Seek> Seek for Counter<T> {
    fn seek(&mut self, pos: SeekFrom) -> binrw::io::Result<u64> {
        self.inner.seek(pos)
    }
}