#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient as TokenAdminClient;

fn setup_test_env<'a>() -> (Env, PesoAnchorEscrowClient<'a>, Address, Address, TokenClient<'a>, TokenAdminClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, PesoAnchorEscrow);
    let escrow_client = PesoAnchorEscrowClient::new(&env, &contract_id);

    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);

    // Register a mock token to represent USDC on Stellar
    let token_admin_id = env.register_stellar_asset_contract(Address::generate(&env));
    let token_client = TokenClient::new(&env, &token_admin_id);
    let token_admin = TokenAdminClient::new(&env, &token_admin_id);

    // Fund the client with mock USDC tokens
    token_admin.mint(&client, &500_i128);

    (env, escrow_client, client, freelancer, token_client, token_admin)
}

#[test]
fn test_happy_path_escrow_lifecycle() {
    let (env, escrow_client, client, freelancer, token_client, _) = setup_test_env();

    // 1. Initialize escrow with 200 USDC tokens
    escrow_client.create_escrow(&client, &freelancer, &token_client.address, &200_i128);
    assert_eq!(token_client.balance(&client), 300_i128);
    assert_eq!(token_client.balance(&escrow_client.address), 200_i128);

    // 2. Release funds to the freelancer
    escrow_client.release();
    assert_eq!(token_client.balance(&freelancer), 200_i128);
    assert_eq!(token_client.balance(&escrow_client.address), 0_i128);
}

#[test]
#[should_panic(expected = "Escrow amount must be positive")]
fn test_edge_case_invalid_amount() {
    let (_, escrow_client, client, freelancer, token_client, _) = setup_test_env();
    
    // Attempting to pass 0 tokens triggers a failure scenario guard rail
    escrow_client.create_escrow(&client, &freelancer, &token_client.address, &0_i128);
}

#[test]
fn test_state_verification() {
    let (env, escrow_client, client, freelancer, token_client, _) = setup_test_env();

    escrow_client.create_escrow(&client, &freelancer, &token_client.address, &150_i128);
    assert_eq!(escrow_client.is_completed(), false);

    escrow_client.release();
    assert_eq!(escrow_client.is_completed(), true);
}

#[test]
#[should_panic(expected = "Funds have already been released")]
fn test_edge_case_duplicate_release_prevention() {
    let (env, escrow_client, client, freelancer, token_client, _) = setup_test_env();

    escrow_client.create_escrow(&client, &freelancer, &token_client.address, &100_i128);
    escrow_client.release();
    
    // A secondary malicious or accidental payout trigger must fail
    escrow_client.release();
}

#[test]
#[should_panic]
fn test_edge_case_unauthorized_action() {
    let (env, escrow_client, client, freelancer, token_client, _) = setup_test_env();
    escrow_client.create_escrow(&client, &freelancer, &token_client.address, &100_i128);

    // Switch context authorizations dynamically to fake an unauthorized malicious caller
    let malicious_attacker = Address::generate(&env);
    env.mock_auths(&[]); // Strip out automatic mocked authorization checks
    
    // This should panic due to missing cryptographic signatures from the actual contract owner client
    escrow_client.release();
}