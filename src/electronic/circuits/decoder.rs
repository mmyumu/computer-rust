use crate::data::bits::Bits;
use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::and3::And3;

pub struct Decoder2To4 {
    _not0: Not,
    _not1: Not,
    _ands3: Vec<And3>
}

impl Decoder2To4 {
    pub fn new() -> Self {
        let ands3 = (0.. 4).map(|_n|And3::new()).collect::<Vec<And3>>();
        Decoder2To4 {
            _not0: Not::new(),
            _not1: Not::new(),
            _ands3: ands3
        }
    }

    pub fn evaluate(&mut self, _signal_a: &bool, _signal_b: &bool, _enable: bool) -> Vec<bool> {
        let _not0_result = self._not0.evaluate(*_signal_a);
        let _not1_result = self._not1.evaluate(*_signal_b);

        let mut outputs = vec![false; 4];
        outputs[0] = self._ands3[0].evaluate(_not1_result, _not0_result, _enable);
        outputs[1] = self._ands3[1].evaluate(_not1_result, *_signal_a, _enable);
        outputs[2] = self._ands3[2].evaluate(*_signal_b, _not0_result, _enable);
        outputs[3] = self._ands3[3].evaluate(*_signal_b, *_signal_a, _enable);

        outputs.reverse();
        outputs
    }
}

pub struct Decoder {
    _depth: u8,
    _not: Not,
    _ands: Option<Vec<And>>,
    _lower_half_decoder: Option<Box<Decoder>>,
    _upper_half_decoder: Option<Box<Decoder>>,
    _decoder2to4: Option<Decoder2To4>
}

impl Decoder {
    pub fn new(depth: u8) -> Self {
        if depth == 2 {
            Decoder {
                _depth: depth,
                _not: Not::new(),
                _ands: None,
                _lower_half_decoder: None,
                _upper_half_decoder: None,
                _decoder2to4: Some(Decoder2To4::new())
            }
        } else {
            let bits_size = 2_u8.pow(depth as u32);
            let ands = (0.. bits_size).map(|_n|And::new()).collect::<Vec<And>>();

            let lower_half_decoder = Box::new(Decoder::new(depth - 1));
            let upper_half_decoder = Box::new(Decoder::new(depth - 1));

            Decoder {
                _depth: depth,
                _not: Not::new(),
                _ands: Some(ands),
                _lower_half_decoder: Some(lower_half_decoder),
                _upper_half_decoder: Some(upper_half_decoder),
                _decoder2to4: None
            }
        }
        
    }

    pub fn evaluate(&mut self, _inputs: Bits, _enable: bool) -> Vec<bool> {
        if _inputs.len() != self._depth as usize {
            panic!("Expected {} input signals, got {}", self._depth, _inputs.len())
        }

        if self._depth == 2 {
            if let Some(decoder) = self._decoder2to4.as_mut() {
                return decoder.evaluate(&_inputs[1], &_inputs[0], _enable);
            } else {
                panic!("Decoder2To4 is not initialized.");
            }
        }

        let _not_result = self._not.evaluate(_inputs[0]);
        let _lower_result = self._lower_half_decoder.as_mut().unwrap().evaluate(Bits::from_slice_b(&_inputs[1.._inputs.len()], None), _not_result);
        let _upper_result = self._upper_half_decoder.as_mut().unwrap().evaluate(Bits::from_slice_b(&_inputs[1.._inputs.len()], None), _inputs[0]);
        let combined_upper_lower = [_upper_result, _lower_result].concat();
        return combined_upper_lower.iter().enumerate().map(|(i, bit)| self._ands.as_mut().unwrap()[i].evaluate(*bit, _enable)).collect::<Vec<bool>>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_2to4_evaluate_disabled() {
        let mut decoder = Decoder::new(2);

        for a1 in [false, true] {
            for a0 in [false, true] {
                let result = decoder.evaluate(Bits::from_slice_b(&[a1, a0], Some(2)), false);

                for bit in result {
                    assert!(!bit)
                }
            }
        }
    }

    #[test]
    fn decoder_2to4_evaluate_enabled() {
        let mut decoder = Decoder::new(2);

        let mut bit_true_index = 0;
        for a1 in [false, true] {
            for a0 in [false, true] {
                let result = decoder.evaluate(Bits::from_slice_b(&[a1, a0], Some(2)), true);

                for (bit_index, bit) in result.iter().rev().enumerate() {
                    if bit_index == bit_true_index {
                        assert!(bit)
                    } else {
                        assert!(!bit)
                    }
                }
            bit_true_index += 1;
            }
        }
    }

    #[test]
    fn decoder_4to16_evaluate_disabled() {
        let mut decoder = Decoder::new(4);

        for a3 in [false, true] {
            for a2 in [false, true] {
                for a1 in [false, true] {
                    for a0 in [false, true] {
                        let result = decoder.evaluate(Bits::from_slice_b(&[a3, a2, a1, a0], Some(4)), false);

                        for bit in result {
                            assert!(!bit)
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn decoder_4to16_evaluate_enabled() {
        let mut decoder = Decoder::new(4);

        let mut bit_true_index = 0;
        for a3 in [false, true] {
            for a2 in [false, true] {
                for a1 in [false, true] {
                    for a0 in [false, true] {
                        let result = decoder.evaluate(Bits::from_slice_b(&[a3, a2, a1, a0], Some(4)), true);

                        for (bit_index, bit) in result.iter().rev().enumerate() {
                            if bit_index == bit_true_index {
                                assert!(bit)
                            } else {
                                assert!(!bit)
                            }
                        }
                    bit_true_index += 1;
                    }
                }
            }
        }
    }
}