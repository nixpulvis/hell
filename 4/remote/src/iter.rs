use std::marker::PhantomData;
use std::iter::Peekable;
use std::io::{Read, Bytes};
use serde::Deserialize;
use serde_json as json;
use serde_json::error::Error as JsonError;

/// Iterator over JSON deserialized data.
pub struct DeserializeJsonIter<'a, T: Deserialize, R: 'a + Read> {
    bytes: Peekable<Bytes<&'a mut R>>,
    _phantom: PhantomData<T>,
}

impl<'a, T: Deserialize, R: Read> DeserializeJsonIter<'a, T, R> {
    /// Returns an iterator given some reader, for example a `TcpStream`.
    pub fn new(reader: &mut R) -> DeserializeJsonIter<T, R> {
        DeserializeJsonIter {
            bytes: reader.bytes().peekable(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: Deserialize, R: Read> Iterator for DeserializeJsonIter<'a, T, R> {
    type Item = Result<T, JsonError>;

    fn next(&mut self) -> Option<Result<T, JsonError>> {
        if self.bytes.peek().is_none() {
            None
        } else {
            let mut deserializer = json::Deserializer::new(&mut self.bytes);
            Some(Deserialize::deserialize(&mut deserializer))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use take5::StackId;
    use message::RequestMessage;

    #[test]
    fn test_iterate_message() {
        let json = "[\"take-turn\",[[[1,2]],[[2,2]],[[3,2]],[[4,2]]]][\"take-turn\",[[[1,2]],[[2,2]],[[3,2]],[[4,2]]]]";
        let mut stream = Cursor::new(json);
        for result in DeserializeJsonIter::new(&mut stream) {
            match result {
                Ok(RequestMessage::TakeTurn(board)) => {
                    assert_eq!(1, board[StackId::A].len());
                },
                _ => assert!(false),
            }
        }
    }
}
