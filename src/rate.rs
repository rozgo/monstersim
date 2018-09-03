use asset::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Rate {
    pub credit: Vec<Asset>,
    pub debit: Vec<Asset>,
}

impl Rate {
    pub fn new(credit: &[Asset], debit: &[Asset]) -> Rate {
        Rate {
            credit: credit.to_vec(),
            debit: debit.to_vec(),
        }
    }
}
