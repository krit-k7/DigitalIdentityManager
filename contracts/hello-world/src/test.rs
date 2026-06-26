#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;

#[test]
fn test_set_identity_creates_new_identity() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, DigitalIdentityManager);
    let client = DigitalIdentityManagerClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let name = String::from_str(&env, "Alice");
    let email = String::from_str(&env, "alice@example.com");
    let data_hash = String::from_str(&env, "QmHash123");

    client.set_identity(&user, &name, &email, &data_hash);

    let identity = client.get_identity(&user).unwrap();
    assert_eq!(identity.name, name);
    assert_eq!(identity.email, email);
    assert_eq!(identity.data_hash, data_hash);
}

#[test]
fn test_set_identity_updates_existing_identity() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, DigitalIdentityManager);
    let client = DigitalIdentityManagerClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // First set (create)
    client.set_identity(
        &user,
        &String::from_str(&env, "Alice"),
        &String::from_str(&env, "alice@old.com"),
        &String::from_str(&env, "QmOldHash"),
    );

    // Second set (update) -- same user, new data
    let new_email = String::from_str(&env, "alice@new.com");
    let new_hash = String::from_str(&env, "QmNewHash");
    client.set_identity(
        &user,
        &String::from_str(&env, "Alice"),
        &new_email,
        &new_hash,
    );

    let identity = client.get_identity(&user).unwrap();
    assert_eq!(identity.email, new_email);
    assert_eq!(identity.data_hash, new_hash);
}

#[test]
fn test_get_identity_returns_none_when_not_set() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, DigitalIdentityManager);
    let client = DigitalIdentityManagerClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // No identity has been set for this user yet
    let identity = client.get_identity(&user);
    assert_eq!(identity, None);
}

#[test]
fn test_delete_identity_removes_it() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, DigitalIdentityManager);
    let client = DigitalIdentityManagerClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    client.set_identity(
        &user,
        &String::from_str(&env, "Alice"),
        &String::from_str(&env, "alice@example.com"),
        &String::from_str(&env, "QmHash123"),
    );

    // Confirm it exists first
    assert!(client.get_identity(&user).is_some());

    client.delete_identity(&user);

    // After deletion, get_identity should return None
    let identity = client.get_identity(&user);
    assert_eq!(identity, None);
}

#[test]
fn test_two_users_have_independent_identities() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, DigitalIdentityManager);
    let client = DigitalIdentityManagerClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.set_identity(
        &user1,
        &String::from_str(&env, "Alice"),
        &String::from_str(&env, "alice@example.com"),
        &String::from_str(&env, "QmHashAlice"),
    );
    client.set_identity(
        &user2,
        &String::from_str(&env, "Bob"),
        &String::from_str(&env, "bob@example.com"),
        &String::from_str(&env, "QmHashBob"),
    );

    let identity1 = client.get_identity(&user1).unwrap();
    let identity2 = client.get_identity(&user2).unwrap();

    assert_eq!(identity1.name, String::from_str(&env, "Alice"));
    assert_eq!(identity2.name, String::from_str(&env, "Bob"));

    // Deleting user1's identity shouldn't touch user2's
    client.delete_identity(&user1);
    assert_eq!(client.get_identity(&user1), None);
    assert!(client.get_identity(&user2).is_some());
}
