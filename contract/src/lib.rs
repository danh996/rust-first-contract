/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};
use near_sdk::collections::LookupMap;

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DeCash {
    memo: LookupMap<String, Vec<String>>
}

impl Default for DeCash {
  fn default() -> Self {
    Self {
        memo: LookupMap::new(b"memo".to_vec());
    }
  }
}

#[near_bindgen]
impl DeCash {
    pub fn add_memo(&mut self, memo_text:String, price: String){
        let acccount_id = env::signer_account_id();
        let contains_user = self.memo.contains_key(&acccount_id);

        if contains_user {
            let mut temp_list = match self.memo.get(&acccount_id){
                // Some(x) => println("{:?}", x),
                Some(x) => x,
                None => vec!()
            }

            temp_list.push(memo_text+ " || " + &price + "NEAR");
            self.memo.insert(&acccount_id, &temp_list);
        } else {
            let fresh_vec = vec![memo_text+ " || " + &price + "NEAR"];
            self.memo.insert(&acccount_id, &fresh_vec);
        }
    }

    pub fn transfer_money(@mut self, account_id: AccountId, amount: f64){
        //send account id, but don't know send to where?
        Promise::new(account_id).transfer(amount as u128);
    }

    //view methods
    pub fn get_memos(self, user:String)->Vec<String>{
        match self.memo.get(&user){
            Some(x) => x, //vector contain all your memos ["memo1", "memo2"]
            None => vec![] //ortherwise this will return an empty vector []
        } //don't have a semicolon will auto return this value ;


    }

}
