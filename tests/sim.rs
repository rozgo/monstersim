extern crate monstersim;

// use account::*;
// use asset::*;
// use monstersim::*;
// use rate::*;

// fn house_default() -> Account {
//     Account(vec![
//         Asset::LifeTime(LifeTime(), Quantity(i32::max_value())),
//         Asset::LifeTime(LifeTime(), Quantity(i32::max_value())),
//     ])
// }

// fn monster_default() -> Account {
//     Account(vec![
//         Asset::State(State::Health, Quantity(10000)),
//         Asset::State(State::Hunger, Quantity(10000)),
//         Asset::State(State::Energy, Quantity(10000)),
//         Asset::State(State::Cleanliness, Quantity(10000)),
//     ])
// }

// fn rates_default() -> Vec<Rate> {
//     vec![
//         Rate {
//             credit: vec![Asset::LifeTime(LifeTime(), Quantity(1))],
//             debit: vec![
//                 Asset::State(State::Energy, Quantity(9)),
//                 Asset::State(State::Hunger, Quantity(3)),
//                 Asset::State(State::Cleanliness, Quantity(1)),
//             ],
//         },
//         Rate {
//             credit: vec![Asset::LifeTime(LifeTime(), Quantity(1))],
//             debit: vec![Asset::State(State::Health, Quantity(1))],
//         },
//         Rate {
//             credit: vec![Asset::State(State::Health, Quantity(100))],
//             debit: vec![Asset::Resource(Resource::FirstAid, Quantity(1))],
//         },
//         Rate {
//             credit: vec![Asset::State(State::Hunger, Quantity(100))],
//             debit: vec![Asset::Resource(Resource::Candy, Quantity(1))],
//         },
//         Rate {
//             credit: vec![Asset::State(State::Energy, Quantity(100))],
//             debit: vec![Asset::Resource(Resource::EnergyDrink, Quantity(1))],
//         },
//         Rate {
//             credit: vec![Asset::State(State::Cleanliness, Quantity(100))],
//             debit: vec![Asset::Resource(Resource::Soap, Quantity(1))],
//         },
//     ]
// }
