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
pub enum Asset {
    Resource(Resource, Quantity),
    State(State, Quantity),
    LifeTime(LifeTime, Quantity),
}

impl Asset {
    pub fn op<F>(lhs: &Asset, rhs: &Asset, op: F) -> Option<Asset>
    where
        F: Fn(Quantity, Quantity) -> Quantity,
    {
        match (lhs, rhs) {
            (Asset::Resource(lhs_resource, lhs_quantity), Asset::Resource(rhs_resource, rhs_quantity))
                if lhs_resource == rhs_resource =>
            {
                Some(Asset::Resource(
                    lhs_resource.clone(),
                    op(lhs_quantity.clone(), rhs_quantity.clone()),
                ))
            }
            (Asset::State(lhs_state, lhs_quantity), Asset::State(rhs_state, rhs_quantity))
                if lhs_state == rhs_state =>
            {
                Some(Asset::State(
                    lhs_state.clone(),
                    op(lhs_quantity.clone(), rhs_quantity.clone()),
                ))
            }
            (Asset::LifeTime(_, lhs_quantity), Asset::LifeTime(_, rhs_quantity)) => Some(
                Asset::LifeTime(LifeTime(), op(lhs_quantity.clone(), rhs_quantity.clone())),
            ),
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
