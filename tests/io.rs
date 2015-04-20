extern crate podio;

use std::io;
use std::io::prelude::*;
use podio::ReadPodExt;

struct TestReader {
    state: u32,
}

impl TestReader {
    fn new(state: u32) -> TestReader {
        TestReader { state: state }
    }

    fn get(&mut self) -> io::Result<u32> {
        self.read_u32::<podio::LittleEndian>()
    }

    fn test(state: u32) -> io::Result<u32> {
        TestReader::new(state).get()
    }
}

impl Read for TestReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let oldstate = self.state;
        self.state = 0;
        match oldstate {
            0 => {},
            1 => return Err(io::Error::new(io::ErrorKind::Interrupted, "Interrupted")),
            2 => return Ok(0),
            _ => {},
        }
        buf[0] = 0;
        Ok(1)
    }
}

#[test]
fn interrupted() {
    // Getting an io::ErrorKind::Interrupted should be retried, and thus succeed
    assert_eq!(TestReader::test(1).unwrap(), 0);
}

#[test]
fn eof() {
    // Getting a Ok(0) implies an unexpected EOF
    assert!(TestReader::test(2).is_err());
}
