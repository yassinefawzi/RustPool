#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{ Ord, Ordering };

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
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

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = if s.starts_with("AB") {
            (&s[..2], &s[2..])
        } else {
            (&s[..1], &s[1..])
        };
        Ok(BloodType {
            antigen: antigen_str.parse()?,
            rh_factor: rh_str.parse()?,
        })
    }
}

use std::fmt::{ self, Debug };

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let rh_ok = match (self.rh_factor.clone(), other.rh_factor.clone()) {
            (RhFactor::Negative, RhFactor::Positive) => false,
            _ => true,
        };
        let antigen_ok = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
        };

        rh_ok && antigen_ok
    }
    pub fn donors(&self) -> Vec<Self> {
        let all = vec![
            "O+".parse().unwrap(),
            "O-".parse().unwrap(),
            "A+".parse().unwrap(),
            "A-".parse().unwrap(),
            "B+".parse().unwrap(),
            "B-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "AB-".parse().unwrap()
        ];
        all.into_iter()
            .filter(|b| self.can_receive_from(b))
            .collect()
    }
    pub fn recipients(&self) -> Vec<Self> {
        let all = [
            "O+".parse::<BloodType>().unwrap(),
            "O-".parse::<BloodType>().unwrap(),
            "A+".parse::<BloodType>().unwrap(),
            "A-".parse::<BloodType>().unwrap(),
            "B+".parse::<BloodType>().unwrap(),
            "B-".parse::<BloodType>().unwrap(),
            "AB+".parse::<BloodType>().unwrap(),
            "AB-".parse::<BloodType>().unwrap(),
        ];
        all.into_iter()
            .filter(|b| b.can_receive_from(self))
            .collect()
    }
}
