#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[derive(Debug, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    BalanceTooLow,
    AllowanceTooLow,
}

type Result<T> = core::result::Result<T, Error>;

#[ink::trait_definition]
pub trait TERC20 {

    #[ink(message)]
    pub fn balance_of(&self, who: AccountId) -> Balance;

    #[ink(message)]
    fn total_supply(&self) -> Balance;

    #[ink(message)]
    fn approve(&self, to: AccountId, value: Balance) -> Result<()>;

    #[ink(message)]
    fn transfer_from();
}
