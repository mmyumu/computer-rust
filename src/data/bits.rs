
// TODO: see extern crate bitvector for better optimization
pub struct Bits {
    _size: u8,
    _data: Vec<bool>
}

impl Bits {
    pub fn len(&self) -> usize {
        return self._data.len()
    }

    pub fn from_vector_b(bools: Vec<bool>, size: Option<u8>) -> Self {
        let size = size.unwrap_or(bools.len() as u8);
        Bits { _size: size, _data: bools }
    }

    pub fn from_slice_b(bools: &[bool], size: Option<u8>) -> Self {
        let size = size.unwrap_or(bools.len() as u8);
        let padding = size as usize - bools.len().min(size as usize);

        // Préfixer avec `false`, ensuite les éléments de `bools`, limité par `size`.
        let data = std::iter::repeat(false)
            .take(padding) // Prendre le nombre de `false` calculé.
            .chain(bools.iter().cloned().take(size as usize - padding)) // Ajouter les éléments de `bools`.
            .collect::<Vec<bool>>(); // Collecte les éléments dans un Vec<bool>.
        Bits::from_vector_b(data, Some(size))
    }

    pub fn from_slice_i(ints: &[u8], size: Option<u8>) -> Self {
        let bools = ints.iter().map(|&n| n == 1).collect::<Vec<bool>>();
        Bits::from_slice_b(&bools, size)
    }

    pub fn from_int(value: u32, size: Option<u8>) -> Self {
        let real_size: u8;
        if size.is_none() {
            let size_str = format!("{value:b}");
            real_size = size_str.len() as u8;
        } else {
            real_size = size.unwrap();
        }

        if value >= u32::pow(2, real_size.into()) {
        // if value >= u32::pow(2, real_size as u32) {
            panic!("The given value is bigger than the value that can be represented with {} bits", real_size);
        }

        let data = (0..real_size).rev().map (|n| ((value >> n) & 1) != 0);

        Bits {
            _size: real_size,
            _data: data.collect()
        }
    }

    pub fn to_int(&self) -> u32 {
        self._data
            .iter()
            .rev()
            .enumerate()
            .map(|(place, bit)| u32::from(*bit) << place)
            .sum()
    }
}

impl<Idx> std::ops::Index<Idx> for Bits
where
    Idx: std::slice::SliceIndex<[bool]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self._data[index]
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
}