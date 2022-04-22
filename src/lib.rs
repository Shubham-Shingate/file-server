use std::io::{self, BufRead, BufReader, LineWriter, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::fs::File;
use tempfile::tempfile;

pub struct LinesCodec {
    // Our buffered reader & writers
    reader: BufReader<TcpStream>,
    writer: LineWriter<TcpStream>,
}

impl LinesCodec {
    /// Encapsulate a TcpStream with buffered reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        // Both BufReader and LineWriter need to own a stream
        // We can clone the stream to simulate splitting Tx & Rx with `try_clone()`
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    // change read timeout, used for attachment checks
    pub fn set_timeout(&mut self, time: u64) {
        match time{
            0 => self.reader.get_mut().set_read_timeout(None), // reset
            _ => self.reader.get_mut().set_read_timeout(Some(Duration::from_secs(time))),
        };
    }

    /// Write the given message (appending a newline) to the TcpStream
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        self.writer.write(&['\n' as u8])?; // This will also signal a `writer.flush()` for us; thanks LineWriter!
        Ok(())
    }

    /// Read a received message from the TcpStream
    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?; // Use `BufRead::read_line()` to read a line from the TcpStream
        line.pop(); // Remove the trailing "\n"
        Ok(line)
    }

    // Write the given file (appending a newline) to the TcpStream
    pub fn send_file(&mut self, file: &mut File) -> io::Result<()> {
        let writer = self.writer.get_mut(); // copy writer in linewriter
        io::copy(file, writer)?; // write directly to tcp
        writer.flush()?;
        Ok(())
    }

    // Read a received file from the TcpStream
    pub fn read_file(&mut self) -> io::Result<File> {
        let mut file = tempfile()?; // create tempfile to write to
        io::copy(&mut self.reader, &mut file)?; // copy tcp to temp file
        Ok(file) // return tempfile
    }
}