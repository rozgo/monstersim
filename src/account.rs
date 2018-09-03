use asset::*;
use rate::*;
use std::cmp;
use std::ops;

#[derive(Debug, Clone)]
pub struct Account(pub Vec<Asset>);

impl Account {
    pub fn exchange(rate: &Rate, buyer: &Account, seller: &Account) -> (Account, Account) {
        let b = buyer.clone();
        let s = seller.clone();
        (b, s)
    }

    pub fn prime(&mut self, rhs: &Account) {
        let Account(rhs) = rhs;
        for rhs_asset in rhs {
            let mut found_asset = None;
            {
                let Account(lhs) = &*self;
                for lhs_asset in lhs {
                    if let Some(asset) = lhs_asset + rhs_asset {
                        found_asset = Some(asset);
                        continue;
                    }
                }
            }
            let Account(acc) = self;
            match found_asset {
                Some(_) => (),
                _ => acc.push(Asset::op(rhs_asset, rhs_asset, |_, _| Quantity(0)).unwrap()),
            };
        }
    }

    pub fn op<F>(lhs: &Account, rhs: &Account, op: F) -> Account
    where
        F: Fn(&Asset, &Asset) -> Option<Asset>,
    {
        let acc = &mut vec![];
        let lhs = &mut lhs.clone();
        let rhs = &mut rhs.clone();
        lhs.prime(rhs);
        rhs.prime(lhs);
        let Account(lhs) = &*lhs;
        let Account(rhs) = &*rhs;
        for lhs_asset in lhs {
            for rhs_asset in rhs {
                let diff_asset = op(lhs_asset, rhs_asset);
                if let Some(diff_asset) = diff_asset {
                    acc.push(diff_asset)
                }
            }
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
        lhs.sort_by(|ref a, ref b| a.partial_cmp(&b).unwrap());
        rhs.sort_by(|ref a, ref b| a.partial_cmp(&b).unwrap());
        lhs == rhs
    }
}

impl<'a, 'b> ops::Add<&'a Account> for &'b Account {
    type Output = Account;

    fn add(self, rhs: &Account) -> Account {
        Account::op(self, rhs, |la, ra| la + ra)
    }
}

impl<'a, 'b> ops::Sub<&'a Account> for &'b Account {
    type Output = Account;

    fn sub(self, rhs: &Account) -> Account {
        Account::op(self, rhs, |la, ra| la - ra)
    }
}
