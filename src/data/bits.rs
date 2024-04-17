use std::slice::Iter;

// TODO: see extern crate bitvector for better optimization
pub struct Bits {
    size: u8,
    data: Vec<bool>,
}

impl Bits {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn from_vector_b(bools: Vec<bool>, size: Option<u8>) -> Self {
        let size = size.unwrap_or(bools.len() as u8);
        Bits { size, data: bools }
    }

    pub fn from_slice_b(bools: &[bool], size: Option<u8>) -> Self {
        let size = size.unwrap_or(bools.len() as u8);
        let padding = size as usize - bools.len().min(size as usize);

        let data = std::iter::repeat(false)
            .take(padding)
            .chain(bools.iter().cloned().take(size as usize - padding))
            .collect::<Vec<bool>>();
        Bits::from_vector_b(data, Some(size))
    }

    pub fn from_slice_i(ints: &[u8], size: Option<u8>) -> Self {
        let bools = ints.iter().map(|&n| n == 1).collect::<Vec<bool>>();
        Bits::from_slice_b(&bools, size)
    }

    pub fn from_bits(bits: &Bits) -> Self {
        Bits::from_vector_b(bits.data.clone(), Some(bits.size))
    }

    pub fn from_int(value: u32, size: Option<u8>) -> Self {
        let real_size = if let Some(size) = size {
            size
        } else {
            let size_str = format!("{value:b}");
            size_str.len() as u8
        };

        if value >= u32::pow(2, real_size.into()) {
            panic!(
                "The given value is bigger than the value that can be represented with {} bits",
                real_size
            );
        }

        let data = (0..real_size).rev().map(|n| ((value >> n) & 1) != 0);

        Bits {
            size: real_size,
            data: data.collect(),
        }
    }

    pub fn to_int(&self) -> u32 {
        self.data
            .iter()
            .rev()
            .enumerate()
            .map(|(place, bit)| u32::from(*bit) << place)
            .sum()
    }

    // pub fn reverse(&mut self) {
    //     self._data.reverse();
    // }
    pub fn data(&self) -> Vec<bool> {
        self.data.clone()
    }

    pub fn iter(&self) -> Iter<'_, bool> {
        self.data.iter()
    }
}

impl PartialEq<Bits> for Bits {
    fn eq(&self, other: &Bits) -> bool {
        self.to_int() == other.to_int()
    }
}

impl<T> PartialEq<T> for Bits
where
    T: AsRef<[bool]> + ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        self.to_int() == Bits::from_slice_b(other.as_ref(), None).to_int()
    }
}

impl PartialEq<Bits> for &[bool] {
    fn eq(&self, other: &Bits) -> bool {
        other.data == *self
    }
}

impl PartialEq<Bits> for [bool] {
    fn eq(&self, other: &Bits) -> bool {
        other.data == *self
    }
}

// impl PartialEq<Bits> for [bool] {
//     fn eq(&self, other: &Bits) -> bool {
//         other == self
//     }
// }

impl<Idx> std::ops::Index<Idx> for Bits
where
    Idx: std::slice::SliceIndex<[bool]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_from_slice_b() {
        let bits = Bits::from_slice_b(&[false, true, false, true], Some(8));
        assert!(bits.to_int() == 5);
        assert!(bits.len() == 8);
    }

    #[test]
    fn bits_from_slice_b_no_size() {
        let bits = Bits::from_slice_b(&[false, true, false, true], None);
        assert!(bits.to_int() == 5);
        assert!(bits.len() == 4);
    }

    #[test]
    fn bits_from_slice_i() {
        let bits = Bits::from_slice_i(&[1, 0, 0, 1], Some(8));
        assert!(bits.to_int() == 9);
        assert!(bits.len() == 8);
    }

    #[test]
    fn bits_from_int() {
        let bits = Bits::from_int(125, Some(8));
        assert!(bits.to_int() == 125);
        assert!(bits.len() == 8);
    }

    #[test]
    fn bits_from_int_no_size() {
        let bits = Bits::from_int(509, None);
        assert!(bits.to_int() == 509);
        assert!(bits.len() == 9);
    }

    #[test]
    #[should_panic]
    fn bits_from_int_too_big() {
        Bits::from_int(253, Some(4));
    }

    #[test]
    fn bits_equal() {
        let a = Bits::from_int(253, Some(8));
        let b = Bits::from_int(253, Some(8));
        assert!(a == b);

        let a = Bits::from_int(253, Some(8));
        let b = Bits::from_int(253, Some(16));
        assert!(a == b);
    }

    #[test]
    fn bits_not_equal() {
        let a = Bits::from_int(253, Some(8));
        let b = Bits::from_int(125, Some(8));
        assert!(a != b);

        let a = Bits::from_int(253, Some(8));
        let b = Bits::from_int(125, Some(16));
        assert!(a != b);
    }
}
