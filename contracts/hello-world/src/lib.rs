#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Symbol, Map, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    // CREATE PRODUCT
    pub fn create_product(env: Env, id: Symbol, name: String, stock: u32, price: u32) {
        let mut product = Map::new(&env);
        product.set(Symbol::short("name"), name);
        product.set(Symbol::short("stock"), stock);
        product.set(Symbol::short("price"), price);

        env.storage().instance().set(&id, &product);
    }

    // READ PRODUCT
    pub fn get_product(env: Env, id: Symbol) -> Option<Map<Symbol, i128>> {
        env.storage().instance().get(&id)
    }

    // UPDATE STOCK
    pub fn update_stock(env: Env, id: Symbol, new_stock: u32) {
        let mut product: Map<Symbol, i128> =
            env.storage().instance().get(&id).unwrap();

        product.set(Symbol::short("stock"), new_stock);
        env.storage().instance().set(&id, &product);
    }

    // DELETE PRODUCT
    pub fn delete_product(env: Env, id: Symbol) {
        env.storage().instance().remove(&id);
    }
}

mod test;