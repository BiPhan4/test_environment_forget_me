use std::{io, string};
use std::collections::BTreeMap;


use cosmwasm_std::{
    debug_print, to_binary, Api, Env, Extern, HandleResponse, HumanAddr, Querier, StdError,
    StdResult, Storage, testing::{MockStorage, self}, coin,
};
use cosmwasm_storage::{singleton, bucket, bucket_read};
use schemars::JsonSchema;
use secret_toolkit::storage;
use serde::{Serialize, Deserialize};

static WALLET_INFO_LOCATION: &[u8] = b"WALLET_INFO";

pub fn get_namespace<'a, S: Storage>(store: &'a S, sender: &String) -> StdResult<String> {

    let loaded_wallet: Result<WalletInfo, StdError> = bucket_read(WALLET_INFO_LOCATION, store).load(sender.as_bytes());
    let unwrapped_wallet = loaded_wallet?;
    Ok(unwrapped_wallet.namespace) 
}

pub fn get_counter<'a, S: Storage>(store: &'a S, sender: &String) -> StdResult<i32> {

    let loaded_wallet: Result<WalletInfo, StdError> = bucket_read(WALLET_INFO_LOCATION, store).load(sender.as_bytes());
    let unwrapped_wallet = loaded_wallet?;
    Ok(unwrapped_wallet.counter) 
}

pub fn return_wallet(x: Option<WalletInfo>) -> WalletInfo {
    match x {
        Some(i) => i,//if exists, their wallet init could be false or true, and their namespace is present, 
        //If none, it means the user has never called init before, so we return a wallet info that can be altered and saved right away
        None => WalletInfo { init: false, namespace: "empty".to_string(), counter: 0 }, 
        
    }
}

pub fn bucket_save_file<'a, S: Storage>(store: &'a mut S, path: &String, folder: String, namespace: &String) {
    let bucket_response = bucket(namespace.as_bytes(), store).save(path.as_bytes(), &folder);
    match bucket_response {
        Ok(bucket_response) => bucket_response,
        Err(e) => panic!("Bucket Error: {}", e),
    }
}

pub fn bucket_load_file<'a, S: Storage>(store: &'a mut S, path: &String, namespace: &String) -> String {
    bucket(namespace.as_bytes(), store).load(path.as_bytes()).unwrap_or(String::from("file does not exist!"))
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug, Clone)]
pub struct WalletInfo<> {
    pub init: bool,//init declared as public just so that main can access it. In storage contract, WalletInfo struct is in backendfile where all relevant code is
    pub namespace: String,
    pub counter: i32
}

//this function is just used for testing purposes
pub fn get_wallet_info<'a, S: Storage>(store: &'a mut S, sender: &String) -> WalletInfo {
    let loaded_wallet: Result<Option<WalletInfo>, StdError> = bucket(WALLET_INFO_LOCATION, store).may_load(sender.as_bytes());
    let unwrapped_wallet = loaded_wallet.unwrap();
    let matched_wallet = return_wallet(unwrapped_wallet);
    matched_wallet
}


pub fn file_exists<'a, S: Storage>(store: &'a mut S, path: &String, namespace: &String) -> bool {
    let f: Result<String, StdError> = bucket(namespace.as_bytes(), store).load(path.as_bytes());
    match f {
        Ok(_v) => true,
        Err(_e) => false,
    }
}

pub fn query_file<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    path: String,
    behalf: &HumanAddr,
) -> StdResult<String> {
    //surgically take it from path...and also take the counter too
    
    let full_namespace = get_namespace_from_path(&deps, path.clone()).unwrap_or(String::from("namespace not found!"));

    let f = bucket_load_readonly_file(&deps.storage, &path, &full_namespace); //need a namespace
    
    match f {
        Ok(f1) => {Ok(f1)} 
        Err(_err) => {
            let error_message = String::from("Error querying file.");
            Err(StdError::generic_err(error_message))
        }
    }
}

pub fn try_change_owner<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    path: String,
    new_owner: String,
) -> StdResult<HandleResponse> {
    let signer = deps
        .api
        .human_address(&deps.api.canonical_address(&env.message.sender)?)?;

    //if alice now wants to give ownership of the file back to anyone, she would have to pass in the namespace of anyone
    //the only way to get the namespace of the file owner, is from the passed in path 
    let full_namespace = get_namespace_from_path(deps, path.clone()).unwrap_or(String::from("namespace not found!"));

    let mut f = bucket_load_file(&mut deps.storage, &path, &full_namespace); //don't think I need this 
    let f = new_owner;

        bucket_save_file(&mut deps.storage, &path, f, &full_namespace);

    Ok(HandleResponse::default())
}

pub fn get_namespace_from_path<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    path: String,
) -> StdResult<String> {

    let split = path.split('/');
    let vec = split.collect::<Vec<&str>>();
    let namespace_owner = vec[0].to_string();
    let counter = get_counter(&deps.storage, &namespace_owner)?.to_string();
    let full_namespace = format!("{}{}", namespace_owner, counter);
    Ok(full_namespace)

}

pub fn bucket_load_readonly_file<'a, S: Storage>(
    store: &'a S,
    path: &String,
    namespace: &String
) -> Result<String, StdError> {
    bucket_read(namespace.as_bytes(), store).load(path.as_bytes())
}