#![allow(dead_code)]
use std::collections::HashMap;
use std::path::PrefixComponent;

use borsh::schema::BorshSchemaContainer;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Timestamp};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// Neu co BorshDeserialize va BorshSerialize thi khong the su dung hashmap
// Neu khong co BorshDeserialize va BorshSerialize thi khong the su dung UnorderedMap
pub struct Contract {
  //pub hash: HashMap<String, u64>, khong the su dung hashmap
  pub owner: AccountId,
  pub total_course: u64,
  pub all_course: UnorderedMap<String, Course>,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// nen khai bao Serialize, Deserialize cho 1 struct khi muon su dung struct do
// trong cac function
#[serde(crate = "near_sdk::serde")]
pub struct Course {
  pub instructor: AccountId,
  pub course_id: String,
  pub price: Balance,
  pub created_at: Timestamp,
}

//Create code
//Update code
//Payment => Native tokens
//Payment => Token => Usdt

// Implement contract structure
#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    Self {
      owner: env::signer_account_id(),
      total_course: 0,
      all_course: UnorderedMap::new(b"all_courses".try_to_vec().unwrap()), //signer_account_id la nguoi ky
    }
  }
  pub fn create_course(&mut self, course_id: String, price: Balance) {
    self.total_course += 1;
    let course = Course {
      instructor: env::signer_account_id(),
      course_id: course_id.clone(),
      price,
      created_at: env::block_timestamp_ms(),
    };
    self.all_course.insert(&course_id, &course);
  }

  pub fn update_course(&mut self, course_id: String, price: Balance) {
    let mut course = self.all_course.get(&course_id).unwrap();
    course.price = price;
    self.all_course.insert(&course_id, &course);
  }
  pub fn get_course_by_id(&self, course_id: String) -> Course {
    self.all_course.get(&course_id).unwrap()
  }
}
fn name() {
  let a = env::signer_account_id(); // signer
  let b = env::predecessor_account_id(); // nguoi tien nhiem
  let c = env::account_balance(); // balance account
  let d = env::attached_deposit(); // near deposit
  let e = env::block_timestamp_ms(); // epoch time
}
