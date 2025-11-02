#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Donations,
}

#[contract]
pub struct TrustPay;

#[contractimpl]
impl TrustPay {
    pub fn initialize(env: Env) {
        let donations: Map<Symbol, i128> = Map::new(&env);
        env.storage().instance().set(&DataKey::Donations, &donations);
    }

    pub fn donate(env: Env, donor: Address, ngo: Symbol, amount: i128) -> i128 {
        donor.require_auth();
        
        let mut donations: Map<Symbol, i128> = env
            .storage()
            .instance()
            .get(&DataKey::Donations)
            .unwrap_or(Map::new(&env));
        
        let current = donations.get(ngo.clone()).unwrap_or(0);
        let new_total = current + amount;
        donations.set(ngo.clone(), new_total);
        
        env.storage().instance().set(&DataKey::Donations, &donations);
        
        new_total
    }

    pub fn get_total(env: Env, ngo: Symbol) -> i128 {
        let donations: Map<Symbol, i128> = env
            .storage()
            .instance()
            .get(&DataKey::Donations)
            .unwrap_or(Map::new(&env));
        
        donations.get(ngo).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, testutils::Address as _, Env};

    #[test]
    fn test_donation() {
        let env = Env::default();
        let contract_id = env.register(TrustPay, ());
        let client = TrustPayClient::new(&env, &contract_id);

        client.initialize();

        let donor = Address::generate(&env);
        env.mock_all_auths();

        let ngo = symbol_short!("RedCross");
        let total = client.donate(&donor, &ngo, &1000);
        
        assert_eq!(total, 1000);
        assert_eq!(client.get_total(&ngo), 1000);
    }
}
