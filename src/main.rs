use std::{io, string};
use std::collections::BTreeMap;
use hello_cargo::backend::{self, WalletInfo, get_namespace, bucket_save_file, bucket_load_file, return_wallet, get_wallet_info, file_exists, query_file, try_change_owner};
use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{coins, from_binary};

use cosmwasm_std::{
    debug_print, to_binary, Api, Env, Extern, HandleResponse, HumanAddr, Querier, StdError,
    StdResult, Storage, testing::{MockStorage, self}, coin, 
};
use cosmwasm_storage::{singleton, bucket, bucket_read};
use schemars::JsonSchema;
use secret_toolkit::storage;
use serde::{Serialize, Deserialize};

static WALLET_INFO_LOCATION: &[u8] = b"WALLET_INFO";

fn main() {
    let mut deps = mock_dependencies(20, &coins(2, "token"));
    let env = mock_env(String::from("BiPhan"), &[]); 
    let borrowed_sender = &env.message.sender.to_string(); 
    //borrow so we can use it everywhere in main()
    //In storage contract, we wouldn't need to use borrow too often, but inside main(), we are going to be using the same variable multiple times so
    //this is only possible if we borrow that variable, i.e., use a reference. Below, we also borrow other variables as needed
    
    //let's say a user is currently using storage, and they've yet to call forget_me. Their walletinfo would have
    //the following:

    let Bi_namespace = format!("{}{}", borrowed_sender, 0); //namespace starts with address0
    let wallet_info = WalletInfo{init: true, namespace: Bi_namespace, counter: 0};

    let bucket_response =
    bucket(WALLET_INFO_LOCATION, &mut deps.storage).save(borrowed_sender.as_bytes(), &wallet_info);
    match bucket_response {
        Ok(bucket_response) => bucket_response,
        Err(e) => panic!("Bucket Error: {}", e),
    }

    //let's save a file for our user. We need to get their namespace, then call bucket_save_file. We will force all our handles and queries to get the user's current namespace.
    let Bi_namespace_0 = get_namespace(&mut deps.storage, borrowed_sender).unwrap_or("namespace does not exist!".to_string());
    println!("Bi's initial wallet info is \n{:#?}", get_wallet_info(&mut deps.storage, borrowed_sender));
    println!("Bi's initial namespace is {}\n", Bi_namespace_0);
    
    bucket_save_file(&mut deps.storage, &String::from("BiPhan/"), String::from("mock folder/file"), &Bi_namespace_0);
    
    //now let's load and print the file
    let loaded_file = bucket_load_file(&mut deps.storage,&String::from("BiPhan/"),&Bi_namespace_0);
    //should return a String called "mock folder/file"
    println!("The loaded file is {}", loaded_file);
    //We can also test our query function. We are not using behalf in simulation because we are simulating using a String, not a file
    //This simulation could also have been done with a file that just had contents and owner
    let queried_file = query_file(&deps, String::from("BiPhan/"), &HumanAddr::from("Alice")).unwrap_or(String::from("file does not exist!"));
    println!("The queried file is {:#?}\n", queried_file);
    
    let _forgotten = try_forget_me(&mut deps, env).unwrap_or(HandleResponse { ..Default::default() }); //should be handling errors better than this
    
    //so now Bi's walletinfo has init: false, namespace: BiPhan1, counter: 1 

    let bi_namespace_after_forget_me = get_namespace(&mut deps.storage, borrowed_sender).unwrap_or("namespace does not exist!".to_string());
    let borrowed_bi_namespace_after_forget_me = &bi_namespace_after_forget_me;
    println!("Bi's wallet info after forget_me is\n {:#?}", get_wallet_info(&mut deps.storage, borrowed_sender));
    println!("Bi's namespace after forget_me is: {}\n", bi_namespace_after_forget_me);
    
    //now let's try to load and print the file using the new namespace.This should return error message "file does not exist"
    let loaded_file = bucket_load_file(&mut deps.storage,&String::from("BiPhan/"),&borrowed_bi_namespace_after_forget_me);
    println!("{}\n", loaded_file);

    //now suppose user calls try_init after previously having called forget_me
    
    let mut path = borrowed_sender.to_string();
    path.push('/');
    
    let already_init = file_exists(&mut deps.storage, &path, &borrowed_bi_namespace_after_forget_me);

    match already_init {
        false => {
            //create_file(&mut deps.storage, env.message.sender.to_string(), path.clone(), contents); - createfile would involve importing too much code, let's just make a String and save it
            bucket_save_file(&mut deps.storage, &path, String::from("Hasbullah wrestles bear"), &borrowed_bi_namespace_after_forget_me);
            //   One of two things could happen:
            //a) they already have a wallet info saved, so we just pull it out and set init to true
            //b) they don't have a wallet info saved, so may_load will return None, which prompts a return of a default walletinfo that can be altered and saved asap
            
            let loaded_wallet: Result<Option<WalletInfo>, StdError> = bucket(WALLET_INFO_LOCATION, &mut deps.storage).may_load(borrowed_sender.as_bytes());
            let unwrapped_wallet = loaded_wallet.unwrap();//in storage contract, try_init returns Result, so we can use ? instead of the dangerous unwrap()
            let mut returned_wallet = return_wallet(unwrapped_wallet);
            
            if returned_wallet.namespace == "empty".to_string() {
                returned_wallet.init = true;
                let new_namespace = format!("{}{}", borrowed_sender, 0);
                returned_wallet.namespace = new_namespace;
            } else {
                returned_wallet.init = true;
            }
            
            let bucket_response =
            bucket(WALLET_INFO_LOCATION, &mut deps.storage).save(borrowed_sender.as_bytes(), &returned_wallet);
            match bucket_response {
                Ok(bucket_response) => bucket_response,
                Err(e) => panic!("Bucket Error: {}", e),
            }
        }
        true => {
            let error_message = format!("User has already been initiated");
            //Err(StdError::generic_err(error_message)); 
            //main() is not built to return StdResult, so we can't return an Err in this test environment - totally doable in storage contract though
            println!("{}", error_message);
        }
    }

    println!("Bi's wallet info after simulated try_init is\n{:#?}", get_wallet_info(&mut deps.storage, borrowed_sender));
    //now let's try to load and print the new saved file using the new namespace. This should succeed.
    let loaded_file = bucket_load_file(&mut deps.storage,&String::from("BiPhan/"),&borrowed_bi_namespace_after_forget_me);
    println!("{}", loaded_file);
    //We are not using behalf in this simulation because we are not testing with a file
    let queried_file = query_file(&deps, String::from("BiPhan/"), &HumanAddr::from("Alice")).unwrap_or(String::from("file does not exist!"));
    println!("The queried file is {:#?}\n", queried_file);

    //to be sure, if we use same path as above but with old namespace to load, we will get "mockfolder/file"
    let loaded_file = bucket_load_file(&mut deps.storage,&String::from("BiPhan/"),&Bi_namespace_0);
    println!("The loaded file is {}", loaded_file);
    //query_file is written to only query BiPhan's most updated namespace. There is no way to call GetContents (query_file) on the old namespace

    //Lets test for when user is calling try_init for the very first time. Passing in Nuggie as key will return None, which will return a wallet info with "empty" as the namespace
    let loaded_wallet: Result<Option<WalletInfo>, StdError> = bucket(WALLET_INFO_LOCATION, &mut deps.storage).may_load(String::from("Nuggie").as_bytes());
    let unwrapped_wallet = loaded_wallet.unwrap();//in storage contract, try_init returns Result, so we can use ? instead of the dangerous unwrap()
    let mut returned_wallet = return_wallet(unwrapped_wallet);
    
    if returned_wallet.namespace == "empty".to_string() {
        returned_wallet.init = true;
        let new_namespace = format!("{}{}", "Nuggie".to_string(), 0);
        returned_wallet.namespace = new_namespace;
    } else {
        returned_wallet.init = true;
    }

    println!("This is Nuggie's first time calling try init, his wallet info is: {:#?} ", returned_wallet);

    //Simulate change owner by changing the file that is saved in BiPhan/ with the latest namespace. We have to declare another env because env doesn't implement the copy trait and it 
    //was already used above for forget_me
    let env = mock_env(String::from("BiPhan"), &[]);  
    let _changing_owner = try_change_owner(&mut deps, env, String::from("BiPhan/"), String::from("Khabib smash") );

    //Lets query the file and see if it's been changed

    let queried_file = query_file(&deps, String::from("BiPhan/"), &HumanAddr::from("Alice")).unwrap_or(String::from("file does not exist!"));
    println!("After calling change_owner, the queried file is {:#?}\n", queried_file);

}

pub fn try_forget_me<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let ha = deps
        .api
        .human_address(&deps.api.canonical_address(&env.message.sender)?)?;
    let adr = String::from(ha.as_str());

    let load_bucket: Result<WalletInfo, StdError> =
        bucket_read(WALLET_INFO_LOCATION, &deps.storage).load(adr.as_bytes());
    let mut wallet_info = load_bucket?;

    wallet_info.init = false;
    let new_counter = wallet_info.counter + 1;
    wallet_info.counter = new_counter; 
    let new_namespace = format!("{}{}", adr, new_counter);
    wallet_info.namespace = new_namespace;

    bucket(WALLET_INFO_LOCATION, &mut deps.storage)
        .save(ha.as_str().as_bytes(), &wallet_info)
        .map_err(|err| println!("{:?}", err))
        .ok();

    Ok(HandleResponse::default())
}