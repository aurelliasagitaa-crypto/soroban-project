#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Symbol};

#[test]
fn test_inventory() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let id = Symbol::short("p1");

    // CREATE
    client.create_product(&id, &String::from_str(&env, "Laptop"));

    // READ
    let product = client.get_product(&id);
    assert_eq!(product, Some(String::from_str(&env, "Laptop")));

    // DELETE
    client.delete_product(&id);
    let deleted = client.get_product(&id);
    assert_eq!(deleted, None);
}