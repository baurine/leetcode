use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    read_size: usize,
    read_times: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            read_size: 0,
            read_times: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.read_size
    }

    pub fn reads(&self) -> usize {
        self.read_times
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_times += 1;
        let readed = self.wrapped.read(buf);
        if let Ok(x) = readed {
            self.read_size += x;
        }
        readed
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    write_size: usize,
    write_times: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            write_size: 0,
            write_times: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.write_size
    }

    pub fn writes(&self) -> usize {
        self.write_times
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_times += 1;
        let writed = self.wrapped.write(buf);
        if let Ok(x) = writed {
            self.write_size += x;
        }
        writed
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
