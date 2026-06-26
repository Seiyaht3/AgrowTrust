#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Vault(Address), // Maps farmer address to their specific harvest vault
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarvestVault {
    pub farmer: Address,
    pub supplier: Address,
    pub cooperative_lead: Address,
    pub yield_amount_kg: u32,
    pub is_funded: bool,
    pub is_disbursed: bool,
}

#[contract]
pub struct AgrowTrustContract;

#[contractimpl]
impl AgrowTrustContract {
    /// Initializes a new harvest asset vault for a specific farmer.
    pub fn create_vault(
        e: Env,
        farmer: Address,
        supplier: Address,
        cooperative_lead: Address,
        yield_amount_kg: u32,
    ) {
        // Authenticate the cooperative lead validating this harvest size
        cooperative_lead.require_auth();

        let vault = HarvestVault {
            farmer: farmer.clone(),
            supplier,
            cooperative_lead,
            yield_amount_kg,
            is_funded: true, // Funded upon successful cooperative verification
            is_disbursed: false,
        };

        e.storage().persistent().set(&DataKey::Vault(farmer), &vault);
    }

    /// Simulates the core delivery receipt, moving the state to disbursed.
    pub fn confirm_delivery(e: Env, farmer: Address) {
        // Authenticate that the farmer received the fertilizer
        farmer.require_auth();

        let key = DataKey::Vault(farmer.clone());
        let mut vault: HarvestVault = e.storage().persistent().get(&key).unwrap();
        
        assert!(vault.is_funded, "Vault is not funded");
        assert!(!vault.is_disbursed, "Delivery already completed");

        vault.is_disbursed = true;
        e.storage().persistent().set(&key, &vault);
    }

    /// Fetches details of a specific farmer's vault for verification.
    pub fn get_vault(e: Env, farmer: Address) -> HarvestVault {
        e.storage().persistent().get(&DataKey::Vault(farmer)).unwrap()
    }
}