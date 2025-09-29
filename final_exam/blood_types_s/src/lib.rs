#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let r_ok = match (self.rh_factor, other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };
        if !r_ok {
            return false;
        }
        match self.antigen {
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
            Antigen::O => matches!(other.antigen, Antigen::O),
        }
    }

    fn types() -> Vec<Self> {
        let mut result = Vec::new();
        for antigen in [Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for rh in [RhFactor::Positive, RhFactor::Negative] {
                result.push(BloodType {
                    antigen,
                    rh_factor: rh,
                });
            }
        }
        result
    }

    pub fn donors(self) -> Vec<Self> {
        Self::types()
            .into_iter()
            .filter(|bt| self.can_receive_from(*bt))
            .collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        Self::types()
            .into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }
}
