mod storage;

use storage::{Record, Scanner};

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write, stdout};
use std::str;
use std::{fs::File, io::stdin};

#[derive(Debug)]
struct LogPointer {
    offset: u64,
}

struct Bitcask {
    index: HashMap<Vec<u8>, LogPointer>,
    buf_reader: BufReader<File>,
    buf_writer: BufWriter<File>,
}

impl Bitcask {
    pub fn new(db_path: &str) -> Self {
        let db_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(db_path)
            .expect("can't open a file");
        let file_writer_clone = db_file.try_clone().expect("can't clone a file");
        let file_reader_clone = db_file.try_clone().expect("can't clone a file");
        let buf_writer = BufWriter::new(file_writer_clone);
        let buf_reader = BufReader::new(file_reader_clone);

        Self {
            index: HashMap::new(),
            buf_writer,
            buf_reader,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> std::io::Result<()> {
        Record::new(key, value).write_to(&mut self.buf_writer)?;
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Option<Record> {
        let log_pointer = self.index.get(key.as_bytes());

        if let Some(val) = log_pointer {
            self.buf_reader
                .seek(SeekFrom::Start(val.offset))
                .expect("Can't seek reader position");

            let maybe_record =
                Record::read_from(&mut self.buf_reader).map_or(None, |val| Some(val));

            return maybe_record;
        }

        None
    }

    pub fn scan(&mut self) -> std::io::Result<()> {
        Scanner::scan(&mut self.buf_reader, |record, offset| {
            self.index.insert(record.key, LogPointer { offset });
        })?;
        println!("Index: {:?}", self.index);

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut bitcask = Bitcask::new("db");
    bitcask.scan()?;

    loop {
        let mut prompt = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut prompt).expect("can't read line");
        let parsed_prompt: Vec<&str> = prompt.trim_end().split_whitespace().collect();
        let command = parsed_prompt[0];

        match command {
            "set" => {
                let key = parsed_prompt[1];
                let value = parsed_prompt[2];
                println!("{}, {}", key, value);

                bitcask.set(key, value)?;
            }
            "get" => {
                let key = parsed_prompt[1];
                let value = bitcask.get(key);

                match value {
                    Some(val) => {
                        println!("{:?}", str::from_utf8(&val.value).unwrap());
                    }
                    None => {
                        println!("Not found")
                    }
                }
            }
            "scan" => {
                bitcask.scan()?;
            }
            _ => {}
        }
    }
}
