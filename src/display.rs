pub struct Display {
    data: [bool; 64 * 32],
}

impl Display {
    pub fn new() -> Display {
        Display {
            data: [false; 64 * 32],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(false);
    }

    pub fn get_data(&self) -> [bool; 64 * 32] {
        self.data
    }

    pub fn get_pixel(&self, position: u16) -> bool {
        self.data[position as usize]
    }

    pub fn set_pixel(&mut self, position: u16, value: bool) {
        self.data[position as usize] = value;
    }
}