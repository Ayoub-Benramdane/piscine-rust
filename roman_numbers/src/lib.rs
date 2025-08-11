#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Unsupported single RomanDigit value"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut number: u32) -> Self {
        if number == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mapping: &[(u32, &[RomanDigit])] = &[
            (1000, &[RomanDigit::M]),
            (900, &[RomanDigit::C, RomanDigit::M]),
            (500, &[RomanDigit::D]),
            (400, &[RomanDigit::C, RomanDigit::D]),
            (100, &[RomanDigit::C]),
            (90, &[RomanDigit::X, RomanDigit::C]),
            (50, &[RomanDigit::L]),
            (40, &[RomanDigit::X, RomanDigit::L]),
            (10, &[RomanDigit::X]),
            (9, &[RomanDigit::I, RomanDigit::X]),
            (5, &[RomanDigit::V]),
            (4, &[RomanDigit::I, RomanDigit::V]),
            (1, &[RomanDigit::I]),
        ];

        let mut result = Vec::new();

        for &(value, digits) in mapping {
            while number >= value {
                result.extend_from_slice(digits);
                number -= value;
            }
        }

        RomanNumber(result)
    }
}