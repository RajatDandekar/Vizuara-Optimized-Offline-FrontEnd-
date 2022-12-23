/*Author: Htet Aung Hlaing
Modified Data: 22th Dec, 2022
<summary>
Description: 
This script will handle data related to chapters, classes, and their files

Error Handling: 
If an error is raised while reading the file, we would assume that something has gone wrong, and we would 
delete everything inside the data folder possibly, (a function to be implemented) 

1. Unparsable JSON
2. User Preference Exists but Struct Key does not exist (DONE)

Action -> Delete the entire folder and create new one, and then restart the application
</summary>*/

/*#region importing necessary libraries */
use crate::connection_manager;
use crate::file_manager;
extern crate lazy_static;

use serde_json::{Result, Value};
use std::sync::RwLock;

use lazy_static::lazy_static;
/*#endregion */

/*#region constant variable */
lazy_static!{
    static ref DATA_STRUCT_DATA: RwLock<Option<Value>> = RwLock::new( 
        None
    );
}  

/* 
    <summary>
    These variables will be used to check the data integrity
    basically by checking if those keys exist within the data struct variable
    </summary>
*/
const DATA_STRUCT_VARIABLE_TO_CHECK_1: &str = "O1xqt41b";

const DATA_STRUCT_VARIABLE_TO_CHECK_2: &str = "UaYp2sG1";

/*#endregion */

/*
<summary>
    download from the server and process it. And then run it
</summary>
*/
pub async fn save_data_struct_from_server() -> std::result::Result<(),()>{
    let get_data_struct_state = connection_manager::get_data_struct().await;
    
    if get_data_struct_state.is_ok() {
        let data_struct = get_data_struct_state.ok().unwrap();

        //println!("Data Struct File {}", data_struct);
        file_manager::create_file(file_manager::get_data_struct_keyfile_path(), data_struct);
        Ok(())
    }else{
        //do nothing and possibly reset the application
        println!("Something has occurred");
        Err(())
    }
}

pub async fn read_and_check_and_save_data_struct_file_integrity() -> std::result::Result<(),()>{
    if !file_manager::does_data_struct_keyfile_exists() {
        Err(())
    }else{
        //We will try to parse the data into the global variable
        //if failed Err()
        //If parsed successfully -> Ok()
        
        let saving_result = read_and_save_to_global_data_struct_variable().await;
        
        /*#region saved to variable */
        if saving_result.is_ok() {
            if data_integrity_is_valid(){
                Ok(())
            }else{
                Err(())
            }
        }
        /*#endregion */

        /*#region Saving Failed some type of error has occured */
        else{
            Err(())
        }
    }
}

fn data_integrity_is_valid() -> bool{
    let data_struct: &Option<Value> = &*DATA_STRUCT_DATA.read().unwrap();
    println!("Data integrity function triggered!");
    if data_struct.is_some(){
            check_if_key_exists_in_data_struct(DATA_STRUCT_VARIABLE_TO_CHECK_1.into()) && 
            check_if_key_exists_in_data_struct(DATA_STRUCT_VARIABLE_TO_CHECK_2.into())
    }
    else{
        false
    }
}

fn check_if_key_exists_in_data_struct(keyName: String) -> bool {
    let data_struct_option: &Option<Value> = &*DATA_STRUCT_DATA.read().unwrap();
    let data_struct = &data_struct_option.as_ref().unwrap();

    data_struct.get(keyName).is_some()
}

async fn read_data_struct_file() -> std::result::Result<Value, ()>{
    let read_file_result = file_manager::read_file(file_manager::get_data_struct_keyfile_path());

    let json_parse_result: Result<Value> = serde_json::from_str(read_file_result.as_str());
    if json_parse_result.is_ok(){
        Ok(json_parse_result.unwrap())
    }else{
        Err(())
    }
}

async fn read_and_save_to_global_data_struct_variable() -> std::result::Result<(),()>{

    println!("reading and saving");

    let read_data_struct_file_result = read_data_struct_file().await;

    if read_data_struct_file_result.is_ok(){
        save_to_global_data_struct_variable(read_data_struct_file_result.unwrap());
        Ok(())
    }else{
        println!("data struct error");
        Err(())
    }
}

fn save_to_global_data_struct_variable(data_struct_new_value : Value){

    let mut global_data_struct_pointer = DATA_STRUCT_DATA.write().unwrap();
    *global_data_struct_pointer = Some(data_struct_new_value);
}
