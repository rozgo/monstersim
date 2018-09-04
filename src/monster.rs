use asset::*;
use account::*;
use rate::*;

pub struct Monster {
    pub account: Account,
    pub is_alive: bool,
}

impl Monster {
    pub fn simulate(&mut self, rates: &Vec<Rate>, house: &Account) {
        let Quantity(lifetime_before) = self.account.map().get(&Asset::LifeTime).unwrap().clone();
        for rate in rates {
            let (buyer, _) = Account::exchange(rate, Quantity(1), &self.account, house);
            self.account = buyer;
        }
        let Quantity(lifetime_after) = self.account.map().get(&Asset::LifeTime).unwrap().clone();
        if lifetime_after <= lifetime_before {
            self.is_alive = false;
        }
    }
}

