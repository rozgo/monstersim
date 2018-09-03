extern crate monstersim;

use monstersim::*;
use account::*;
use asset::*;

#[test]
fn accounts_equal_exisiting_assets() {
    let lhs = Account(vec![
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    let rhs = Account(vec![
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(100)),
        Asset::State(State::Hunger, Quantity(10000)),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_equal_missing_assets() {
    let lhs = Account(vec![
        Asset::State(State::Health, Quantity(0)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    let rhs = Account(vec![
        Asset::State(State::Energy, Quantity(0)),
        Asset::State(State::Cleanliness, Quantity(100)),
        Asset::State(State::Hunger, Quantity(10000)),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_not_equal_existing() {
    let lhs = Account(vec![
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    let rhs = Account(vec![
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(10)),
    ]);
    assert!(lhs != rhs);
}

#[test]
fn accounts_sub_existing_assets() {
    let lhs = Account(vec![
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    let rhs = Account(vec![
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Health, Quantity(250)),
        Asset::State(State::Cleanliness, Quantity(200)),
        Asset::State(State::Energy, Quantity(700)),
    ]);
    let res = Account(vec![
        Asset::State(State::Energy, Quantity(100)),
        Asset::State(State::Health, Quantity(250)),
        Asset::State(State::Hunger, Quantity(0)),
        Asset::State(State::Cleanliness, Quantity(-100)),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_sub_missing_assets() {
    let lhs = Account(vec![
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    let rhs = Account(vec![
        Asset::State(State::Health, Quantity(250)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Energy, Quantity(700)),
    ]);
    let res = Account(vec![
        Asset::State(State::Health, Quantity(250)),
        Asset::State(State::Hunger, Quantity(-10000)),
        Asset::State(State::Energy, Quantity(100)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_add_existing_assets() {
    let lhs = Account(vec![
        Asset::State(State::Health, Quantity(250)),
        Asset::State(State::Hunger, Quantity(100)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(400)),
    ]);
    let rhs = Account(vec![
        Asset::State(State::Health, Quantity(250)),
        Asset::State(State::Hunger, Quantity(200)),
        Asset::State(State::Cleanliness, Quantity(200)),
        Asset::State(State::Energy, Quantity(700)),
    ]);
    let res = Account(vec![
        Asset::State(State::Energy, Quantity(1500)),
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Hunger, Quantity(300)),
        Asset::State(State::Cleanliness, Quantity(600)),
    ]);
    assert_eq!(&lhs + &rhs, res);
}

#[test]
fn accounts_add_missing_assets() {
    let lhs = Account(vec![
        Asset::State(State::Health, Quantity(500)),
        Asset::State(State::Energy, Quantity(800)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    let rhs = Account(vec![
        Asset::State(State::Health, Quantity(250)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Energy, Quantity(700)),
    ]);
    let res = Account(vec![
        Asset::State(State::Health, Quantity(750)),
        Asset::State(State::Hunger, Quantity(10000)),
        Asset::State(State::Energy, Quantity(1500)),
        Asset::State(State::Cleanliness, Quantity(100)),
    ]);
    assert_eq!(&lhs + &rhs, res);
}

