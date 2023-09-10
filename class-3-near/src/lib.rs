use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap, UnorderedSet};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, Balance, env};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserMetadata{
    pub name: String,
    pub user_id: AccountId,
    pub age: u8,

}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonUser{
  pub user_id: AccountId,
  pub user_metadata: UserMetadata,
  pub courses: Vec<CourseId>, 
  
}   
pub type CourseId = String;


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseMetada{
    pub course_id: CourseId,
    pub content: String,
    pub price: Balance, // u128
    pub students: u32,
    pub students_completed: u32,

}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IdentityContractMetada{
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct ElearningContract{
    pub owner_id: AccountId,
    //co near_bindgen nen khong su dung duoc vec, su dung 
    //UnorderMap.iter() hoac LookupMap(khong co iter())
    // khi du lieu can loc qua thi dung UnorderMap
    pub users: UnorderedSet<AccountId>,
    pub all_users: LookupMap<AccountId, JsonUser>,
    pub all_courses: LookupMap<CourseId, CourseMetada>,
}


#[near_bindgen]
impl ElearningContract{
    #[init]
    pub fn init() -> Self{
        Self::new_identity();
        Self{
            owner_id: env::signer_account_id(), 
            users: UnorderedSet::new(b"user".try_to_vec().unwrap()),
            all_users: LookupMap::new(b"all_users".try_to_vec().unwrap()), 
            all_courses: LookupMap::new(b"all_courses".try_to_vec().unwrap()), 
        }
    }

    #[init]
    pub fn new_identity() -> IdentityContractMetada {
        IdentityContractMetada{
            spec: "elearning-1.0.0".to_string(), 
            name: "Eleaning Bootcamp".to_string(),
            symbol: "ELB".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }

    pub fn create_user(&mut self, name: String, age: u8){
        let user = UserMetadata{
            name,
            user_id: env::signer_account_id(),
            age,
        };
        let json_user = JsonUser{user_id: env::signer_account_id(), user_metadata: user, courses: Vec::new()};
        self.all_users.insert(&env::signer_account_id(), &json_user);
        self.users.insert(&env::signer_account_id());
    } 

    pub fn create_course(&mut self, course_id: CourseId, content: String, price: Balance) {
       let course =  CourseMetada{
            course_id: course_id.clone(),
            content,
            price,
            students_completed: 0,
            students: 0,
        };
        let mut user = self.view_user_by_id(env::signer_account_id()).unwrap();
        user.courses.push(course_id.clone());
        self.all_users.insert(&env::signer_account_id(), &user);
        self.all_courses.insert(&course_id, &course);
    }

    pub fn view_user_by_id(&self, user_id: AccountId) -> Option<JsonUser>{
        if let Some(result) = self.all_users.get(&user_id){
          Some(result)
        }
        else{
          None
        }
    }

    pub fn view_all_users(&self) -> Vec<JsonUser>{
      let mut users = Vec::new();
      for i in self.users.iter(){
        let result = self.view_user_by_id(i).unwrap();
        users.push(result)
      }
      users
    }

    pub fn get_course_by_id(&self, course_id: CourseId) -> Option<CourseMetada>{
        let result =  self.all_courses.get(&course_id);
        if let Some(course) = result{
            Some(course)
        }
        else{
            None
        }
    }

    pub fn get_course_by_id_1(&self, course_id: CourseId) -> CourseMetada{
        self.all_courses.get(&course_id).unwrap()
    }
}
