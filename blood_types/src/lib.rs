#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    B,
    AB,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim_end_matches(&['+', '-'][..]) {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err("Invalid antigen type"),
        }
    }
}

impl FromStr for RhFactor {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.chars().last() {
            Some('+') => Ok(RhFactor::Positive),
            Some('-') => Ok(RhFactor::Negative),
            _ => Err("Invalid Rh factor"),
        }
    }
}

impl FromStr for BloodType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen = Antigen::from_str(s)?;
        let rh_factor = RhFactor::from_str(s)?;
        Ok(BloodType { antigen, rh_factor })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

use std::fmt;

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh_symbol = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen_str, rh_symbol)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_compatible = match (&self.antigen, &other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };

        let rh_compatible = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };

        antigen_compatible && rh_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_blood_types = [
            "O+", "A+", "B+", "AB+", "O-", "A-", "B-", "AB-"
        ];

        let mut valid_donors = vec![];

        for bt_str in all_blood_types.iter() {
            if let Ok(candidate) = BloodType::from_str(bt_str) {
                if self.can_receive_from(&candidate) {
                    valid_donors.push(candidate);
                }
            }
        }

        valid_donors.sort();
        valid_donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all_blood_types = [
            "O+", "A+", "B+", "AB+", "O-", "A-", "B-", "AB-"
        ];

        let mut receivers = vec![];

        for bt_str in all_blood_types.iter() {
            if let Ok(candidate) = BloodType::from_str(bt_str) {
                if candidate.can_receive_from(self) {
                    receivers.push(candidate);
                }
            }
        }

        receivers.sort();
        receivers
    }
}
