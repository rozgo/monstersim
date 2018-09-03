use std::ops;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub enum Resource {
    FirstAid,
    Soap,
    Candy,
    EnergyDrink,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Idle,
    Sleep,
    Eat,
    Clean,
    Train,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub enum State {
    Health,
    Hunger,
    Energy,
    Happiness,
    Cleanliness,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct LifeTime();

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Quantity(pub i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub enum Item {
    Resource(Resource),
    State(State),
    LifeTime(LifeTime),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Asset(pub Item, pub Quantity);

impl Asset {
    pub fn op<F>(lhs: &Asset, rhs: &Asset, op: F) -> Option<Asset>
    where
        F: Fn(Quantity, Quantity) -> Quantity,
    {
        match (lhs, rhs) {
            (Asset(lhs_item, lhs_quantity), Asset(rhs_item, rhs_quantity))
                if lhs_item == rhs_item =>
            {
                Some(Asset(
                    lhs_item.clone(),
                    op(lhs_quantity.clone(), rhs_quantity.clone()),
                ))
            }
            _ => None,
        }
    }
}

impl<'a, 'b> ops::Add<&'a Asset> for &'b Asset {
    type Output = Option<Asset>;

    fn add(self, rhs: &Asset) -> Option<Asset> {
        Asset::op(self, rhs, |Quantity(lq), Quantity(rq)| Quantity(lq + rq))
    }
}

impl<'a, 'b> ops::Sub<&'a Asset> for &'b Asset {
    type Output = Option<Asset>;

    fn sub(self, rhs: &Asset) -> Option<Asset> {
        Asset::op(self, rhs, |Quantity(lq), Quantity(rq)| Quantity(lq - rq))
    }
}
