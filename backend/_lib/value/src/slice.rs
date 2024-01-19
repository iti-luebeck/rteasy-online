use super::{Bit, Value};
use std::borrow::ToOwned;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
#[repr(C)]
pub struct ValueSlice {
    pub(crate) bits: [Bit],
}

impl ValueSlice {
    pub fn size(&self) -> usize {
        self.bits.len()
    }

    pub fn write(&mut self, value: &ValueSlice) {
        for (idx, bit) in self.bits.iter_mut().enumerate() {
            *bit = value.bits.get(idx).copied().unwrap_or_default();
        }
    }

    pub fn is_zero(&self) -> bool {
        self.bits.iter().all(|b| *b == Bit::Zero)
    }

    pub fn as_bin(&self, with_leading_zeros: bool) -> String {
        if self.is_zero() && !with_leading_zeros {
            return "0".to_string();
        }

        let mut result = String::new();

        for &b in self.bits.iter().rev().skip_while(|&&b| b == Bit::Zero && !with_leading_zeros) {
            match b {
                Bit::Zero => result.push('0'),
                Bit::One => result.push('1'),
            }
        }

        result
    }

    pub fn as_dec(&self) -> String {
        self.as_radix(false, 10)
    }

    pub fn as_dec_signed(&self) -> String {
        self.as_radix(true, 10)
    }

    pub fn as_hex(&self) -> String {
        self.as_radix(false, 16)
    }

    pub fn as_hex_with_leading_zeros(&self) -> String {
        let result = self.as_hex();
        let len = (self.size() as i32) / 4 - (result.len() as i32);
        if len <= 0 {
            result
        } else {
            format!("{}{}", "0".repeat(len as usize), result)
        }
    }

    /// # Panics
    ///
    /// Panics if given a radix larger than 36.
    fn as_radix(&self, signed: bool, radix: u32) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let mut value = self.to_owned();
        let mut result = Vec::new();
        let negative = signed && value.bits.last().unwrap() == &Bit::One && value.bits.len() > 1;

        if negative {
            value = -value;
            value.remove_leading_zeros();
        }

        while !value.is_zero() {
            let mut r = 0;
            let mut value_rest = Vec::new();

            for &b in value.bits.iter().rev() {
                r = 2 * r + u32::from(b);
                if r >= radix {
                    value_rest.push(Bit::One);
                    r -= radix;
                } else {
                    value_rest.push(Bit::Zero);
                }
            }

            value = Value { bits: value_rest.into_iter().rev().collect() };
            value.remove_leading_zeros();
            result.push(char::from_digit(r, radix).unwrap().to_ascii_uppercase());
        }

        if negative {
            result.push('-');
        }

        result.into_iter().rev().collect()
    }
}

impl ToOwned for ValueSlice {
    type Owned = Value;

    fn to_owned(&self) -> Self::Owned {
        Value { bits: self.bits.into() }
    }
}

impl Hash for ValueSlice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Remove leading zeros
        let mut len = self.bits.len();
        while len > 1 {
            if self.bits[len - 1] == Bit::Zero {
                len -= 1;
            } else {
                break;
            }
        }

        self.bits[0..len].hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_bin() {
        assert_eq!(Value { bits: vec![Bit::Zero, Bit::Zero,] }.as_bin(false), "0".to_string());
        assert_eq!(Value { bits: vec![Bit::Zero, Bit::Zero,] }.as_bin(true), "00".to_string());
        assert_eq!(Value { bits: vec![Bit::One, Bit::Zero,] }.as_bin(false), "1".to_string());
        assert_eq!(Value { bits: vec![Bit::Zero, Bit::One,] }.as_bin(false), "10".to_string());
        assert_eq!(
            Value { bits: vec![Bit::One, Bit::One, Bit::Zero, Bit::One, Bit::Zero] }.as_bin(false),
            "1011".to_string()
        );
        assert_eq!(
            Value { bits: vec![Bit::One, Bit::One, Bit::Zero, Bit::One, Bit::Zero] }.as_bin(true),
            "01011".to_string()
        );

        assert_eq!(
            Value {
                bits: vec![
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                ]
            }
            .as_bin(false),
            "11000101110".to_string()
        );
    }

    #[test]
    fn test_as_dec() {
        assert_eq!(Value { bits: vec![Bit::Zero, Bit::Zero,] }.as_dec(), "0".to_string());
        assert_eq!(Value { bits: vec![Bit::One, Bit::Zero,] }.as_dec(), "1".to_string());
        assert_eq!(Value { bits: vec![Bit::Zero, Bit::One,] }.as_dec(), "2".to_string());

        assert_eq!(
            Value {
                bits: vec![
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                ]
            }
            .as_dec(),
            "791".to_string()
        );
    }

    #[test]
    fn test_as_hex() {
        assert_eq!(Value { bits: vec![Bit::Zero, Bit::Zero,] }.as_hex(), "0".to_string());
        assert_eq!(Value { bits: vec![Bit::One, Bit::Zero,] }.as_hex(), "1".to_string());
        assert_eq!(Value { bits: vec![Bit::Zero, Bit::One,] }.as_hex(), "2".to_string());
        assert_eq!(
            Value { bits: vec![Bit::One, Bit::One, Bit::One, Bit::One,] }.as_hex(),
            "F".to_string()
        );

        assert_eq!(
            Value {
                bits: vec![
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                ]
            }
            .as_hex(),
            "62E".to_string()
        );
    }
}
