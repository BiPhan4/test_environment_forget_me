use std::io;
use std::collections::BTreeMap;

use cosmwasm_std::{
    debug_print, to_binary, Api, Env, Extern, HandleResponse, HumanAddr, Querier, StdError,
    StdResult, Storage, testing::MockStorage, coin,
};
use cosmwasm_storage::{singleton, bucket, bucket_read};
use schemars::JsonSchema;
use secret_toolkit::storage;
use serde::{Serialize, Deserialize};

static FILE_LOCATION: &[u8] = b"FILES";
static WALLET_INFO_LOCATION: &[u8] = b"WALLET_INFO";

//To do: forget_me will load their wallet info, update the key (could just do **** - don't matter b/c it gets converted to byte form anyway, 
fn main() {
    let mut store = MockStorage::new();
    let wallet_info = WalletInfo{init: true, counter: String::from("0")};

    let sender = String::from("Bi");
    let borrowed_sender = &sender;

    let bucket_response =
    bucket(WALLET_INFO_LOCATION, &mut store).save(borrowed_sender.as_bytes(), &wallet_info);
    match bucket_response {
        Ok(bucket_response) => bucket_response,
        Err(e) => panic!("Bucket Error: {}", e),
    }

    let counter = get_counter(&mut store, borrowed_sender.to_string());
    bucket_save_file(&mut store, &String::from("home/pepe"), "Angle".to_string(), counter.as_bytes());

    let load_bucket: Result<String, StdError> =
    bucket_read(counter.as_bytes(), &mut store).load(&String::from("home/pepe").as_bytes());
    let loaded_wallet_info = load_bucket.unwrap();
    println!("{}", loaded_wallet_info);

    let behalf = HumanAddr::from("Bi");
    let borrowed_behalf = &behalf;
    let behalf_as_string = borrowed_behalf.to_string();
    let behalf_as_slice = behalf_as_string.as_bytes();
    let behalf_as_str = borrowed_behalf.as_str();
    let behalf_str_to_slice = behalf_as_str.as_bytes();

    let counter = 0; 
    let increased_counter = counter + 1;
    let increased_counter = increased_counter + 1;
    let format_counter = format!("{}", increased_counter);

    let address = String::from("Bi");
    let namespace = format!("{}{}", address, increased_counter);
    
    println!("{}", namespace);
    let as_bytes_namespace = namespace.as_bytes();
    println!("{:#?}", as_bytes_namespace);
    

}

pub fn get_counter<'a, S: Storage>(store: &'a mut S, sender: String) -> String {
    let load_bucket: Result<WalletInfo, StdError> =
    bucket_read(WALLET_INFO_LOCATION, store).load(sender.as_bytes());
    let loaded_wallet_info = load_bucket.unwrap();
    loaded_wallet_info.counter
    //could also just make this return the slice if needed
}

pub fn bucket_save_file<'a, S: Storage>(store: &'a mut S, path: &String, folder: String, namespace: &[u8]) {

    let bucket_response = bucket(namespace, store).save(path.as_bytes(), &folder);
    match bucket_response {
        Ok(bucket_response) => bucket_response,
        Err(e) => panic!("Bucket Error: {}", e),
    }
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug, Clone)]
pub struct WalletInfo<> {
    init: bool,
    pub counter: String
}
// let mut wallet_info = WalletInfo{ init: true, counter: b"0" };

// let mut store = MockStorage::new();
// let wal_info = singleton(&mut store, WALLET_INFO_LOCATION);
// wal_info.save(&wallet_info);

// let vector = vec![FILE_LOCATION, wallet_info.counter];

// let new_namespace = [wallet_info.counter, FILE_LOCATION].concat();
// println!("{:#?}", new_namespace);
// println!("{:#?}", FILE_LOCATION);

// pub fn save_wallet_info<'a, S: Storage>(store: &'a mut S, adr: String, namespace: &[u8]) {
    //     let bucket_response =
    //                 bucket(namespace, store).save(adr.as_bytes(), &wallet_info { init: val, counter: val });
    //             match bucket_response {
    //                 Ok(bucket_response) => bucket_response,
    //                 Err(e) => panic!("Bucket Error: {}", e),
    //             }
    // }

// let mut map = BTreeMap::<&String, &String>::new();
// let a = String::from("anyone/test");
// let b = String::from("anyone/pepe");

// let borrow_a = &a;
// let borrow_b = &b;

// map.insert(borrow_a,borrow_a);
// map.insert(borrow_b,borrow_b);

// for (key, value) in map.iter() {
//     println!("{key}: {value}");
//     let string = value.to_string();
//     println!("{}", string);
// }

// fn main() {
//     let mut input_text = String::new();
//     io::stdin()
//         .read_line(&mut input_text)
//         .expect("failed to read from stdin");
//     let trimmed = input_text.trim();
//     match trimmed.parse::<i32>(){
//         Ok(i) => evaluate_wallet(i),
//         Err(..) => println!("bad int: {}", trimmed),
//     }


// }

// fn evaluate_wallet(wallet_balance: i32) {
//     match wallet_balance{
//         0 => println!("Empty!"),
//         _ => println!("Invalid balance"),
//     }
// }