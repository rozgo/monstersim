extern crate monstersim;

use account::*;
use asset::*;
use monstersim::*;
use rate::*;
use std::collections::HashMap;

fn house_default() -> Account {
    Account(hashmap![
        Asset::LifeTime => Quantity(1000000),
    ])
}

fn monster_default() -> Account {
    Account(hashmap![
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
    ]
}

#[test]
fn rate_buy_lifetime() {
    let house = house_default();
    let monster = monster_default();
    let rates = rates_default();
    let rate = rates.get(&Exchange::LifeTimeForState).unwrap();

    let res_seller = Account(hashmap![
        Asset::LifeTime => Quantity(999999),
        Asset::State(State::Hunger) => Quantity(3),
        Asset::State(State::Energy) => Quantity(9),
        Asset::State(State::Cleanliness) => Quantity(1),
    ]);

    let res_buyer = Account(hashmap![
        Asset::LifeTime => Quantity(1),
        Asset::State(State::Health) => Quantity(10000),
        Asset::State(State::Hunger) => Quantity(9997),
        Asset::State(State::Energy) => Quantity(9991),
        Asset::State(State::Cleanliness) => Quantity(9999),
    ]);

    match Account::exchange(rate, Quantity(1), &monster, &house) {
        Tranx::Approved(buyer, seller) => {
            assert_eq!(res_seller, seller);
            assert_eq!(res_buyer, buyer);
        }
        _ => assert!(false),
    }
}

#[test]
fn rate_buy_lifetime_quantity() {
    let house = house_default();
    let monster = monster_default();
    let rates = rates_default();
    let rate = rates.get(&Exchange::LifeTimeForState).unwrap();

    let res_seller = Account(hashmap![
        Asset::LifeTime => Quantity(999998),
        Asset::State(State::Hunger) => Quantity(6),
        Asset::State(State::Energy) => Quantity(18),
        Asset::State(State::Cleanliness) => Quantity(2),
    ]);

    let res_buyer = Account(hashmap![
        Asset::LifeTime => Quantity(2),
        Asset::State(State::Health) => Quantity(10000),
        Asset::State(State::Hunger) => Quantity(9994),
        Asset::State(State::Energy) => Quantity(9982),
        Asset::State(State::Cleanliness) => Quantity(9998),
    ]);

    match Account::exchange(rate, Quantity(2), &monster, &house) {
        Tranx::Approved(buyer, seller) => {
            assert_eq!(res_seller, seller);
            assert_eq!(res_buyer, buyer);
        }
        _ => assert!(false),
    }
}
