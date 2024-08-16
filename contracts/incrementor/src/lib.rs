#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

// Our contract data keys
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    /// increment an internal counter; return the new value.
    pub fn increment(env: Env) -> u32 {

        // contract data access
        // We use env.storage to access and update contract data
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;

        // lets log the new value of count
        log!(&env, "count: {}", count);

        // store the value in instance storage
        env.storage().instance().set(&COUNTER, &count);

        // increase the TTL / time extension of the stored value
        env.storage().instance().extend_ttl(100, 100);

        // return count
        count
    }
}

mod test;