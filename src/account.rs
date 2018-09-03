use asset::*;
use rate::*;
use std::ops;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Account(pub HashMap<Asset, Quantity>);

impl Account {

    pub fn exchange(rate: &Rate, quantity: Quantity, buyer: &Account, seller: &Account) -> (Account, Account) {
        let credit = Account(rate.credit.clone());
        let debit = Account(rate.debit.clone());
        (&(buyer - &debit) + &credit, &(seller - &credit) + &debit)
    }

    pub fn prime(&mut self, rhs: &Account) {
        let Account(lhs) = self;
        let Account(rhs) = rhs;
        for rhs_key in rhs.keys() {
            if !lhs.contains_key(rhs_key) {
                lhs.insert(rhs_key.clone(), Quantity(0));
            }
        }
    }

    pub fn op<F>(lhs: &Account, rhs: &Account, op: F) -> Account
    where
        F: Fn(&Quantity, &Quantity) -> Quantity,
    {
        let mut acc = hashmap![];
        let lhs = &mut lhs.clone();
        let rhs = &mut rhs.clone();
        lhs.prime(rhs);
        rhs.prime(lhs);
        let Account(lhs) = &*lhs;
        let Account(rhs) = &*rhs;
        for key in lhs.keys() {
            let lhs_quantity = lhs.get(key).unwrap();
            let rhs_quantity = rhs.get(key).unwrap();
            let quantity = op(lhs_quantity, rhs_quantity);
            acc.insert(key.clone(), quantity.clone());
        }
        Account(acc.clone())
    }
}

impl PartialEq for Account {
    fn eq(&self, rhs: &Account) -> bool {
        let lhs = &mut self.clone();
        let rhs = &mut rhs.clone();
        lhs.prime(rhs);
        rhs.prime(lhs);
        let Account(lhs) = lhs;
        let Account(rhs) = rhs;
        lhs == rhs
    }
}

impl<'a, 'b> ops::Add<&'a Account> for &'b Account {
    type Output = Account;

    fn add(self, rhs: &Account) -> Account {
        Account::op(self, rhs, |Quantity(lq), Quantity(rq)| Quantity(lq + rq))
    }
}

impl<'a, 'b> ops::Sub<&'a Account> for &'b Account {
    type Output = Account;

    fn sub(self, rhs: &Account) -> Account {
        Account::op(self, rhs, |Quantity(lq), Quantity(rq)| Quantity(lq - rq))
    }
}

