use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Seek, SeekFrom, Write, stdout};
use std::{fs::File, io::stdin};

struct Record {
    key_len: usize,
    value_len: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Debug)]
struct LogPointer {
    offset: u64,
}

struct Bitcask {
    file: File,
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
            file: db_file,
            index: HashMap::new(),
            buf_writer,
            buf_reader,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let key_bytes = key.as_bytes();
        let value_bytes = value.as_bytes();
        let key_len = key_bytes.len().to_le_bytes();
        let value_len = value_bytes.len().to_le_bytes();
        let mut all_bytes = Vec::new();
        all_bytes.extend_from_slice(&key_len);
        all_bytes.extend_from_slice(&value_len);
        all_bytes.extend_from_slice(key_bytes);
        all_bytes.extend_from_slice(value_bytes);

        self.buf_writer
            .write_all(&all_bytes)
            .expect("can't write to db");
        self.buf_writer.flush().expect("can't flush a buf writer");
    }

    pub fn get(&mut self, key: &str) -> Option<&LogPointer> {
        self.index.get(key.as_bytes())
    }

    pub fn read_record(&mut self) -> std::io::Result<Record> {
        let mut buf = [0; size_of::<usize>()];

        self.buf_reader.read_exact(&mut buf)?;
        let key_len = usize::from_le_bytes(buf);

        self.buf_reader.read_exact(&mut buf)?;
        let value_len = usize::from_le_bytes(buf);

        let mut key = vec![0; key_len];
        self.buf_reader.read_exact(&mut key)?;

        let mut value = vec![0; value_len];
        self.buf_reader.read_exact(&mut value)?;

        Ok(Record {
            key_len,
            value_len,
            key,
            value,
        })
    }

    pub fn scan(&mut self) -> std::io::Result<()> {
        self.buf_reader.seek(SeekFrom::Start(0));

        'scan: loop {
            let position = self.buf_reader.stream_position()?;

            match self.read_record() {
                Ok(record) => {
                    println!("key = {:?}, value = {:?}", record.key, record.value);
                    self.index
                        .insert(record.key, LogPointer { offset: position });
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

fn main() {
    let mut bitcask = Bitcask::new("db");

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

                bitcask.set(key, value);
            }
            "get" => {
                let key = parsed_prompt[1];
                let value = bitcask.get(key);

                match value {
                    Some(val) => {
                        println!("{:?}", val);
                    }
                    None => {
                        println!("Not found")
                    }
                }
            }
            "scan" => {
                bitcask.scan();
            }
            _ => {}
        }
    }
}
