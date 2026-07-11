use crate::Record;

use std::{
    fs::File,
    io::{BufReader, ErrorKind, Seek, SeekFrom},
};

pub struct Scanner {}

impl Scanner {
    pub fn scan<F>(buf_reader: &mut BufReader<File>, mut callback: F) -> std::io::Result<()>
    where
        F: FnMut(Record, u64),
    {
        buf_reader.seek(SeekFrom::Start(0))?;

        'scan: loop {
            let offset = buf_reader.stream_position()?;

            let maybe_record = Record::read_from(buf_reader);

            match maybe_record {
                Ok(val) => {
                    callback(val, offset);
                }
                Err(err) => {
                    if err.kind() == ErrorKind::UnexpectedEof {
                        break 'scan;
                    }
                }
            }
        }

        Ok(())
    }
}
