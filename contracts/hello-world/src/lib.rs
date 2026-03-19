#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Address};

// Define identity structure
#[contracttype]
#[derive(Clone)]
pub struct Identity {
    pub name: String,
    pub email: String,
    pub data_hash: String, // could store IPFS hash or encrypted data reference
}

#[contract]
pub struct DigitalIdentityManager;

#[contractimpl]
impl DigitalIdentityManager {

    // Create or update identity
    pub fn set_identity(env: Env, user: Address, name: String, email: String, data_hash: String) {
        user.require_auth();

        let identity = Identity {
            name,
            email,
            data_hash,
        };

        env.storage().instance().set(&user, &identity);
    }

    // Fetch identity
    pub fn get_identity(env: Env, user: Address) -> Option<Identity> {
        env.storage().instance().get(&user)
    }

    // Delete identity
    pub fn delete_identity(env: Env, user: Address) {
        user.require_auth();
        env.storage().instance().remove(&user);
    }
}
