use std::{fs::File, path::Path, io::Read};
use flate2::read::GzDecoder;

use crate::reader::Reader;

pub struct NBTParser {
    reader: Reader
}

#[derive(Debug)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(i8, Vec<Tag>),
    Compound(Vec<Tag>),

    // newer
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),

    Named(Box<Tag>, Box<Tag>)
}

impl Tag {
    pub fn id(&self) -> i8 {
        match self {
            Tag::End => 0,
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_, _) => 9,
            Tag::Compound(_) => 10,
            Tag::Named(_, tag) => tag.id(),
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12
            //_ => panic!("unhandled tag: {:?}", self)
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        match self {
            Tag::End => bytes.push(0x0),
            Tag::Byte(data) => bytes.push(data.to_be_bytes()[0]),
            Tag::Short(data) => bytes.append(&mut data.to_be_bytes().to_vec()),
            Tag::Int(data) => bytes.append(&mut data.to_be_bytes().to_vec()),
            Tag::Long(data) => bytes.append(&mut data.to_be_bytes().to_vec()),
            Tag::Float(data) => bytes.append(&mut data.to_be_bytes().to_vec()),
            Tag::Double(data) => bytes.append(&mut data.to_be_bytes().to_vec()),
            Tag::ByteArray(data) => {
                let len = data.len() as i32;
                bytes.append(&mut len.to_be_bytes().to_vec());
                bytes.append(&mut data.clone());
            },
            Tag::String(data) => {
                let len = data.len() as i16;
                bytes.append(&mut len.to_be_bytes().to_vec());
                bytes.append(&mut data.as_bytes().to_vec());
            },
            Tag::List(id, tags) => {
                let len = tags.len() as i32;
                bytes.push(id.to_be_bytes()[0]);
                bytes.append(&mut len.to_be_bytes().to_vec());
                
                let children = tags.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
                for mut child in children {
                    bytes.append(&mut child);
                }
            },
            Tag::Compound(tags) => {
                let children = tags.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
                for mut child in children {
                    bytes.append(&mut child);
                }
            },
            Tag::Named(name, value) => {
                bytes.push(self.id().to_be_bytes()[0]);
                bytes.append(&mut name.as_bytes());
                bytes.append(&mut value.as_bytes());
            },
            
            Tag::IntArray(data) => {
                let len = data.len() as i32;
                bytes.append(&mut len.to_be_bytes().to_vec());
                
                let children = data.iter().map(|x| x.to_be_bytes().to_vec()).collect::<Vec<_>>();
                for mut child in children {
                    bytes.append(&mut child);
                }
            },
            Tag::LongArray(data) => {
                let len = data.len() as i32;
                bytes.append(&mut len.to_be_bytes().to_vec());
                
                let children = data.iter().map(|x| x.to_be_bytes().to_vec()).collect::<Vec<_>>();
                for mut child in children {
                    bytes.append(&mut child);
                }
            }
            //_ => panic!("unhandled tag: {:?}", self)
        };

        bytes
    }

}

impl NBTParser {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            reader: Reader::new(data)
        }
    }

    pub fn from_path<P : AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut file = File::open(path).expect("Failed to open file path");
        let mut data = Vec::<u8>::new();
        
        match file.read_to_end(&mut data) {
            Err(err) => Result::Err(err),
            Ok(_) => {
                if data[0] == 0x1F && data[1] == 0x8B && data[2] == 0x8 { // compressed with DEFLATE algorithm (GZIP RFC 1952)
                    let mut buf = Vec::<u8>::new();
                    
                    match GzDecoder::new(data.as_slice()).read_to_end(&mut buf) {
                        Err(err) => return Result::Err(err),
                        Ok(_) => {
                            data = buf;
                        }
                    };
                }

                Result::Ok(NBTParser::new(data))
            }
        }
    }

    fn read_tag(&mut self, id: &i8) -> Tag {
        match id {
            0 => Tag::End,
            1 => Tag::Byte(self.reader.read_byte()),
            2 => Tag::Short(self.reader.read_short()),
            3 => Tag::Int(self.reader.read_int()),
            4 => Tag::Long(self.reader.read_long()),
            5 => Tag::Float(self.reader.read_float()),
            6 => Tag::Double(self.reader.read_double()),
            7 => {
                let len = self.reader.read_int();
                let mut bytes = Vec::<u8>::new();

                for _ in 0..len {
                    bytes.push(self.reader.read());
                }

                Tag::ByteArray(bytes)
            },
            8 => {
                let len = self.reader.read_short();
                let mut chars = Vec::<u8>::new();

                for _ in 0..len {
                    chars.push(self.reader.read());
                }

                Tag::String(String::from_utf8(chars).expect("Failed to read utf-8 string tag"))
            },
            9 => {
                let id = self.reader.read_byte();
                let len = self.reader.read_int();
                let mut tags = Vec::<Tag>::new();

                for _ in 0..len {
                    tags.push(self.read_tag(&id));
                }

                Tag::List(id, tags)
            },
            10 => self.read_compound(),
            
            11 => {
                let len = self.reader.read_int();
                let mut ints = Vec::new();

                for _ in 0..len {
                    ints.push(self.reader.read_int());
                }

                Tag::IntArray(ints)
            },
            12 => {
                let len = self.reader.read_int();
                let mut longs = Vec::new();

                for _ in 0..len {
                    longs.push(self.reader.read_long());
                }

                Tag::LongArray(longs)
            },
            _ => panic!("unhandled id {}; {}", id, self.reader.get_idx())
        }
    }

    fn read_named_tag(&mut self) -> Tag {
        let id = self.reader.read_byte();

        match id {
            0 => {
                Tag::End
            },
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                let name = self.read_tag(&8);
                let tag = self.read_tag(&id);

                Tag::Named(Box::new(name), Box::new(tag))
            },
            _ => panic!("unhandled id {}; {}", id, self.reader.get_idx())
        }
    }

    fn read_compound(&mut self) -> Tag {
        let mut tags = Vec::<Tag>::new();

        let mut id = self.reader.peek();
        while id != 0 {
            tags.push(self.read_named_tag());
            
            id = self.reader.peek();
        }

        self.reader.read(); // consume TAG_End
        tags.push(Tag::End);

        Tag::Compound(tags)
    }

    pub fn parse(&mut self) -> Tag {
        self.read_named_tag()
    }
}