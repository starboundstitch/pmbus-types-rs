#![no_std]

pub mod ulinear16 {
    const EXPONENT: f32 = 0.0009765625;

    pub fn from(val: f32) -> u16 {
        (val / EXPONENT) as u16
    }

    pub fn to(val: u16) -> f32 {
        val as f32 * EXPONENT
    }
}

pub mod slinear11 {
    pub fn from(val: f32, exp: i8) -> u16 {
        let base: u32 = 2;
        let lsb: f32 = if exp.is_positive() {
            base.pow(exp as u32) as f32
        } else {
            1. / base.pow(exp.unsigned_abs() as u32) as f32
        };

        let mant: i32 = (val / lsb) as i32;
        let mant = mant & 0x07FF;
        let exp: u32 = (exp & 0x1F) as u32;
        (mant as u32 | (exp << 11)) as u16
    }

    pub fn to(val: u16) -> f32 {
        let mut exp = (val >> 11) as i8;
        if exp > 16 {
            exp -= 32;
        }
        let mut mant: u32 = val as u32 & 0x000007FF;
        if mant > 0x03FF {
            mant |= 0xFFFFF800;
        }

        let base: u8 = 2;
        // Workaround not having access to powi
        if exp.is_positive() {
            mant as f32 * base.pow(exp as u32) as f32
        } else {
            mant as f32 / base.pow(exp.unsigned_abs() as u32) as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_to_ulinear16() {
        let result = ulinear16::from(1.0);
        assert_eq!(result, 1024);
        assert_ne!(result, 59);
    }

    #[test]
    fn ulinear16_to_float() {
        let result = ulinear16::to(1024);
        assert_eq!(result, 1.0);
        assert_ne!(result, 1.2);
    }

    #[test]
    fn slinear11_to_float() {
        let result = slinear11::to(0xE804);
        assert_eq!(result, 0.5);
        assert_ne!(result, 0.3);
        assert_eq!(slinear11::to(0xE054), 5.25);
    }

    #[test]
    fn float_to_slinear11() {
        let result = slinear11::from(5.25, -4);
        assert_eq!(result, 0xE054);
        assert_ne!(result, 0xE000);
    }

    // #[test]
    // fn chain() {
    //     let val = 3.26;
    //     let lin = slinear11::from(val, -4);
    //     // Fails Due to accuracy constraints in slinear11 format
    //     assert_eq!(slinear11::to(lin), val);
    // }
}
