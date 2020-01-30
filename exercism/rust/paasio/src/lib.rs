use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    read_bytes: usize,
    read_times: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            reader: wrapped,
            read_bytes: 0,
            read_times: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.read_bytes
    }

    pub fn reads(&self) -> usize {
        self.read_times
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // self.read_times += 1;
        // let readed = self.reader.read(buf);
        // if let Ok(x) = readed {
        //     self.read_bytes += x;
        // }
        // readed

        self.read_times += 1;
        let bytes = self.reader.read(buf)?;
        self.read_bytes += bytes;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    writer: W,
    write_bytes: usize,
    write_times: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writer: wrapped,
            write_bytes: 0,
            write_times: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.write_bytes
    }

    pub fn writes(&self) -> usize {
        self.write_times
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // self.write_times += 1;
        // let writed = self.writer.write(buf);
        // if let Ok(x) = writed {
        //     self.write_bytes += x;
        // }
        // writed

        self.write_times += 1;
        let bytes = self.writer.write(buf)?;
        self.write_bytes += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
