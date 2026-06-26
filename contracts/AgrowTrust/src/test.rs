#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_happy_path_mvp_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgrowTrustContract);
    let client = AgrowTrustContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let supplier = Address::generate(&env);
    let coop_lead = Address::generate(&env);
    let yield_kg = 500;

    // Test 1: Happy Path - Create Vault and Confirm Delivery
    client.create_vault(&farmer, &supplier, &coop_lead, &yield_kg);
    client.confirm_delivery(&farmer);

    let vault = client.get_vault(&farmer);
    assert!(vault.is_disbursed);
}

#[test]
#[should_panic(expected = "Delivery already completed")]
fn test_edge_case_duplicate_delivery_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgrowTrustContract);
    let client = AgrowTrustContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let supplier = Address::generate(&env);
    let coop_lead = Address::generate(&env);

    client.create_vault(&farmer, &supplier, &coop_lead, &500);
    client.confirm_delivery(&farmer);
    
    // Test 2: Edge Case - Second delivery call triggers expected panic state
    client.confirm_delivery(&farmer);
}

#[test]
fn test_state_verification_matches() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgrowTrustContract);
    let client = AgrowTrustContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let supplier = Address::generate(&env);
    let coop_lead = Address::generate(&env);

    client.create_vault(&farmer, &supplier, &coop_lead, &750);

    // Test 3: State Verification - Confirm state storage structures reflect input params accurately
    let vault = client.get_vault(&farmer);
    assert_eq!(vault.yield_amount_kg, 750);
    assert_eq!(vault.is_funded, true);
    assert_eq!(vault.is_disbursed, false);
}