extern crate monstersim;

use account::*;
use asset::*;
use monster::*;
use monstersim::*;
use rate::*;
use std::collections::HashMap;
use std::time::Instant;

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

fn rates_default() -> HashMap<Exchange, Rate> {
    hashmap![
        Exchange::LifeTimeForState =>
        Rate {
            credit: hashmap![Asset::LifeTime => Quantity(1)],
            debit: hashmap![
                Asset::State(State::Energy) => Quantity(9),
                Asset::State(State::Hunger) => Quantity(3),
                Asset::State(State::Cleanliness) => Quantity(1),
            ],
        },
        Exchange::LifeTimeForHealth =>
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

    while monster.is_alive {
        monster.simulate(&rates, &house);
    }

    let Quantity(total_secs) = monster.account.map().get(&Asset::LifeTime).unwrap().clone();

    let hours = (total_secs / 60) / 60;
    let mins = (total_secs - hours * 60 * 60) / 60;
    let secs = total_secs - (hours * 60 * 60 + mins * 60);

    // println!("{:#?}", monster.account);
    println!(
        "RIP! Monster was alive for {} hours, {} minutes and {} seconds.",
        hours, mins, secs
    );
}
