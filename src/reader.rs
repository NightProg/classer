#[derive(Debug, Clone, PartialEq)]
pub struct Reader {
    pub data: Vec<u8>,
    pub cursor: usize,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Reader {
        Reader { data, cursor: 0 }
    }

    pub fn read_string(&mut self, length: usize) -> String {
        let slice = &self.data[self.cursor..self.cursor + length];
        self.cursor += length;
        String::from_utf8(slice.to_vec()).unwrap()
    }

    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let slice = &self.data[self.cursor..self.cursor + length];
        self.cursor += length;
        slice.to_vec()
    }

    pub fn read_int8(&mut self) -> u64 {
        let slice = &self.data[self.cursor..self.cursor + 8];
        self.cursor += 8;
        u64::from_be_bytes(slice.try_into().unwrap())
    }

    pub fn read_int4(&mut self) -> u32 {
        let slice = &self.data[self.cursor..self.cursor + 4];
        self.cursor += 4;
        u32::from_be_bytes(slice.try_into().unwrap())
    }

    pub fn read_int2(&mut self) -> u16 {
        let slice = &self.data[self.cursor..self.cursor + 2];
        self.cursor += 2;
        u16::from_be_bytes(slice.try_into().unwrap())
    }

    pub fn read_int1(&mut self) -> u8 {
        let slice = &self.data[self.cursor..self.cursor + 1];
        self.cursor += 1;
        u8::from_be_bytes(slice.try_into().unwrap())
    }

    pub fn read(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn is_at_end(&self) -> bool {
        self.cursor >= self.data.len()
    }
}
