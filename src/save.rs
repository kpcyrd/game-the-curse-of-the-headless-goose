// Maximum size we are willing to save/load
const SAVE_SIZE: usize = 256;

pub struct Save {
    pub buf: [u8; SAVE_SIZE],
    pub cursor: usize,
    pub capacity: usize,
}

impl Save {
    pub const fn new() -> Self {
        Self {
            buf: [0; SAVE_SIZE],
            cursor: 0,
            capacity: 0,
        }
    }

    pub fn reset(&mut self, capacity: usize) {
        self.capacity = capacity;
        self.cursor = 0;
    }

    // This is needed because embedded-savegame currently expects this because of w25q
    pub fn slice(&mut self) -> &mut [u8] {
        &mut self.buf[..self.capacity]
    }

    fn take(&mut self, amount: usize) -> Option<&[u8]> {
        let slice = self
            .buf
            .get(..self.capacity)
            .unwrap_or_default()
            .get(self.cursor..)
            .unwrap_or_default()
            .get(..amount);
        self.cursor = self.cursor.saturating_add(amount);
        slice
    }

    pub fn pull_u16(&mut self, default: u16) -> u16 {
        if let Some(value) = self.take(2) {
            let value = u16::from_be_bytes(value.try_into().unwrap());
            value
        } else {
            default
        }
    }

    pub fn pull_u8(&mut self, default: u8) -> u8 {
        if let Some(value) = self.take(1) {
            value[0]
        } else {
            default
        }
    }

    fn push(&mut self, buf: &[u8]) {
        let range = self.cursor..self.cursor + buf.len();
        if let Some(slice) = self.buf.get_mut(range) {
            slice.copy_from_slice(buf);
            self.cursor = self.cursor.saturating_add(buf.len());
            self.capacity = self.cursor;
        }
    }

    pub fn push_u16(&mut self, value: u16) {
        self.push(&value.to_be_bytes());
    }

    pub fn push_u8(&mut self, value: u8) {
        self.push(&[value]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_pull() {
        let mut save = Save::new();
        save.buf[..4].copy_from_slice(&[0x12, 0x34, 0x56, 0x78]);
        save.reset(4);

        assert_eq!(save.pull_u16(0xFF), 0x1234);
        assert_eq!(save.pull_u8(0xFF), 0x56);
        assert_eq!(save.pull_u8(0xFF), 0x78);
    }

    #[test]
    fn test_save_push() {
        let mut save = Save::new();
        assert_eq!(save.slice(), &[]);

        save.push_u16(0x1234);
        save.push_u8(0x56);
        save.push_u8(0x78);

        assert_eq!(save.slice(), &[0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_save_pull_from_empty() {
        let mut save = Save::new();
        assert_eq!(save.pull_u16(0x1337), 0x1337);
    }
}
