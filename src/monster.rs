use account::*;
use asset::*;
use rate::*;
use std::collections::HashMap;

pub struct Monster {
    pub account: Account,
    pub is_alive: bool,
}

impl Monster {
    pub fn simulate(&mut self, rates: &HashMap<Exchange, Rate>, house: &Account) {
        // Every tick monster should be able to purchase 1 LifeTime.
        // First it tries to purchase LifeTime with its State through Exchange::LifeTimeForState.
        // If this fails, it will try to purchase through Exchange::LifeTimeForHealth.
        // If monster cannot purchase any more LifeTime it dies.
        let Quantity(lifetime_before) = self.account.map().get(&Asset::LifeTime).unwrap().clone();
        let exs = [Exchange::LifeTimeForState, Exchange::LifeTimeForHealth];
        if let Some(Tranx::Approved(buyer, _)) = exs.iter().find_map(|ex| {
            match Account::exchange(rates.get(ex).unwrap(), Quantity(1), &self.account, house) {
                Tranx::Denied(_) => None,
                tranx => Some(tranx),
            }
        }) {
            self.account = buyer;
        }
        let Quantity(lifetime_after) = self.account.map().get(&Asset::LifeTime).unwrap().clone();
        if lifetime_after <= lifetime_before {
            self.is_alive = false;
        }
    }
}
