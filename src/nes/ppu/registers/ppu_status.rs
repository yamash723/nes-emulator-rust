pub struct PpuStatus {
    // 1: VBlank clear by reading this register
    pub vblank_flag: bool,
    // 1: sprite hit
    pub sprite_hit: bool,
    // 0: less than 8, 1: 9 or more
    pub sprite_overflow: bool,
}

impl PpuStatus {
    pub fn new() -> Self {
        PpuStatus {
            vblank_flag: false,
            sprite_hit: false,
            sprite_overflow: false,
        }
    }

    pub fn to_u8(&self) -> u8 {
        (self.sprite_overflow as u8) << 5 |
        (self.sprite_hit as u8)      << 6 |
        (self.vblank_flag as u8)     << 7
    }
}

#[cfg(test)]
mod ppu_status_test {
    use super::*;

    #[test]
    fn to_u8_test() {
        let mut status = PpuStatus::new();
        assert_eq!(status.to_u8(), 0b00000000);

        status.vblank_flag = true;
        assert_eq!(status.to_u8(), 0b10000000);

        status.sprite_hit = true;
        assert_eq!(status.to_u8(), 0b11000000);

        status.sprite_overflow = true;
        assert_eq!(status.to_u8(), 0b11100000);
    }
}
