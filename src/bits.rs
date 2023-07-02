#[derive(Debug)]
pub struct Bits {
    data: u64,
    pub size: usize,
}

impl Bits {
    pub fn new() -> Self {
        Bits { data: 0, size: 32 }
    }

    pub fn set_size(&mut self, size: usize) {
        assert!(size <= 64, "Size exceeds 64 bits");
        self.size = size;
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        assert!(index < self.size, "Index out of bounds");
        if value {
            self.data |= 1 << index;
        } else {
            self.data &= !(1 << index);
        }
    }

    pub fn get_bit(&self, index: usize) -> bool {
        assert!(index < self.size, "Index out of bounds");
        (self.data >> index) & 1 != 0
    }

    pub fn flip_bit(&mut self, index: usize) {
        assert!(index < self.size, "Index out of bounds");
        self.data ^= 1 << index;
    }

    pub fn to_bit_string(&self) -> String {
        let bit_string: String = (0..self.size)
            .rev()
            .map(|index| if self.get_bit(index) { '1' } else { '0' })
            .collect();
        format!("0b{}", bit_string)
    }

    pub fn to_hex_string(&self) -> String {
        let num_digits = (self.size + 3) / 4;
        format!("0x{:0width$X}", self.data, width = num_digits)
    }

    pub fn to_dec_string(&self) -> String {
        format!("{}", self.data)
    }
}