pub struct Reader {
    data: Vec<u8>,
    idx: usize
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            idx: 0
        }
    }

    pub fn get_idx(&mut self) -> usize {
        self.idx.clone()
    }

    pub fn read(&mut self) -> u8 {
        let x = &self.data[self.idx];
        self.idx += 1;
        x.clone()
    }

    pub fn read_byte(&mut self) -> i8 {
        let x = &self.data[self.idx];
        self.idx += 1;
        x.clone() as i8
    }

    pub fn read_short(&mut self) -> i16 {
        let x = &self.data[self.idx..self.idx + 2];
        self.idx += 2;
        i16::from_be_bytes(x.try_into().expect(format!("failed to convert [u8; 2] -> i16; {}", self.idx - 2).as_str()))
    }

    pub fn read_int(&mut self) -> i32 {
        let x = &self.data[self.idx..self.idx + 4];
        self.idx += 4;
        i32::from_be_bytes(x.try_into().expect(format!("failed to convert [u8; 4] -> i32; {}", self.idx - 4).as_str()))
    }

    pub fn read_long(&mut self) -> i64 {
        let x = &self.data[self.idx..self.idx + 8];
        self.idx += 8;
        i64::from_be_bytes(x.try_into().expect(format!("failed to convert [u8; 8] -> i64; {}", self.idx - 8).as_str()))
    }

    pub fn read_float(&mut self) -> f32 {
        let x = &self.data[self.idx..self.idx + 4];
        self.idx += 4;
        f32::from_be_bytes(x.try_into().expect(format!("failed to convert [u8; 4] -> f32; {}", self.idx - 4).as_str()))
    }
    
    pub fn read_double(&mut self) -> f64 {
        let x = &self.data[self.idx..self.idx + 8];
        self.idx += 8;
        f64::from_be_bytes(x.try_into().expect(format!("failed to convert [u8; 8] -> f64; {}", self.idx - 8).as_str()))
    }

    pub fn peek(&mut self) -> u8 {
        self.data[self.idx].clone()
    }
}