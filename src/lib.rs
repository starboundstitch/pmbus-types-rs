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
}
