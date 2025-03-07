use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC};
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::fungible_token::{FungibleTokenCore, FungibleTokenResolver};
use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue, NearToken, Promise};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    owner_id: AccountId,
}







#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Fungible Token".to_string(),
                symbol: "FT".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }



    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"t".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            owner_id: owner_id.clone(),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        this
    }
}








/// FungibleTokenCore

#[near_bindgen]
impl FungibleTokenCore for Contract {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        assert_ne!(env::predecessor_account_id(), receiver_id, "Self transfers are not allowed");
        self.token.internal_transfer(&env::predecessor_account_id(), &receiver_id, amount.into(), memo);
    }

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_ne!(env::predecessor_account_id(), receiver_id, "Self transfers are not allowed");
        self.token.internal_transfer(&env::predecessor_account_id(), &receiver_id, amount.into(), memo);
        near_sdk::PromiseOrValue::Value(FungibleToken::ft_resolve_transfer(
            &mut self.token,
            env::predecessor_account_id(),
            receiver_id,
            amount,
        ))
    }

    fn ft_total_supply(&self) -> U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.token.ft_balance_of(account_id)
    }
}




/// FungibleTokenResolver

#[near_bindgen]
impl FungibleTokenResolver for Contract {
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        self.token.ft_resolve_transfer(sender_id, receiver_id, amount)
    }
}




/// 📀
/// StorageManagement

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.token.storage_deposit(account_id, registration_only)
    }

    fn storage_withdraw(&mut self, amount: Option<NearToken>) -> StorageBalance {
        self.token.storage_withdraw(amount)
    }

    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        self.token.storage_unregister(force)
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.token.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.token.storage_balance_of(account_id)
    }
}




/// ℹ️
/// FungibleTokenMetadataProvider

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}






// 🏉
// rugfactory

#[near_bindgen]
impl Contract {
    /// Returns the owner of the contract
    pub fn rugfactory_owner_check(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// Deletes the contract and transfers remaining balance to the owner
    pub fn rugfactory_token_delete(&mut self) {
        // Ensure only the owner can call this method
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner can delete the token"
        );

        // Transfer any remaining FT balance to the owner
        let balance = self.token.ft_balance_of(env::current_account_id());
        if balance.0 > 0 {
            self.token.internal_transfer(
                &env::current_account_id(),
                &self.owner_id,
                balance.0,
                None,
            );
        }

        // Delete the account and transfer all remaining NEAR to the owner
        Promise::new(env::current_account_id()).delete_account(self.owner_id.clone());
    }
}


