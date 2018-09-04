extern crate monstersim;

use monstersim::*;
use asset::*;
use account::*;
use rate::*;
use monster::*;
use std::time::{Instant};

fn house_default() -> Account {
    Account(hashmap![
        Asset::LifeTime => Quantity(1000000),
    ])
}

fn monster_default() -> Account {
    Account(hashmap![
        Asset::LifeTime => Quantity(1),
        Asset::BirthCertificate(Instant::now()) => Quantity(1),
        Asset::State(State::Health) => Quantity(10000),
        Asset::State(State::Hunger) => Quantity(10000),
        Asset::State(State::Energy) => Quantity(10000),
        Asset::State(State::Cleanliness) => Quantity(10000),
    ])
}

fn rates_default() -> Vec<Rate> {
    vec![
        Rate {
            credit: hashmap![Asset::LifeTime => Quantity(1)],
            debit: hashmap![
                Asset::State(State::Energy) => Quantity(9),
                Asset::State(State::Hunger) => Quantity(3),
                Asset::State(State::Cleanliness) => Quantity(1),
            ],
        },
        Rate {
            credit: hashmap![Asset::LifeTime => Quantity(1)],
            debit: hashmap![Asset::State(State::Health) => Quantity(1)],
        },
    ]
}

#[test]
fn monster_lifetime_until_death() {

    let house = house_default();
    let rates = rates_default();

    let mut monster = Monster {
        account: monster_default(),
        is_alive: true,
    };

    // while monster.is_alive {
    //     monster.simulate(&rates, &house);
    // }

    // println!("{:#?}", monster.account);
}
