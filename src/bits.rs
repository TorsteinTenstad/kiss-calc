pub type DataType = u32;

#[derive(Debug)]
pub struct Bits {
    pub data: DataType,
}

impl Bits {
    pub fn new() -> Self {
        Bits { data: 0}
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        if value {
            self.data |= 1 << index;
        } else {
            self.data &= !(1 << index);
        }
    }

    pub fn get_bit(&self, index: usize) -> bool {
        (self.data >> index) & 1 != 0
    }

    pub fn flip_bit(&mut self, index: usize) {
        self.data ^= 1 << index;
    }

    pub fn to_radix_string(&self, radix: u32) -> String {
        match radix {
            2 => format!("{:#b}", self.data),
            16 => format!("{:#X}", self.data),
            _ => format!("{}", self.data),
        }
    }
}