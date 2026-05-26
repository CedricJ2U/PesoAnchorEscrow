#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Client,
    Freelancer,
    Token,
    Amount,
    IsReleased,
}

#[contract]
pub struct PesoAnchorEscrow;

#[contractimpl]
impl PesoAnchorEscrow {
    /// Initializes the escrow agreement, transferring USDC from the client into the contract.
    pub fn create_escrow(
        env: Env,
        client: Address,
        freelancer: Address,
        token: Address,
        amount: i128,
    ) {
        // Ensure client authenticates the transaction
        client.require_auth();

        // Prevent negative or zero amounts
        if amount <= 0 {
            panic!("Escrow amount must be positive");
        }

        // Store state variables inside contract storage
        env.storage().instance().set(&DataKey::Client, &client);
        env.storage().instance().set(&DataKey::Freelancer, &freelancer);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::IsReleased, &false);

        // Transfer funds from client to the smart contract address
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&client, &env.current_contract_address(), &amount);
    }

    /// Releases the held funds to the freelancer. Can only be initiated by the client.
    pub fn release(env: Env) {
        // Retrieve state variables from storage
        let client: Address = env.storage().instance().get(&DataKey::Client).unwrap();
        let freelancer: Address = env.storage().instance().get(&DataKey::Freelancer).unwrap();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();
        let is_released: bool = env.storage().instance().get(&DataKey::IsReleased).unwrap();

        // Enforce safety guards
        if is_released {
            panic!("Funds have already been released");
        }
        
        // Ensure only the client can release funds
        client.require_auth();

        // Mark as released to prevent re-entrancy / double payout
        env.storage().instance().set(&DataKey::IsReleased, &true);

        // Transfer the locked funds out of the contract to the freelancer
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &freelancer, &amount);
    }

    /// Read function to check if the escrow is completed
    pub fn is_completed(env: Env) -> bool {
        env.storage().instance().get(&DataKey::IsReleased).unwrap_or(false)
    }
}

mod test;