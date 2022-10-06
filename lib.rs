#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    use ink::prelude::borrow::ToOwned;
    use ink::prelude::string::String;

   #[ink(storage)]
    pub struct ERC20 {
       total_supply: Balance,
       balances: Mapping<AccountId, Balance>,
       approval: Mapping<(AccountId, AccountId), Balance>
   }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        sender: AccountId,
        value: Balance,
    }

    impl ERC20 {

        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();

            let sender = Self::env().caller();

            balances.insert(&sender, &total_supply);

            Self::env().emit_event(
                Transfer {
                    from: AccountId::default(),
                    to: sender,
                    value: total_supply,
                }
            );

            Self {
                total_supply,
                balances,
                approval: Default::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> core::result::Result<(), String> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err("Insufficiemt Balance".to_owned());
            }

            self.balances.insert(&from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(&to, &(to_balance + value));

            self.env().emit_event(
                Transfer {
                    from,
                    to,
                    value,
                }
            );

            Ok(())
        }
    }
}
