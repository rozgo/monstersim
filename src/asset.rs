
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
pub enum State {
    Health,
    Hunger,
    Energy,
    Happiness,
    Cleanliness,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
pub struct LifeTime();

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
pub enum Asset {
    Resource(Resource),
    State(State),
    LifeTime(LifeTime),
}
