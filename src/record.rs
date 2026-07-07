use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub struct Record {
    pub key_len: usize,
    pub value_len: usize,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl Record {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key_len: key.len(),
            value_len: key.len(),
            key: key.as_bytes().to_vec(),
            value: value.as_bytes().to_vec(),
        }
    }

    pub fn read_from(buf_reader: &mut BufReader<File>) -> std::io::Result<Record> {
        let mut buf = [0; size_of::<usize>()];

        buf_reader.read_exact(&mut buf)?;
        let key_len = usize::from_le_bytes(buf);

        buf_reader.read_exact(&mut buf)?;
        let value_len = usize::from_le_bytes(buf);

        let mut key = vec![0; key_len];
        buf_reader.read_exact(&mut key)?;

        let mut value = vec![0; value_len];
        buf_reader.read_exact(&mut value)?;

        Ok(Record {
            key_len,
            value_len,
            key,
            value,
        })
    }

    pub fn write_to(&mut self, buf_writer: &mut BufWriter<File>) -> std::io::Result<()> {
        let key_len_bytes = self.key.len().to_le_bytes();
        let value_len_bytes = self.value.len().to_le_bytes();
        let mut record_bytes = Vec::new();
        record_bytes.extend_from_slice(&key_len_bytes);
        record_bytes.extend_from_slice(&value_len_bytes);
        record_bytes.extend_from_slice(&self.key);
        record_bytes.extend_from_slice(&self.value);

        buf_writer.write_all(&record_bytes)?;
        buf_writer.flush()?;

        Ok(())
    }
}
