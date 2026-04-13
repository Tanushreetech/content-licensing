#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Symbol, Env, Address, Map};

#[contract]
pub struct ContentLicenseContract;

#[contractimpl]
impl ContentLicenseContract {

    // Register content with owner and price
    pub fn register_content(env: Env, content_id: Symbol, owner: Address, price: i128) {
        let mut content_map: Map<Symbol, (Address, i128)> =
            env.storage().instance().get(&symbol_short!("CONTENT")).unwrap_or(Map::new(&env));

        content_map.set(content_id.clone(), (owner, price));

        env.storage().instance().set(&symbol_short!("CONTENT"), &content_map);
    }

    // Purchase license
    pub fn buy_license(env: Env, content_id: Symbol, buyer: Address) {
        let content_map: Map<Symbol, (Address, i128)> =
            env.storage().instance().get(&symbol_short!("CONTENT")).unwrap();

        let (owner, price) = content_map.get(content_id.clone()).unwrap();

        // Store license ownership
        let mut license_map: Map<(Symbol, Address), bool> =
            env.storage().instance().get(&symbol_short!("LICENSE")).unwrap_or(Map::new(&env));

        license_map.set((content_id.clone(), buyer.clone()), true);

        env.storage().instance().set(&symbol_short!("LICENSE"), &license_map);

        // (Payment logic would go here - simplified for now)
    }

    // Check if user owns license
    pub fn has_license(env: Env, content_id: Symbol, user: Address) -> bool {
        let license_map: Map<(Symbol, Address), bool> =
            env.storage().instance().get(&symbol_short!("LICENSE")).unwrap_or(Map::new(&env));

        license_map.get((content_id, user)).unwrap_or(false)
    }
}