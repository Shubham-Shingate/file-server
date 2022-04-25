
use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::str;

pub struct LinesCodec {
    // Our buffered reader & writers
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl LinesCodec {
    /// Encapsulate a TcpStream with buffered reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        // Both BufReader and LineWriter need to own a stream
        // We can clone the stream to simulate splitting Tx & Rx with `try_clone()`
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    /// Write the given message (appending a newline) to the TcpStream
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        // This will also signal a `writer.flush()` for us; thanks LineWriter!
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    /// Read a received message from the TcpStream
    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        // Use `BufRead::read_line()` to read a line from the TcpStream
        self.reader.read_line(&mut line)?;
        line.pop(); // Remove the trailing "\n"
        Ok(line)
    }

    pub fn read_file_socket(&mut self) -> io::Result<String> {
        let mut all_lines = String::new();
        // Use `BufRead::read_line()` to read all lines from the TcpStream
        let mut this_line = String::new();
        while let Ok(_) = self.reader.read_line(&mut this_line) {
            if this_line.starts_with("e*-o") {
                break;
            }
            all_lines = all_lines + &this_line;
            this_line.clear();
        }

        Ok(all_lines)
    }

}
