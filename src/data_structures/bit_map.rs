pub struct BitMap {
    bytes: Vec<u16>,
    nbits: usize,
}

impl BitMap {
    pub fn new(nbits: usize) -> BitMap {
        BitMap {
            nbits,
            bytes: Vec::with_capacity(nbits / 16 + 1),
        }
    }

    pub fn set(&mut self, k: usize) {
        if k > self.nbits {
            return
        }
        let byte_index = k / 16;
        let bit_index = k % 16;
        self.bytes[byte_index] |= 1 << bit_index;
    }

    pub fn get(&self, k: usize) -> bool {
        if k > self.nbits {
            return false;
        }
        let byte_index = k / 16;
        let bit_index = k % 16;
        self.bytes[byte_index] & (1 << bit_index) != 0
    }
}